### Makefile for kvproto

CURDIR := $(shell pwd)

export PATH := $(CURDIR)/bin/:$(PATH)

all: go rust c++

init:
	mkdir -p $(CURDIR)/bin
go: init
	# Standalone GOPATH
	./generate_go.sh
	GO111MODULE=on go build ./pkg/...

rust: init
	# RUSTFLAGS will not work as it's a warnings from cargo shell.
	@cargo check --features regenerate 2>&1 | tee /dev/fd/2 | grep -qvz warning || (echo Please fix warnings; exit 1)

c++:
	./generate_cpp.sh
	rm -rf build_cpp && mkdir build_cpp && cd build_cpp && cmake ../cpp && make && cd .. && rm -rf build_cpp

.PHONY: all
