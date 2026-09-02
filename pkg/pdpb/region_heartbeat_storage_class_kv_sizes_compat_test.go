package pdpb

import (
	"testing"

	"github.com/golang/protobuf/proto"
)

type legacyRegionHeartbeatRequest struct {
	ApproximateKvSize uint64 `protobuf:"varint,18,opt,name=approximate_kv_size,json=approximateKvSize,proto3"`
}

func (m *legacyRegionHeartbeatRequest) Reset() {
	*m = legacyRegionHeartbeatRequest{}
}

func (m *legacyRegionHeartbeatRequest) String() string {
	return proto.CompactTextString(m)
}

func (*legacyRegionHeartbeatRequest) ProtoMessage() {}

func TestRegionHeartbeatApproximateStorageClassKVSizesWireCompatibility(t *testing.T) {
	data, err := proto.Marshal(&RegionHeartbeatRequest{
		ApproximateKvSize:         100,
		ApproximateIaKvSize:       40,
		ApproximateStandardKvSize: 60,
	})
	requireNoProtoError(t, err)
	roundTrippedRequest := &RegionHeartbeatRequest{}
	requireNoProtoError(t, proto.Unmarshal(data, roundTrippedRequest))
	if got := roundTrippedRequest.GetApproximateIaKvSize(); got != 40 {
		t.Fatalf("round-tripped IA approximate KV size = %d, want 40", got)
	}
	if got := roundTrippedRequest.GetApproximateStandardKvSize(); got != 60 {
		t.Fatalf("round-tripped Standard approximate KV size = %d, want 60", got)
	}

	legacyRequest := &legacyRegionHeartbeatRequest{}
	requireNoProtoError(t, proto.Unmarshal(data, legacyRequest))
	if got := legacyRequest.ApproximateKvSize; got != 100 {
		t.Fatalf("legacy approximate KV size = %d, want 100", got)
	}

	data, err = proto.Marshal(&legacyRegionHeartbeatRequest{ApproximateKvSize: 101})
	requireNoProtoError(t, err)

	newRequest := &RegionHeartbeatRequest{}
	requireNoProtoError(t, proto.Unmarshal(data, newRequest))
	if got := newRequest.GetApproximateKvSize(); got != 101 {
		t.Fatalf("new approximate KV size = %d, want 101", got)
	}
	if got := newRequest.GetApproximateIaKvSize(); got != 0 {
		t.Fatalf("IA approximate KV size from legacy request = %d, want 0", got)
	}
	if got := newRequest.GetApproximateStandardKvSize(); got != 0 {
		t.Fatalf("Standard approximate KV size from legacy request = %d, want 0", got)
	}
}
