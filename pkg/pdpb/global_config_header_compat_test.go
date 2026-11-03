package pdpb

import (
	"bytes"
	"testing"

	"github.com/golang/protobuf/proto"
)

type legacyWatchGlobalConfigRequest struct {
	ConfigPath string `protobuf:"bytes,1,opt,name=config_path,json=configPath,proto3"`
	Revision   int64  `protobuf:"varint,2,opt,name=revision,proto3"`
}

func (m *legacyWatchGlobalConfigRequest) Reset() { *m = legacyWatchGlobalConfigRequest{} }
func (m *legacyWatchGlobalConfigRequest) String() string {
	return proto.CompactTextString(m)
}
func (*legacyWatchGlobalConfigRequest) ProtoMessage() {}

type legacyStoreGlobalConfigRequest struct {
	Changes    []*GlobalConfigItem `protobuf:"bytes,1,rep,name=changes,proto3"`
	ConfigPath string              `protobuf:"bytes,2,opt,name=config_path,json=configPath,proto3"`
}

func (m *legacyStoreGlobalConfigRequest) Reset() { *m = legacyStoreGlobalConfigRequest{} }
func (m *legacyStoreGlobalConfigRequest) String() string {
	return proto.CompactTextString(m)
}
func (*legacyStoreGlobalConfigRequest) ProtoMessage() {}

type legacyLoadGlobalConfigRequest struct {
	Names      []string `protobuf:"bytes,1,rep,name=names,proto3"`
	ConfigPath string   `protobuf:"bytes,2,opt,name=config_path,json=configPath,proto3"`
}

func (m *legacyLoadGlobalConfigRequest) Reset() { *m = legacyLoadGlobalConfigRequest{} }
func (m *legacyLoadGlobalConfigRequest) String() string {
	return proto.CompactTextString(m)
}
func (*legacyLoadGlobalConfigRequest) ProtoMessage() {}

type globalConfigHeaderRequest interface {
	proto.Message
	GetHeader() *RequestHeader
}

func TestGlobalConfigRequestHeaderWireCompatibility(t *testing.T) {
	header := &RequestHeader{
		ClusterId:       42,
		SenderId:        43,
		CallerId:        "tidb",
		CallerComponent: "globalconfig",
	}
	changes := []*GlobalConfigItem{
		{Name: "config-a", Payload: []byte("value"), Kind: EventType_PUT},
		{Name: "config-b", Kind: EventType_DELETE},
	}
	tests := []struct {
		name    string
		legacy  proto.Message
		current globalConfigHeaderRequest
	}{
		{
			name:    "watch",
			legacy:  &legacyWatchGlobalConfigRequest{ConfigPath: "/global/config/", Revision: 100},
			current: &WatchGlobalConfigRequest{ConfigPath: "/global/config/", Revision: 100, Header: header},
		},
		{
			name:    "store",
			legacy:  &legacyStoreGlobalConfigRequest{Changes: changes, ConfigPath: "/global/config/"},
			current: &StoreGlobalConfigRequest{Changes: changes, ConfigPath: "/global/config/", Header: header},
		},
		{
			name:    "load",
			legacy:  &legacyLoadGlobalConfigRequest{Names: []string{"config-a", "config-b"}, ConfigPath: "/global/config/"},
			current: &LoadGlobalConfigRequest{Names: []string{"config-a", "config-b"}, ConfigPath: "/global/config/", Header: header},
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			legacyData, err := proto.Marshal(tt.legacy)
			requireNoProtoError(t, err)
			decoded := proto.Clone(tt.current).(globalConfigHeaderRequest)
			decoded.Reset()
			requireNoProtoError(t, proto.Unmarshal(legacyData, decoded))
			if decoded.GetHeader() != nil {
				t.Fatal("legacy request unexpectedly has a header")
			}
			data, err := proto.Marshal(decoded)
			requireNoProtoError(t, err)
			if !bytes.Equal(data, legacyData) {
				t.Fatalf("legacy wire format changed: got %x, want %x", data, legacyData)
			}

			data, err = proto.Marshal(tt.current)
			requireNoProtoError(t, err)
			decoded.Reset()
			requireNoProtoError(t, proto.Unmarshal(data, decoded))
			if !proto.Equal(decoded, tt.current) {
				t.Fatalf("request changed after round trip: got %v, want %v", decoded, tt.current)
			}

			legacy := proto.Clone(tt.legacy)
			legacy.Reset()
			requireNoProtoError(t, proto.Unmarshal(data, legacy))
			if !proto.Equal(legacy, tt.legacy) {
				t.Fatalf("legacy fields changed when decoding a request with a header: got %v, want %v", legacy, tt.legacy)
			}
		})
	}
}
