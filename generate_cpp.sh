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
pop

push cpp

echo "generate CMakeLists..."

CMAKE_FILE=CMakeLists.txt

cat <<EOF > ../${CMAKE_FILE}
add_library(kvproto
EOF

for file in `ls`
    do
    echo "    cpp/${file}" >> ../${CMAKE_FILE}
done

cat <<EOF >> ../${CMAKE_FILE}
)
target_include_directories (kvproto PUBLIC cpp)
EOF

