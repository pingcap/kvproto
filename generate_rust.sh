#!/bin/bash

. ./common.sh

if ! check_protoc_version; then
	exit 1
fi

# install rust-protobuf if it's missing
if ! cmd_exists protoc-gen-rust; then
    echo "missing rust-protobuf, try to download/install it"
    cargo install protobuf
fi

if ! cmd_exists grpc_rust_plugin; then
    echo "missing grpc_rust_plugin, please manually install it."
    exit 1
fi

push proto
echo "generate rust code..."

protoc -I.:./include --rust_out ../src *.proto || exit $?
protoc -I.:./include --grpc_out ../src --plugin=protoc-gen-grpc=`which grpc_rust_plugin` *.proto || exit $?
pop

push src
LIB_RS=`mktemp`
rm -f lib.rs
echo "extern crate protobuf;" > ${LIB_RS}
echo "extern crate futures;" >> ${LIB_RS}
echo "extern crate grpc;" >> ${LIB_RS}
echo "extern crate bytes;" >> ${LIB_RS}
echo >> ${LIB_RS}
for file in `ls *.rs`
    do
    base_name=$(basename $file ".rs")
    echo "pub mod $base_name;" >> ${LIB_RS}
done
mv ${LIB_RS} lib.rs
pop

cargo build
