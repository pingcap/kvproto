package kvrpcpb

import (
	"encoding/binary"
	"strings"
	"testing"

	"github.com/gogo/protobuf/proto"
	"github.com/pingcap/kvproto/internal/grpctest"
)

func nestedLockInfo(depth int) []byte {
	var data []byte
	for i := 0; i < depth; i++ {
		field := binary.AppendUvarint([]byte{0x62}, uint64(len(data)))
		data = append(field, data...)
	}
	return data
}

func TestLockInfoUnmarshalDepth(t *testing.T) {
	for _, tc := range []struct {
		name    string
		depth   int
		wantErr bool
	}{
		{"within limit", 999, false},
		{"over limit", 1000, true},
	} {
		t.Run(tc.name, func(t *testing.T) {
			lock := nestedLockInfo(tc.depth)
			data := binary.AppendUvarint([]byte{0x0a}, uint64(len(lock)))
			data = append(data, lock...)
			for _, decoder := range []struct {
				name      string
				unmarshal func([]byte, *KeyError) error
			}{
				{"gogo", func(data []byte, result *KeyError) error { return proto.Unmarshal(data, result) }},
				{"grpc", func(data []byte, result *KeyError) error {
					return grpctest.Unmarshal(t, data, result)
				}},
			} {
				t.Run(decoder.name, func(t *testing.T) {
					var result KeyError
					err := decoder.unmarshal(data, &result)
					if tc.wantErr {
						if err == nil || !strings.Contains(err.Error(), "maximum message nesting depth") {
							t.Fatalf("expected nesting error, got %v", err)
						}
					} else if err != nil {
						t.Fatalf("valid message rejected: %v", err)
					}
				})
			}
		})
	}
}
