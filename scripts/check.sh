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
        return 0
    else
        return 1
    fi
}

if ! check_protoc_version || ! check-protos-compatible; then
	exit 1
fi
