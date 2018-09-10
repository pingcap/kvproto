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
GOGO_ROOT=${GOPATH}/src/${gogo_protobuf_url}
GO_OUT_M=
GO_INSTALL='go install'

echo "install gogoproto code/generator ..."
${GO_INSTALL} ${gogo_protobuf_url}/proto
${GO_INSTALL} ${gogo_protobuf_url}/protoc-gen-gofast
${GO_INSTALL} ${gogo_protobuf_url}/gogoproto

echo "install goimports ..."
${GO_INSTALL} golang.org/x/tools/cmd/goimports

# add the bin path of gogoproto generator into PATH if it's missing
if ! cmd_exists protoc-gen-gofast; then
    for path in $(echo "${GOPATH}" | sed -e 's/:/ /g'); do
        gogo_proto_bin="${path}/bin/protoc-gen-gofast"
        if [ -e "${gogo_proto_bin}" ]; then
            export PATH=$(dirname "${gogo_proto_bin}"):$PATH
            break
        fi
    done
fi


push proto
CPP_OUT=../cpp_out
mkdir -p ${CPP_OUT}
protoc -I.:${GOGO_ROOT}:${GOGO_ROOT}/protobuf:../include --cpp_out ${CPP_OUT} ../include/eraftpb.proto || exit $?
protoc -I.:${GOGO_ROOT}:${GOGO_ROOT}/protobuf:../include --cpp_out ${CPP_OUT} *.proto || exit $?
protoc -I.:${GOGO_ROOT}:${GOGO_ROOT}/protobuf:../include --grpc_out ${CPP_OUT} --plugin=protoc-gen-grpc=`which grpc_cpp_plugin` *.proto || exit $?
pop

push cpp_out
sed -i.bak -E '/gogo\.pb\.h/d' *.cc
sed -i.bak -E '/gogo\.pb\.h/d' *.h
rm -f *.bak

sed -i.bak -E '/::protobuf_gogoproto_2fgogo_2eproto/d' *.cc
sed -i.bak -E '/::protobuf_gogoproto_2fgogo_2eproto/d' *.h
rm -f *.bak
pop

