GO_PREFIX_PATH=github.com/pingcap/kvproto/pkg

GO_OUT_M=

cd proto
for file in `ls *.proto`  
    do  
    base_name=$(basename $file ".proto")
    mkdir -p ../pkg/$base_name
    if [ -z $GO_OUT_M ]; then
        GO_OUT_M="M$file=$GO_PREFIX_PATH/$base_name"
    else
        GO_OUT_M="$GO_OUT_M,M$file=$GO_PREFIX_PATH/$base_name"
    fi
done

echo "generate go code..."
for file in `ls *.proto`  
    do
    base_name=$(basename $file ".proto")
    protoc --go_out=$GO_OUT_M:../pkg/$base_name $file
done