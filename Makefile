### Makefile for kvproto

CURDIR := $(shell pwd)

export PATH := $(CURDIR)/bin/:$(PATH)

all: go rust

init:
	mkdir -p $(CURDIR)/bin
go: init
	# Standalone GOPATH
	./generate_go.sh
	GO111MODULE=on go build ./pkg/...

rust: init
	cargo check --features regenerate

.PHONY: all
