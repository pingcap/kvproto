#!/usr/bin/env bash

. ./common.sh

if ! check_protoc_version; then
	exit 1
fi

echo "generate cpp code..."
KVPROTO_ROOT=`pwd`
push proto
GOGO_ROOT=${KVPROTO_ROOT}/vendor/github.com/gogo/protobuf
protoc -I.:${GOGO_ROOT}:../include --cpp_out ../cpp *.proto || exit $?

