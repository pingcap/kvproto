#!/usr/bin/env bash
docker run --rm -i -t -v "$(pwd):$(pwd)" -w "$(pwd)" tikv/kvproto:3.8.0 "$@"
