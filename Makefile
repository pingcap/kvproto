### Makefile for kvproto

CURDIR := $(shell pwd)

export PATH := $(CURDIR)/bin/:$(PATH)

all: go rust c++

init:
	mkdir -p $(CURDIR)/bin
check: init
	$(CURDIR)/scripts/check.sh
go: check
	# Standalone GOPATH
	$(CURDIR)/scripts/generate_go.sh
	GO111MODULE=on go mod tidy
	GO111MODULE=on go build ./pkg/...

rust: init
	cargo check && \
	cargo check --no-default-features --features prost-codec

c++: check
	mkdir -p kvprotobuild && cd kvprotobuild && cmake ../cpp -DCMAKE_PREFIX_PATH=$$GRPC_INSTALL_PATH && make

.PHONY: all
