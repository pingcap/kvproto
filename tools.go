//go:build tools
// +build tools

package tools

import (
	_ "github.com/gogo/protobuf/protoc-gen-gofast"
	_ "github.com/grpc-ecosystem/grpc-gateway/protoc-gen-grpc-gateway"
	_ "golang.org/x/tools/cmd/goimports"
)
