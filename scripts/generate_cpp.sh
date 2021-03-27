#!/usr/bin/env bash
set -euo pipefail

SHELL_DEBUG=${SHELL_DEBUG:-""}
if [[ -n "$SHELL_DEBUG" ]] ; then
  set -x
fi

SCRIPTS_DIR=$(dirname "$0")
source $SCRIPTS_DIR/common.sh

echo "generate cpp code..."

KVPROTO_ROOT="$SCRIPTS_DIR/.."
GRPC_INCLUDE=.:../include

cd $KVPROTO_ROOT
rm -rf proto-cpp && mkdir -p proto-cpp
rm -rf cpp/kvproto && mkdir cpp/kvproto

cp proto/* proto-cpp/

sed_inplace '/gogo.proto/d' proto-cpp/*
sed_inplace '/option\ *(gogoproto/d' proto-cpp/*
sed_inplace -e 's/\[.*gogoproto.*\]//g' proto-cpp/*

pushd proto-cpp >/dev/null
protoc -I${GRPC_INCLUDE} --cpp_out ../cpp/kvproto *.proto
protoc -I${GRPC_INCLUDE} --grpc_out ../cpp/kvproto --plugin=protoc-gen-grpc=`which grpc_cpp_plugin` *.proto
popd >/dev/null

pushd include >/dev/null
protoc -I${GRPC_INCLUDE} --cpp_out ../cpp/kvproto *.proto google/api/http.proto google/api/annotations.proto
popd >/dev/null

rm -rf proto-cpp
