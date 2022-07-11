#!/usr/bin/env bash

extra_arg=""
if [ "$(uname -p)" == "arm" ]; then
	extra_arg="--platform linux/x86_64"
fi

docker build $extra_arg . -t tikv/kvproto:3.8.0
