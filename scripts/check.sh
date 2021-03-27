#!/usr/bin/env bash
set -euo pipefail

if ! version=$(protoc --version) ; then
	echo ""
	echo "    could not find protoc"
	exit 1
fi

major=$(echo ${version} | sed -n -e 's/.*\([0-9]\{1,\}\)\.[0-9]\{1,\}\.[0-9]\{1,\}.*/\1/p')
minor=$(echo ${version} | sed -n -e 's/.*[0-9]\{1,\}\.\([0-9]\{1,\}\)\.[0-9]\{1,\}.*/\1/p')
if ! [ "$major" -eq 3 ] || ! [ "$minor" -eq 8 ]; then
	echo "protoc version not match, version 3.8.x is needed, current version: ${version}"
	exit 1
fi
