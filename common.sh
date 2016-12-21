#!/usr/bin/env bash

check_protoc_version() {
    ver=$(protoc --version | awk '{print $2}')
    if [ "$ver" != "3.1.0" ]; then
        echo "protoc version not match, version 3.1.0 is needed"
        exit 1
    fi
}

