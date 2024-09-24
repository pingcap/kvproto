#!/usr/bin/env bash

check_protoc_version() {
    version=$(protoc --version | awk '{print $NF}')
    major=$(echo ${version} | cut -d '.' -f 1)
    minor=$(echo ${version} | cut -d '.' -f 2)
    if [ "$major" -eq 3 ] && [ "$minor" -ge 8 ]; then
        return 0
    fi
    # protobuf bumps the major version to 21 after 3.
    # https://github.com/protocolbuffers/protobuf/releases/tag/v21.7
    if [ "$major" -ge 21 ]; then
        return 0
    fi
    echo "protoc version not match, version 3.8.x+ is needed, current version: ${version}"
    return 1
}

check-protos-compatible() {
    GOPATH=$(go env GOPATH)
    if [ -z $GOPATH ]; then
        printf "Error: the environment variable GOPATH is not set, please set it before running %s\n" $PROGRAM > /dev/stderr
        exit 1
    fi
    export PATH=$GOPATH/bin:$PATH

    if [ ! -f "$GOPATH/bin/protolock" ]; then
        go install github.com/nilslice/protolock/cmd/protolock@v0.17.0 || exit 1
	fi

    if protolock status -lockdir=scripts -protoroot=proto; then
        protolock commit -lockdir=scripts -protoroot=proto
    else
        echo "Meet break compatibility problem, please check the code."
        # In order not to block local branch development, when meet break compatibility will force to update `proto.lock`.
        protolock commit --force -lockdir=scripts -protoroot=proto
    fi
    # git report error like "fatal: detected dubious ownership in repository at" when reading the host's git folder
    git config --global --add safe.directory $(pwd)
    # If the output message is encountered, please add proto.lock to git as well.
    git diff scripts/proto.lock | cat
    git diff --quiet scripts/proto.lock
    if [ $? -ne 0 ]; then
        echo "Please add proto.lock to git."
        return 1
    fi
    return 0
}

if ! check_protoc_version || ! check-protos-compatible; then
	exit 1
fi
