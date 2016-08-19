GO_PREFIX_PATH=github.com/pingcap/kvproto/pkg

GOGO_ROOT=${GOPATH}/src/github.com/gogo/protobuf
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
ret=0
for file in `ls *.proto`  
    do
    base_name=$(basename $file ".proto")
    protoc -I.:${GOGO_ROOT}:${GOGO_ROOT}/protobuf --gofast_out=$GO_OUT_M:../pkg/$base_name $file || ret=$?
    cd ../pkg/$base_name
    sed -i.bak -E 's/import _ \"gogoproto\"//g' *.pb.go
    sed -i.bak -E 's/import fmt \"fmt\"//g' *.pb.go
    sed -i.bak -E 's/import io \"io\"//g' *.pb.go
    sed -i.bak -E 's/import math \"math\"//g' *.pb.go
    rm -f *.bak
    goimports -w *.pb.go
    cd ../../proto
done
exit $ret
