#!/usr/bin/env bash

. ./common.sh

if ! check_protoc_version; then
	exit 1
fi

PROGRAM=$(basename "$0")

if [ -z $GOPATH ]; then
    printf "Error: the environment variable GOPATH is not set, please set it before running %s\n" $PROGRAM > /dev/stderr
    exit 1
fi

GO_PREFIX_PATH=github.com/pingcap/kvproto/pkg

gogo_protobuf_url=github.com/gogo/protobuf
CURRENT_DIR=$(pwd)
GOGO_ROOT=${CURRENT_DIR}/vendor/${gogo_protobuf_url}
GO_OUT_M=
GO_INSTALL='go install'

echo "install gogoproto code/generator ..."
cd ${CURRENT_DIR}/vendor/${gogo_protobuf_url}/protoc-gen-gofast && go build -o ${CURRENT_DIR}/bin/protoc-gen-gofast
echo "install goimports ..."
cd ${CURRENT_DIR}/vendor/golang.org/x/tools/cmd/goimports && go build -o ${CURRENT_DIR}/bin/goimports
cd ${CURRENT_DIR}

# add the bin path of gogoproto generator into PATH if it's missing
if ! cmd_exists protoc-gen-gofast; then
	export PATH=${CURRENT_DIR}/bin:$PATH
fi

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
cd proto
for file in `ls *.proto`
    do
    collect $file
done

echo "generate go code..."
ret=0

function gen() {
    base_name=$(basename $1 ".proto")
    protoc -I.:${GOGO_ROOT}:../include --gofast_out=plugins=grpc,$GO_OUT_M:../pkg/$base_name $1 || ret=$?
    cd ../pkg/$base_name
    sed -i.bak -E 's/import _ \"gogoproto\"//g' *.pb.go
    sed -i.bak -E 's/import fmt \"fmt\"//g' *.pb.go
    sed -i.bak -E 's/import io \"io\"//g' *.pb.go
    sed -i.bak -E 's/import math \"math\"//g' *.pb.go
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
