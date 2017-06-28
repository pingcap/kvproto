### Makefile for kvproto

CURDIR := $(shell pwd)

KEEP_FILE := '**/gogo.proto,**/google/protobuf/descriptor.proto'

export PATH := $(CURDIR)/_vendor/bin:$(PATH)

all: go rust test

test:
	# Build test.
	go build ./pkg/...
	cargo check

go:
	rm -f _vendor/src
	ln -s ./vendor _vendor/src
	# Standalone GOPATH
	GOPATH=$(CURDIR)/_vendor ./generate_go.sh

rust:
	./generate_rust.sh

update_include:
	mkdir -p proto/include/gogoproto
	cp _vendor/vendor/github.com/gogo/protobuf/gogoproto/gogo.proto proto/include/gogoproto/
	mkdir -p proto/include/google/protobuf
	cp -r _vendor/vendor/github.com/gogo/protobuf/protobuf/google/protobuf/descriptor.proto proto/include/google/protobuf

update_go_pkg:
	which glide >/dev/null || curl https://glide.sh/get | sh
	which glide-vc || go get -v -u github.com/sgotti/glide-vc
	rm -rf vendor && mv _vendor vendor || true
	rm -rf _vendor
ifdef PKG
	glide get --strip-vendor --skip-test ${PKG}
else
	glide update --strip-vendor --skip-test
endif
	@echo "removing test files"
	glide vc --only-code --no-tests --use-lock-file --keep $(KEEP_FILE)
	mkdir -p _vendor
	mv vendor _vendor

.PHONY: update_go_pkg update_include all
