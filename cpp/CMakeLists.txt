project (kvproto)

cmake_minimum_required (VERSION 2.8)

set (CMAKE_CXX_STANDARD 17)

if (CMAKE_VERSION VERSION_LESS "3.8.0")
    set (CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++1z")
else ()
    set (CMAKE_CXX_STANDARD 17)
    set (CMAKE_CXX_EXTENSIONS 0) # https://cmake.org/cmake/help/latest/prop_tgt/CXX_EXTENSIONS.html#prop_tgt:CXX_EXTENSIONS
    set (CMAKE_CXX_STANDARD_REQUIRED ON)
    set (CXX_FLAGS_INTERNAL_COMPILER "-std=c++1z")
    # This needs to propagate to vendored projects in contrib
    set (CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++17")
endif ()

file(GLOB __kvproto_sources RELATIVE ${CMAKE_CURRENT_SOURCE_DIR} kvproto/*.cc)
file(GLOB __kvproto_headers RELATIVE ${CMAKE_CURRENT_SOURCE_DIR} kvproto/*.h)

add_library(kvproto ${__kvproto_sources} ${__kvproto_headers})
target_include_directories(kvproto PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_include_directories(kvproto PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/kvproto)

if (NOT PROTOBUF_INCLUDE_DIRS)
    find_package(Protobuf REQUIRED)
endif()
target_include_directories(kvproto PUBLIC ${PROTOBUF_INCLUDE_DIRS})