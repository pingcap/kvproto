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
	./generate_rust.sh
	cargo check

c++:
	./generate_cpp.sh
	rm -rf build_cpp && mkdir build_cpp && cd build_cpp && cmake ../cpp && make && cd .. && rm -rf build_cpp

.PHONY: all