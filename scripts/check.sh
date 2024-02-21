#!/usr/bin/env bash

check_protoc_version() {
    version=$(protoc --version)
    major=$(echo ${version} | sed -n -e 's/.*\([0-9]\{1,\}\)\.[0-9]\{1,\}\.[0-9]\{1,\}.*/\1/p')
    minor=$(echo ${version} | sed -n -e 's/.*[0-9]\{1,\}\.\([0-9]\{1,\}\)\.[0-9]\{1,\}.*/\1/p')
    if [ "$major" -eq 3 ] && [ "$minor" -ge 8 ]; then
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
        GO111MODULE=off go get github.com/nilslice/protolock/cmd/protolock
        GO111MODULE=off go install github.com/nilslice/protolock/cmd/protolock
	fi

    if protolock status -lockdir=scripts -protoroot=proto; then
        protolock commit -lockdir=scripts -protoroot=proto
    else
        echo "Meet break compatibility problem, please check the code."
        # In order not to block local branch development, when meet break compatibility will force to update `proto.lock`.
        protolock commit --force -lockdir=scripts -protoroot=proto
    fi
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
