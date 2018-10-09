### Makefile for kvproto

CURDIR := $(shell pwd)

KEEP_FILE := '**/*.proto,**/*.sh'

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

update_go_pkg:
	dep ensure

.PHONY: update_go_pkg all
