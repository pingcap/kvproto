package pdpb

import (
	"testing"

	"github.com/golang/protobuf/proto"
)

type legacyGetGCStateRequest struct {
	KeyspaceScope     *KeyspaceScope `protobuf:"bytes,2,opt,name=keyspace_scope,json=keyspaceScope,proto3"`
	ExcludeGcBarriers bool           `protobuf:"varint,3,opt,name=exclude_gc_barriers,json=excludeGcBarriers,proto3"`
}

func (m *legacyGetGCStateRequest) Reset() {
	*m = legacyGetGCStateRequest{}
}

func (m *legacyGetGCStateRequest) String() string {
	return proto.CompactTextString(m)
}

func (*legacyGetGCStateRequest) ProtoMessage() {}

type legacyGetGCStateResponse struct {
	GcState *GCState `protobuf:"bytes,2,opt,name=gc_state,json=gcState,proto3"`
}

func (m *legacyGetGCStateResponse) Reset() {
	*m = legacyGetGCStateResponse{}
}

func (m *legacyGetGCStateResponse) String() string {
	return proto.CompactTextString(m)
}

func (*legacyGetGCStateResponse) ProtoMessage() {}

func requireNoProtoError(t *testing.T, err error) {
	t.Helper()
	if err != nil {
		t.Fatal(err)
	}
}

func TestGetGCStateGlobalBarriersWireCompatibility(t *testing.T) {
	newRequest := &GetGCStateRequest{
		KeyspaceScope: &KeyspaceScope{
			Keyspace: &KeyspaceScope_KeyspaceId{KeyspaceId: 42},
		},
		ExcludeGcBarriers:       true,
		IncludeGlobalGcBarriers: true,
	}
	data, err := proto.Marshal(newRequest)
	requireNoProtoError(t, err)

	oldRequest := &legacyGetGCStateRequest{}
	requireNoProtoError(t, proto.Unmarshal(data, oldRequest))
	if got := oldRequest.KeyspaceScope.GetKeyspaceId(); got != 42 {
		t.Fatalf("legacy keyspace ID = %d, want 42", got)
	}
	if !oldRequest.ExcludeGcBarriers {
		t.Fatal("legacy request lost exclude_gc_barriers")
	}

	data, err = proto.Marshal(&legacyGetGCStateRequest{
		KeyspaceScope: &KeyspaceScope{
			Keyspace: &KeyspaceScope_KeyspaceId{KeyspaceId: 43},
		},
		ExcludeGcBarriers: true,
	})
	requireNoProtoError(t, err)

	decodedNewRequest := &GetGCStateRequest{}
	requireNoProtoError(t, proto.Unmarshal(data, decodedNewRequest))
	if got := decodedNewRequest.GetKeyspaceScope().GetKeyspaceId(); got != 43 {
		t.Fatalf("new keyspace ID = %d, want 43", got)
	}
	if !decodedNewRequest.GetExcludeGcBarriers() {
		t.Fatal("new request lost exclude_gc_barriers")
	}
	if decodedNewRequest.GetIncludeGlobalGcBarriers() {
		t.Fatal("old request unexpectedly enabled global GC barriers")
	}

	newResponse := &GetGCStateResponse{
		GcState:          &GCState{GcSafePoint: 100},
		GlobalGcBarriers: &GlobalGCBarriersInfo{},
	}
	data, err = proto.Marshal(newResponse)
	requireNoProtoError(t, err)

	oldResponse := &legacyGetGCStateResponse{}
	requireNoProtoError(t, proto.Unmarshal(data, oldResponse))
	if got := oldResponse.GcState.GetGcSafePoint(); got != 100 {
		t.Fatalf("legacy GC safe point = %d, want 100", got)
	}

	data, err = proto.Marshal(&legacyGetGCStateResponse{
		GcState: &GCState{GcSafePoint: 101},
	})
	requireNoProtoError(t, err)

	decodedNewResponse := &GetGCStateResponse{}
	requireNoProtoError(t, proto.Unmarshal(data, decodedNewResponse))
	if got := decodedNewResponse.GetGcState().GetGcSafePoint(); got != 101 {
		t.Fatalf("new GC safe point = %d, want 101", got)
	}
	if decodedNewResponse.GetGlobalGcBarriers() != nil {
		t.Fatal("old response unexpectedly has global GC barriers")
	}

	data, err = proto.Marshal(newResponse)
	requireNoProtoError(t, err)
	roundTripped := &GetGCStateResponse{}
	requireNoProtoError(t, proto.Unmarshal(data, roundTripped))
	if roundTripped.GetGlobalGcBarriers() == nil {
		t.Fatal("present-empty global GC barriers wrapper was lost")
	}
	if got := len(roundTripped.GetGlobalGcBarriers().GetBarriers()); got != 0 {
		t.Fatalf("global GC barrier count = %d, want 0", got)
	}

	newResponse.GlobalGcBarriers.Barriers = []*GlobalGCBarrierInfo{
		{
			BarrierId:  "backup",
			BarrierTs:  102,
			TtlSeconds: 60,
		},
	}
	data, err = proto.Marshal(newResponse)
	requireNoProtoError(t, err)
	roundTripped = &GetGCStateResponse{}
	requireNoProtoError(t, proto.Unmarshal(data, roundTripped))
	barriers := roundTripped.GetGlobalGcBarriers().GetBarriers()
	if len(barriers) != 1 || barriers[0].GetBarrierId() != "backup" {
		t.Fatalf("global GC barriers = %+v, want backup", barriers)
	}
}
