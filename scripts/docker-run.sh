#!/usr/bin/env bash

extra_arg=""
if [ "$(uname -p)" == "arm" ]; then
	extra_arg="--platform linux/x86_64"
fi

docker run $extra_arg --rm -i -t -v "$(pwd):$(pwd)" -w "$(pwd)" tikv/kvproto:3.8.0 "$@"

