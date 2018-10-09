### Makefile for kvproto

CURDIR := $(shell pwd)

KEEP_FILE := '**/*.proto,**/*.sh'

all: go rust

go:
	# Standalone GOPATH
	./generate_go.sh
	go build ./pkg/...

rust:
	./generate_rust.sh
	cargo check

update_go_pkg:
	dep ensure

.PHONY: update_go_pkg all
