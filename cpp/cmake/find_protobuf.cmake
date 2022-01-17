find_package(Protobuf REQUIRED)
message(STATUS "Using protobuf: ${Protobuf_VERSION} : ${Protobuf_INCLUDE_DIR}, ${Protobuf_LIBRARY}, ${Protobuf_PROTOC_EXECUTABLE}")
