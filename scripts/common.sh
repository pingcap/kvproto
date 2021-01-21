#!/usr/bin/env bash

function sed_inplace()
{
	if [ `uname` == "Darwin" ]; then
		sed -i '' "$@"
	else
		sed -i "$@"
	fi
}
