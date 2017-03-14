#!/bin/bash

. ./common.sh

check_protoc_version()

# install rust-protobuf if it's missing
if ! cmd_exists protoc-gen-rust; then
    echo "missing rust-protobuf, try to download/install it"
    cargo install protobuf
fi

if ! cmd_exists protoc-gen-rust-grpc; then
    echo "missing rust-protobuf-grpc, please manually install it."
    exit 1
fi

push proto
echo "generate rust code..."
gogo_protobuf_url=github.com/gogo/protobuf
GOGO_ROOT=${GOPATH}/src/github.com/gogo/protobuf
GO_INSTALL='go install'

echo "install gogoproto code/generator ..."
${GO_INSTALL} ${gogo_protobuf_url}/proto
${GO_INSTALL} ${gogo_protobuf_url}/protoc-gen-gofast
${GO_INSTALL} ${gogo_protobuf_url}/gogoproto

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

protoc -I.:${GOGO_ROOT}:${GOGO_ROOT}/protobuf --rust_out ../src *.proto || exit $?
protoc -I.:${GOGO_ROOT}:${GOGO_ROOT}/protobuf --rust-grpc_out ../src *.proto || exit $?
pop

push src
LIB_RS=`mktemp`
rm -f lib.rs
echo "extern crate protobuf;" > ${LIB_RS}
echo "extern crate grpc;" >> ${LIB_RS}
echo "extern crate futures;" >> ${LIB_RS}
echo "extern crate futures_cpupool;" >> ${LIB_RS}
echo >> ${LIB_RS}
for file in `ls *.rs`
    do
    base_name=$(basename $file ".rs")
    echo "pub mod $base_name;" >> ${LIB_RS}
done
mv ${LIB_RS} lib.rs
pop

cargo build
