package db9_coprocessor

import (
	"encoding/binary"
	"strings"
	"testing"

	"github.com/gogo/protobuf/proto"
	"google.golang.org/grpc/encoding"
	_ "google.golang.org/grpc/encoding/proto"
)

func messageField(tag byte, value []byte) []byte {
	data := binary.AppendUvarint([]byte{tag}, uint64(len(value)))
	return append(data, value...)
}

func nestedType(depth int) []byte {
	var data []byte
	for i := 0; i < depth; i++ {
		data = messageField(0x12, data)
	}
	return data
}

func nestedExpr(depth int, exprTag, childTag byte) []byte {
	var data []byte
	for i := 0; i < depth; i++ {
		data = messageField(exprTag, messageField(childTag, data))
	}
	return data
}

func checkUnmarshalDepth(t *testing.T, data []byte, wantErr bool) {
	t.Helper()
	for _, decoder := range []struct {
		name      string
		unmarshal func([]byte, *Db9DagRequest) error
	}{
		{"gogo", func(data []byte, result *Db9DagRequest) error { return proto.Unmarshal(data, result) }},
		{"grpc", func(data []byte, result *Db9DagRequest) error {
			return encoding.GetCodec("proto").Unmarshal(data, result)
		}},
	} {
		t.Run(decoder.name, func(t *testing.T) {
			var result Db9DagRequest
			err := decoder.unmarshal(data, &result)
			if wantErr {
				if err == nil || !strings.Contains(err.Error(), "maximum message nesting depth") {
					t.Fatalf("expected nesting error, got %v", err)
				}
			} else if err != nil {
				t.Fatalf("valid message rejected: %v", err)
			}
		})
	}
}

func TestExprUnmarshalDepth(t *testing.T) {
	for _, tc := range []struct {
		name     string
		exprTag  byte
		childTag byte
	}{
		{"binary left", 0x22, 0x0a},
		{"binary right", 0x22, 0x1a},
		{"unary", 0x2a, 0x12},
		{"cast", 0x32, 0x0a},
		{"function", 0x3a, 0x12},
		{"is null", 0x42, 0x0a},
	} {
		t.Run(tc.name, func(t *testing.T) {
			checkUnmarshalDepth(t, messageField(0x2a, nestedExpr(499, tc.exprTag, tc.childTag)), false)
			checkUnmarshalDepth(t, messageField(0x2a, nestedExpr(500, tc.exprTag, tc.childTag)), true)
		})
	}
}

func TestTypeUnmarshalDepth(t *testing.T) {
	for _, tc := range []struct {
		name    string
		depth   int
		wantErr bool
	}{
		{"within limit", 998, false},
		{"over limit", 999, true},
	} {
		t.Run(tc.name, func(t *testing.T) {
			expr := messageField(0x0a, nestedType(tc.depth))
			checkUnmarshalDepth(t, messageField(0x2a, expr), tc.wantErr)
		})
	}
}

func TestMixedExprAndTypeUnmarshalDepth(t *testing.T) {
	expr := messageField(0x0a, nestedType(250))
	for i := 0; i < 400; i++ {
		expr = messageField(0x2a, messageField(0x12, expr))
	}
	checkUnmarshalDepth(t, messageField(0x2a, expr), true)

	cast := messageField(0x32, messageField(0x12, nestedType(999)))
	checkUnmarshalDepth(t, messageField(0x2a, cast), true)
}
