### Makefile for kvproto

CURDIR := $(shell pwd)

export PATH := $(CURDIR)/bin/:$(PATH)

all: go rust

init:
	mkdir -p $(CURDIR)/bin
go: init
	# Standalone GOPATH
	./generate_go.sh
	go build ./pkg/...

rust: init
	./generate_rust.sh
	cargo check

update-go-pkg:
	dep ensure

.PHONY: update-go-pkg all
