#!/usr/bin/env bash

check_protoc_version() {
    ver=$(protoc --version | awk '{print $2}')
    major=$(echo $ver | awk -F"[.]" '{print $1}')
    minor=$(echo $ver | awk -F"[.]" '{print $2}')
    if [ "$major" != "3" ] || [ "$minor" != "1" ]; then
        echo "protoc version not match, version 3.1.x is needed"
        exit 1
    fi
}

