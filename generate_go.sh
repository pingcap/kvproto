#!/usr/bin/env bash

set -ex

check_protoc_version() {
    version=$(protoc --version)
    major=$(echo ${version} | sed -n -e 's/.*\([0-9]\{1,\}\)\.[0-9]\{1,\}\.[0-9]\{1,\}.*/\1/p')
    minor=$(echo ${version} | sed -n -e 's/.*[0-9]\{1,\}\.\([0-9]\{1,\}\)\.[0-9]\{1,\}.*/\1/p')
    if [ "$major" -gt 3 ]; then
        return 0
    fi
    if [ "$major" -eq 3 ] && [ "$minor" -ge 1 ]; then
        return 0
    fi
    echo "protoc version not match, version 3.1.x is needed, current version: ${version}"
    return 1
}

push () {
    pushd $1 >/dev/null 2>&1
}

pop () {
    popd $1 >/dev/null 2>&1
}

cmd_exists () {
    which "$1" 1>/dev/null 2>&1
}

if ! check_protoc_version; then
	exit 1
fi

PROGRAM=$(basename "$0")

if [ -z $GOPATH ]; then
    printf "Error: the environment variable GOPATH is not set, please set it before running %s\n" $PROGRAM > /dev/stderr
    exit 1
fi

GO_PREFIX_PATH=github.com/pingcap/kvproto/pkg
export PATH=$(pwd)/_tools/bin:$GOPATH/bin:$PATH

echo "install tools..."
GO111MODULE=off go get github.com/twitchtv/retool
GO111MODULE=off retool sync || exit 1

function collect() {
    file=$(basename $1)
    base_name=$(basename $file ".proto")
    mkdir -p ../pkg/$base_name
    if [ -z $GO_OUT_M ]; then
        GO_OUT_M="M$file=$GO_PREFIX_PATH/$base_name"
    else
        GO_OUT_M="$GO_OUT_M,M$file=$GO_PREFIX_PATH/$base_name"
    fi
}

# Although eraftpb.proto is copying from raft-rs, however there is no
# official go code ship with the crate, so we need to generate it manually.
collect include/eraftpb.proto
collect include/rustproto.proto
cd proto
for file in `ls *.proto`
    do
    collect $file
done

echo "generate go code..."
ret=0

function gen() {
    base_name=$(basename $1 ".proto")
    protoc -I.:../include --gofast_out=plugins=grpc,$GO_OUT_M:../pkg/$base_name $1 || ret=$?
    cd ../pkg/$base_name
    sed -i.bak -E 's/import _ \"gogoproto\"//g' *.pb.go
    sed -i.bak -E 's/import fmt \"fmt\"//g' *.pb.go
    sed -i.bak -E 's/import io \"io\"//g' *.pb.go
    sed -i.bak -E 's/import math \"math\"//g' *.pb.go
    sed -i.bak -E 's/import _ \".*rustproto\"//' *.pb.go
    rm -f *.bak
    goimports -w *.pb.go
    cd ../../proto
}

gen ../include/eraftpb.proto
for file in `ls *.proto`
    do
    gen $file
done
exit $ret
