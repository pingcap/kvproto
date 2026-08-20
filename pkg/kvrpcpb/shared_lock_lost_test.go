package kvrpcpb

import (
	"bytes"
	"testing"

	"github.com/gogo/protobuf/proto"
)

func TestSharedLockLostWireContract(t *testing.T) {
	original := &KeyError{
		SharedLockLost: &SharedLockLost{
			Key:     []byte("key"),
			StartTs: 42,
		},
	}

	encoded, err := proto.Marshal(original)
	if err != nil {
		t.Fatal(err)
	}
	const sharedLockLostTag = byte(14<<3 | 2)
	if len(encoded) == 0 || encoded[0] != sharedLockLostTag {
		t.Fatalf("unexpected outer wire tag: %x", encoded)
	}

	var decoded KeyError
	if err := proto.Unmarshal(encoded, &decoded); err != nil {
		t.Fatal(err)
	}
	if decoded.SharedLockLost == nil {
		t.Fatalf("shared_lock_lost was not decoded: %s", decoded.String())
	}
	if !bytes.Equal(decoded.SharedLockLost.Key, original.SharedLockLost.Key) {
		t.Fatalf("unexpected key: %q", decoded.SharedLockLost.Key)
	}
	if decoded.SharedLockLost.StartTs != original.SharedLockLost.StartTs {
		t.Fatalf("unexpected start_ts: %d", decoded.SharedLockLost.StartTs)
	}
}
