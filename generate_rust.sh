cd proto

echo "generate rust code..."
ret=0
protoc --rust_out ../src *.proto || ret=$?

echo "extern crate protobuf;" > ../src/lib.rs
for file in `ls *.proto`
    do
    base_name=$(basename $file ".proto")
    echo "pub mod $base_name;" >> ../src/lib.rs
done
exit $ret
