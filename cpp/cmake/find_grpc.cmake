# find_package (OpenSSL)
# message (STATUS "Using ssl=${OPENSSL_FOUND}: ${OPENSSL_INCLUDE_DIR} : ${OPENSSL_LIBRARIES}")
# 
# find_package(Protobuf REQUIRED)
# message(STATUS "Using protobuf: ${Protobuf_VERSION} : ${Protobuf_INCLUDE_DIR}, ${Protobuf_LIBRARY}")
# 
# include_directories(${PROTOBUF_INCLUDE_DIRS})
# 
# find_package(c-ares REQUIRED)
# message(STATUS "Lib c-ares found")
# 
# find_package(ZLIB REQUIRED)
# message(STATUS "Using ZLIB: ${ZLIB_INCLUDE_DIRS}, ${ZLIB_LIBRARIES}")

find_package(gRPC REQUIRED)
message(STATUS "Using gRPC: ${gRPC_VERSION} : ${gRPC_INCLUDE_DIRS}, ${gRPC_LIBRARIES}")
