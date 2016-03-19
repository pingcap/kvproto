cd proto

echo "generate rust code..."
protoc --rust_out ../src *.proto

echo "extern crate protobuf;" > ../src/lib.rs
for file in `ls *.proto`
    do
    base_name=$(basename $file ".proto")
    echo "#[cfg_attr(rustfmt, rustfmt_skip)]" >> ../src/lib.rs
    echo "pub mod $base_name;" >> ../src/lib.rs
done 