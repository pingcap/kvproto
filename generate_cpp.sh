#!/bin/bash

set -ex

check_protoc_version() {
    version=$(protoc --version)
    major=$(echo ${version} | sed -n -e 's/.*\([0-9]\{1,\}\)\.[0-9]\{1,\}\.[0-9]\{1,\}.*/\1/p')
    minor=$(echo ${version} | sed -n -e 's/.*[0-9]\{1,\}\.\([0-9]\{1,\}\)\.[0-9]\{1,\}.*/\1/p')
    if [ "$major" -eq 3 ] && [ "$minor" -eq 8 ]; then
        return 0
    fi
    echo "protoc version not match, version 3.8.x is needed, current version: ${version}"
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

echo "generate cpp code..."

KVPROTO_ROOT=`pwd`
GOGO_ROOT=${KVPROTO_ROOT}/_vendor/src/github.com/gogo/protobuf
GRPC_INCLUDE=.:${GOGO_ROOT}:${GOGO_ROOT}/protobuf:../include

rm -rf proto-cpp && mkdir -p proto-cpp
rm -rf cpp/kvproto && mkdir cpp/kvproto

cp proto/* proto-cpp/

function sed_inplace()
{
	if [ `uname` == "Darwin" ]; then
		sed -i '' "$@"
	else
		sed -i "$@"
	fi
}

sed_inplace '/gogo.proto/d' proto-cpp/*
sed_inplace '/option\ (gogoproto/d' proto-cpp/*
sed_inplace -e 's/\[.*gogoproto.*\]//g' proto-cpp/*

push proto-cpp
protoc -I${GRPC_INCLUDE} --cpp_out ../cpp/kvproto *.proto || exit $?
protoc -I${GRPC_INCLUDE} --grpc_out ../cpp/kvproto --plugin=protoc-gen-grpc=`which grpc_cpp_plugin` *.proto || exit $?
pop

push include
protoc -I${GRPC_INCLUDE} --cpp_out ../cpp/kvproto *.proto || exit $?
pop

rm -rf proto-cpp

