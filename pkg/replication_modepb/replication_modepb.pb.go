// Code generated by protoc-gen-gogo. DO NOT EDIT.
// source: replication_modepb.proto

package replication_modepb

import (
	fmt "fmt"
	io "io"
	math "math"
	math_bits "math/bits"

	proto "github.com/golang/protobuf/proto"
)

// Reference imports to suppress errors if they are not otherwise used.
var _ = proto.Marshal
var _ = fmt.Errorf
var _ = math.Inf

// This is a compile-time assertion to ensure that this generated file
// is compatible with the proto package it is being compiled against.
// A compilation error at this line likely means your copy of the
// proto package needs to be updated.
const _ = proto.ProtoPackageIsVersion3 // please upgrade the proto package

type ReplicationMode int32

const (
	// The standard mode. Replicate logs to majority peer.
	ReplicationMode_MAJORITY ReplicationMode = 0
	// DR mode. Replicate logs among 2 DCs.
	ReplicationMode_DR_AUTO_SYNC ReplicationMode = 1
)

var ReplicationMode_name = map[int32]string{
	0: "MAJORITY",
	1: "DR_AUTO_SYNC",
}

var ReplicationMode_value = map[string]int32{
	"MAJORITY":     0,
	"DR_AUTO_SYNC": 1,
}

func (x ReplicationMode) String() string {
	return proto.EnumName(ReplicationMode_name, int32(x))
}

func (ReplicationMode) EnumDescriptor() ([]byte, []int) {
	return fileDescriptor_405bb93d9863dfea, []int{0}
}

type DRAutoSyncState int32

const (
	// Raft logs need to sync between different DCs
	DRAutoSyncState_SYNC DRAutoSyncState = 0
	// Wait for switching to ASYNC. Stop sync raft logs between DCs.
	DRAutoSyncState_ASYNC_WAIT DRAutoSyncState = 1
	// Raft logs need to sync to majority peers
	DRAutoSyncState_ASYNC DRAutoSyncState = 2
	// Switching from ASYNC to SYNC mode
	DRAutoSyncState_SYNC_RECOVER DRAutoSyncState = 3
)

var DRAutoSyncState_name = map[int32]string{
	0: "SYNC",
	1: "ASYNC_WAIT",
	2: "ASYNC",
	3: "SYNC_RECOVER",
}

var DRAutoSyncState_value = map[string]int32{
	"SYNC":         0,
	"ASYNC_WAIT":   1,
	"ASYNC":        2,
	"SYNC_RECOVER": 3,
}

func (x DRAutoSyncState) String() string {
	return proto.EnumName(DRAutoSyncState_name, int32(x))
}

func (DRAutoSyncState) EnumDescriptor() ([]byte, []int) {
	return fileDescriptor_405bb93d9863dfea, []int{1}
}

type RegionReplicationState int32

const (
	// The region's state is unknown
	RegionReplicationState_UNKNOWN RegionReplicationState = 0
	// Logs sync to majority peers
	RegionReplicationState_SIMPLE_MAJORITY RegionReplicationState = 1
	// Logs sync to different DCs
	RegionReplicationState_INTEGRITY_OVER_LABEL RegionReplicationState = 2
)

var RegionReplicationState_name = map[int32]string{
	0: "UNKNOWN",
	1: "SIMPLE_MAJORITY",
	2: "INTEGRITY_OVER_LABEL",
}

var RegionReplicationState_value = map[string]int32{
	"UNKNOWN":              0,
	"SIMPLE_MAJORITY":      1,
	"INTEGRITY_OVER_LABEL": 2,
}

func (x RegionReplicationState) String() string {
	return proto.EnumName(RegionReplicationState_name, int32(x))
}

func (RegionReplicationState) EnumDescriptor() ([]byte, []int) {
	return fileDescriptor_405bb93d9863dfea, []int{2}
}

// The replication status sync from PD to TiKV.
type ReplicationStatus struct {
	Mode                 ReplicationMode `protobuf:"varint,1,opt,name=mode,proto3,enum=replication_modepb.ReplicationMode" json:"mode,omitempty"`
	DrAutoSync           *DRAutoSync     `protobuf:"bytes,2,opt,name=dr_auto_sync,json=drAutoSync,proto3" json:"dr_auto_sync,omitempty"`
	XXX_NoUnkeyedLiteral struct{}        `json:"-"`
	XXX_unrecognized     []byte          `json:"-"`
	XXX_sizecache        int32           `json:"-"`
}

func (m *ReplicationStatus) Reset()         { *m = ReplicationStatus{} }
func (m *ReplicationStatus) String() string { return proto.CompactTextString(m) }
func (*ReplicationStatus) ProtoMessage()    {}
func (*ReplicationStatus) Descriptor() ([]byte, []int) {
	return fileDescriptor_405bb93d9863dfea, []int{0}
}
func (m *ReplicationStatus) XXX_Unmarshal(b []byte) error {
	return m.Unmarshal(b)
}
func (m *ReplicationStatus) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	if deterministic {
		return xxx_messageInfo_ReplicationStatus.Marshal(b, m, deterministic)
	} else {
		b = b[:cap(b)]
		n, err := m.MarshalToSizedBuffer(b)
		if err != nil {
			return nil, err
		}
		return b[:n], nil
	}
}
func (m *ReplicationStatus) XXX_Merge(src proto.Message) {
	xxx_messageInfo_ReplicationStatus.Merge(m, src)
}
func (m *ReplicationStatus) XXX_Size() int {
	return m.Size()
}
func (m *ReplicationStatus) XXX_DiscardUnknown() {
	xxx_messageInfo_ReplicationStatus.DiscardUnknown(m)
}

var xxx_messageInfo_ReplicationStatus proto.InternalMessageInfo

func (m *ReplicationStatus) GetMode() ReplicationMode {
	if m != nil {
		return m.Mode
	}
	return ReplicationMode_MAJORITY
}

func (m *ReplicationStatus) GetDrAutoSync() *DRAutoSync {
	if m != nil {
		return m.DrAutoSync
	}
	return nil
}

// The status of dr-autosync mode.
type DRAutoSync struct {
	// The key of the label that used for distinguish different DC.
	LabelKey string          `protobuf:"bytes,1,opt,name=label_key,json=labelKey,proto3" json:"label_key,omitempty"`
	State    DRAutoSyncState `protobuf:"varint,2,opt,name=state,proto3,enum=replication_modepb.DRAutoSyncState" json:"state,omitempty"`
	// Unique ID of the state, it increases after each state transfer.
	StateId uint64 `protobuf:"varint,3,opt,name=state_id,json=stateId,proto3" json:"state_id,omitempty"`
	// Duration to wait before switching to SYNC by force (in seconds)
	WaitSyncTimeoutHint int32 `protobuf:"varint,4,opt,name=wait_sync_timeout_hint,json=waitSyncTimeoutHint,proto3" json:"wait_sync_timeout_hint,omitempty"`
	// Stores should only sync messages with available stores when state is ASYNC or ASYNC_WAIT.
	AvailableStores      []uint64 `protobuf:"varint,5,rep,packed,name=available_stores,json=availableStores,proto3" json:"available_stores,omitempty"`
	XXX_NoUnkeyedLiteral struct{} `json:"-"`
	XXX_unrecognized     []byte   `json:"-"`
	XXX_sizecache        int32    `json:"-"`
}

func (m *DRAutoSync) Reset()         { *m = DRAutoSync{} }
func (m *DRAutoSync) String() string { return proto.CompactTextString(m) }
func (*DRAutoSync) ProtoMessage()    {}
func (*DRAutoSync) Descriptor() ([]byte, []int) {
	return fileDescriptor_405bb93d9863dfea, []int{1}
}
func (m *DRAutoSync) XXX_Unmarshal(b []byte) error {
	return m.Unmarshal(b)
}
func (m *DRAutoSync) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	if deterministic {
		return xxx_messageInfo_DRAutoSync.Marshal(b, m, deterministic)
	} else {
		b = b[:cap(b)]
		n, err := m.MarshalToSizedBuffer(b)
		if err != nil {
			return nil, err
		}
		return b[:n], nil
	}
}
func (m *DRAutoSync) XXX_Merge(src proto.Message) {
	xxx_messageInfo_DRAutoSync.Merge(m, src)
}
func (m *DRAutoSync) XXX_Size() int {
	return m.Size()
}
func (m *DRAutoSync) XXX_DiscardUnknown() {
	xxx_messageInfo_DRAutoSync.DiscardUnknown(m)
}

var xxx_messageInfo_DRAutoSync proto.InternalMessageInfo

func (m *DRAutoSync) GetLabelKey() string {
	if m != nil {
		return m.LabelKey
	}
	return ""
}

func (m *DRAutoSync) GetState() DRAutoSyncState {
	if m != nil {
		return m.State
	}
	return DRAutoSyncState_SYNC
}

func (m *DRAutoSync) GetStateId() uint64 {
	if m != nil {
		return m.StateId
	}
	return 0
}

func (m *DRAutoSync) GetWaitSyncTimeoutHint() int32 {
	if m != nil {
		return m.WaitSyncTimeoutHint
	}
	return 0
}

func (m *DRAutoSync) GetAvailableStores() []uint64 {
	if m != nil {
		return m.AvailableStores
	}
	return nil
}

// The replication status sync from TiKV to PD.
type RegionReplicationStatus struct {
	State RegionReplicationState `protobuf:"varint,1,opt,name=state,proto3,enum=replication_modepb.RegionReplicationState" json:"state,omitempty"`
	// Unique ID of the state, it increases after each state transfer.
	StateId              uint64   `protobuf:"varint,2,opt,name=state_id,json=stateId,proto3" json:"state_id,omitempty"`
	XXX_NoUnkeyedLiteral struct{} `json:"-"`
	XXX_unrecognized     []byte   `json:"-"`
	XXX_sizecache        int32    `json:"-"`
}

func (m *RegionReplicationStatus) Reset()         { *m = RegionReplicationStatus{} }
func (m *RegionReplicationStatus) String() string { return proto.CompactTextString(m) }
func (*RegionReplicationStatus) ProtoMessage()    {}
func (*RegionReplicationStatus) Descriptor() ([]byte, []int) {
	return fileDescriptor_405bb93d9863dfea, []int{2}
}
func (m *RegionReplicationStatus) XXX_Unmarshal(b []byte) error {
	return m.Unmarshal(b)
}
func (m *RegionReplicationStatus) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	if deterministic {
		return xxx_messageInfo_RegionReplicationStatus.Marshal(b, m, deterministic)
	} else {
		b = b[:cap(b)]
		n, err := m.MarshalToSizedBuffer(b)
		if err != nil {
			return nil, err
		}
		return b[:n], nil
	}
}
func (m *RegionReplicationStatus) XXX_Merge(src proto.Message) {
	xxx_messageInfo_RegionReplicationStatus.Merge(m, src)
}
func (m *RegionReplicationStatus) XXX_Size() int {
	return m.Size()
}
func (m *RegionReplicationStatus) XXX_DiscardUnknown() {
	xxx_messageInfo_RegionReplicationStatus.DiscardUnknown(m)
}

var xxx_messageInfo_RegionReplicationStatus proto.InternalMessageInfo

func (m *RegionReplicationStatus) GetState() RegionReplicationState {
	if m != nil {
		return m.State
	}
	return RegionReplicationState_UNKNOWN
}

func (m *RegionReplicationStatus) GetStateId() uint64 {
	if m != nil {
		return m.StateId
	}
	return 0
}

type StoreDRAutoSyncStatus struct {
	State                DRAutoSyncState `protobuf:"varint,1,opt,name=state,proto3,enum=replication_modepb.DRAutoSyncState" json:"state,omitempty"`
	StateId              uint64          `protobuf:"varint,2,opt,name=state_id,json=stateId,proto3" json:"state_id,omitempty"`
	XXX_NoUnkeyedLiteral struct{}        `json:"-"`
	XXX_unrecognized     []byte          `json:"-"`
	XXX_sizecache        int32           `json:"-"`
}

func (m *StoreDRAutoSyncStatus) Reset()         { *m = StoreDRAutoSyncStatus{} }
func (m *StoreDRAutoSyncStatus) String() string { return proto.CompactTextString(m) }
func (*StoreDRAutoSyncStatus) ProtoMessage()    {}
func (*StoreDRAutoSyncStatus) Descriptor() ([]byte, []int) {
	return fileDescriptor_405bb93d9863dfea, []int{3}
}
func (m *StoreDRAutoSyncStatus) XXX_Unmarshal(b []byte) error {
	return m.Unmarshal(b)
}
func (m *StoreDRAutoSyncStatus) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	if deterministic {
		return xxx_messageInfo_StoreDRAutoSyncStatus.Marshal(b, m, deterministic)
	} else {
		b = b[:cap(b)]
		n, err := m.MarshalToSizedBuffer(b)
		if err != nil {
			return nil, err
		}
		return b[:n], nil
	}
}
func (m *StoreDRAutoSyncStatus) XXX_Merge(src proto.Message) {
	xxx_messageInfo_StoreDRAutoSyncStatus.Merge(m, src)
}
func (m *StoreDRAutoSyncStatus) XXX_Size() int {
	return m.Size()
}
func (m *StoreDRAutoSyncStatus) XXX_DiscardUnknown() {
	xxx_messageInfo_StoreDRAutoSyncStatus.DiscardUnknown(m)
}

var xxx_messageInfo_StoreDRAutoSyncStatus proto.InternalMessageInfo

func (m *StoreDRAutoSyncStatus) GetState() DRAutoSyncState {
	if m != nil {
		return m.State
	}
	return DRAutoSyncState_SYNC
}

func (m *StoreDRAutoSyncStatus) GetStateId() uint64 {
	if m != nil {
		return m.StateId
	}
	return 0
}

func init() {
	proto.RegisterEnum("replication_modepb.ReplicationMode", ReplicationMode_name, ReplicationMode_value)
	proto.RegisterEnum("replication_modepb.DRAutoSyncState", DRAutoSyncState_name, DRAutoSyncState_value)
	proto.RegisterEnum("replication_modepb.RegionReplicationState", RegionReplicationState_name, RegionReplicationState_value)
	proto.RegisterType((*ReplicationStatus)(nil), "replication_modepb.ReplicationStatus")
	proto.RegisterType((*DRAutoSync)(nil), "replication_modepb.DRAutoSync")
	proto.RegisterType((*RegionReplicationStatus)(nil), "replication_modepb.RegionReplicationStatus")
	proto.RegisterType((*StoreDRAutoSyncStatus)(nil), "replication_modepb.StoreDRAutoSyncStatus")
}

func init() { proto.RegisterFile("replication_modepb.proto", fileDescriptor_405bb93d9863dfea) }

var fileDescriptor_405bb93d9863dfea = []byte{
	// 474 bytes of a gzipped FileDescriptorProto
	0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0xff, 0xa4, 0x53, 0x4d, 0x6f, 0xd3, 0x40,
	0x10, 0xcd, 0xe6, 0x83, 0x26, 0xd3, 0xa8, 0x59, 0xb6, 0x50, 0x8c, 0x90, 0x2c, 0x2b, 0x5c, 0x4c,
	0x0e, 0x95, 0x68, 0x0f, 0x88, 0x5b, 0xdd, 0xd6, 0xa2, 0xa6, 0x89, 0x83, 0xd6, 0x2e, 0x55, 0x4f,
	0x2b, 0x27, 0x5e, 0xc1, 0x0a, 0xc7, 0x1b, 0xd9, 0xeb, 0xa2, 0xfc, 0x09, 0xce, 0xfc, 0x24, 0x8e,
	0xfc, 0x03, 0x50, 0xf8, 0x23, 0xc8, 0x1b, 0xb5, 0x21, 0xad, 0x39, 0xf5, 0x36, 0x33, 0x4f, 0xf3,
	0xe6, 0xbd, 0xa7, 0x5d, 0x30, 0x32, 0x3e, 0x4f, 0xc4, 0x34, 0x52, 0x42, 0xa6, 0x6c, 0x26, 0x63,
	0x3e, 0x9f, 0xec, 0xcf, 0x33, 0xa9, 0x24, 0x21, 0xf7, 0x91, 0xfe, 0x37, 0x04, 0x8f, 0xe9, 0x7a,
	0x1c, 0xa8, 0x48, 0x15, 0x39, 0x79, 0x03, 0xcd, 0x12, 0x37, 0x90, 0x85, 0xec, 0x9d, 0x83, 0x97,
	0xfb, 0x15, 0x94, 0xff, 0x2c, 0x8d, 0x64, 0xcc, 0xa9, 0x5e, 0x20, 0x47, 0xd0, 0x8d, 0x33, 0x16,
	0x15, 0x4a, 0xb2, 0x7c, 0x91, 0x4e, 0x8d, 0xba, 0x85, 0xec, 0xed, 0x03, 0xb3, 0x8a, 0xe0, 0x94,
	0x3a, 0x85, 0x92, 0xc1, 0x22, 0x9d, 0x52, 0x88, 0xb3, 0x9b, 0xba, 0xff, 0x0b, 0x01, 0xac, 0x21,
	0xf2, 0x02, 0x3a, 0x49, 0x34, 0xe1, 0x09, 0xfb, 0xc2, 0x17, 0x5a, 0x4e, 0x87, 0xb6, 0xf5, 0xe0,
	0x9c, 0x2f, 0xc8, 0x5b, 0x68, 0xe5, 0x2a, 0x52, 0x5c, 0x9f, 0xf9, 0x8f, 0xce, 0x35, 0x57, 0xe9,
	0x8d, 0xd3, 0xd5, 0x06, 0x79, 0x0e, 0x6d, 0x5d, 0x30, 0x11, 0x1b, 0x0d, 0x0b, 0xd9, 0x4d, 0xba,
	0xa5, 0x7b, 0x2f, 0x26, 0x87, 0xb0, 0xf7, 0x35, 0x12, 0x4a, 0x1b, 0x60, 0x4a, 0xcc, 0xb8, 0x2c,
	0x14, 0xfb, 0x2c, 0x52, 0x65, 0x34, 0x2d, 0x64, 0xb7, 0xe8, 0x6e, 0x89, 0x96, 0x84, 0xe1, 0x0a,
	0x3b, 0x13, 0xa9, 0x22, 0xaf, 0x00, 0x47, 0xd7, 0x91, 0x48, 0xa2, 0x49, 0xc2, 0x59, 0xae, 0x64,
	0xc6, 0x73, 0xa3, 0x65, 0x35, 0xec, 0x26, 0xed, 0xdd, 0xce, 0x03, 0x3d, 0xee, 0x5f, 0xc3, 0x33,
	0xca, 0x3f, 0x09, 0x99, 0xde, 0xcf, 0xfd, 0xe8, 0xc6, 0xd0, 0x2a, 0xf8, 0x41, 0x75, 0xf0, 0x15,
	0xbb, 0x95, 0xbe, 0xea, 0x1b, 0xbe, 0xfa, 0x33, 0x78, 0xaa, 0x15, 0x6c, 0x26, 0x52, 0xe4, 0xeb,
	0x18, 0xd1, 0x83, 0x62, 0xdc, 0x3c, 0x37, 0x78, 0x0d, 0xbd, 0x3b, 0x6f, 0x84, 0x74, 0xa1, 0x3d,
	0x72, 0xde, 0x8f, 0xa9, 0x17, 0x5e, 0xe1, 0x1a, 0xc1, 0xd0, 0x3d, 0xa5, 0xcc, 0xb9, 0x08, 0xc7,
	0x2c, 0xb8, 0xf2, 0x4f, 0x30, 0x1a, 0x9c, 0x41, 0xef, 0xce, 0x1d, 0xd2, 0x86, 0xa6, 0x06, 0x6b,
	0x64, 0x07, 0xc0, 0x29, 0x4b, 0x76, 0xe9, 0x78, 0x21, 0x46, 0xa4, 0x03, 0x2d, 0xdd, 0xe3, 0x7a,
	0xc9, 0xa4, 0x11, 0xea, 0x9e, 0x8c, 0x3f, 0xba, 0x14, 0x37, 0x06, 0x21, 0xec, 0x55, 0xe7, 0x44,
	0xb6, 0x61, 0xeb, 0xc2, 0x3f, 0xf7, 0xc7, 0x97, 0x3e, 0xae, 0x91, 0x5d, 0xe8, 0x05, 0xde, 0xe8,
	0xc3, 0xd0, 0x65, 0xb7, 0xba, 0x10, 0x31, 0xe0, 0x89, 0xe7, 0x87, 0xee, 0xbb, 0xb2, 0x65, 0x25,
	0x1f, 0x1b, 0x3a, 0xc7, 0xee, 0x10, 0xd7, 0x8f, 0xf1, 0x8f, 0xa5, 0x89, 0x7e, 0x2e, 0x4d, 0xf4,
	0x7b, 0x69, 0xa2, 0xef, 0x7f, 0xcc, 0xda, 0xe4, 0x91, 0xfe, 0x59, 0x87, 0x7f, 0x03, 0x00, 0x00,
	0xff, 0xff, 0x6a, 0xcd, 0x87, 0xeb, 0x75, 0x03, 0x00, 0x00,
}

func (m *ReplicationStatus) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalToSizedBuffer(dAtA[:size])
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *ReplicationStatus) MarshalTo(dAtA []byte) (int, error) {
	size := m.Size()
	return m.MarshalToSizedBuffer(dAtA[:size])
}

func (m *ReplicationStatus) MarshalToSizedBuffer(dAtA []byte) (int, error) {
	i := len(dAtA)
	_ = i
	var l int
	_ = l
	if m.XXX_unrecognized != nil {
		i -= len(m.XXX_unrecognized)
		copy(dAtA[i:], m.XXX_unrecognized)
	}
	if m.DrAutoSync != nil {
		{
			size, err := m.DrAutoSync.MarshalToSizedBuffer(dAtA[:i])
			if err != nil {
				return 0, err
			}
			i -= size
			i = encodeVarintReplicationModepb(dAtA, i, uint64(size))
		}
		i--
		dAtA[i] = 0x12
	}
	if m.Mode != 0 {
		i = encodeVarintReplicationModepb(dAtA, i, uint64(m.Mode))
		i--
		dAtA[i] = 0x8
	}
	return len(dAtA) - i, nil
}

func (m *DRAutoSync) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalToSizedBuffer(dAtA[:size])
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *DRAutoSync) MarshalTo(dAtA []byte) (int, error) {
	size := m.Size()
	return m.MarshalToSizedBuffer(dAtA[:size])
}

func (m *DRAutoSync) MarshalToSizedBuffer(dAtA []byte) (int, error) {
	i := len(dAtA)
	_ = i
	var l int
	_ = l
	if m.XXX_unrecognized != nil {
		i -= len(m.XXX_unrecognized)
		copy(dAtA[i:], m.XXX_unrecognized)
	}
	if len(m.AvailableStores) > 0 {
		dAtA3 := make([]byte, len(m.AvailableStores)*10)
		var j2 int
		for _, num := range m.AvailableStores {
			for num >= 1<<7 {
				dAtA3[j2] = uint8(uint64(num)&0x7f | 0x80)
				num >>= 7
				j2++
			}
			dAtA3[j2] = uint8(num)
			j2++
		}
		i -= j2
		copy(dAtA[i:], dAtA3[:j2])
		i = encodeVarintReplicationModepb(dAtA, i, uint64(j2))
		i--
		dAtA[i] = 0x2a
	}
	if m.WaitSyncTimeoutHint != 0 {
		i = encodeVarintReplicationModepb(dAtA, i, uint64(m.WaitSyncTimeoutHint))
		i--
		dAtA[i] = 0x20
	}
	if m.StateId != 0 {
		i = encodeVarintReplicationModepb(dAtA, i, uint64(m.StateId))
		i--
		dAtA[i] = 0x18
	}
	if m.State != 0 {
		i = encodeVarintReplicationModepb(dAtA, i, uint64(m.State))
		i--
		dAtA[i] = 0x10
	}
	if len(m.LabelKey) > 0 {
		i -= len(m.LabelKey)
		copy(dAtA[i:], m.LabelKey)
		i = encodeVarintReplicationModepb(dAtA, i, uint64(len(m.LabelKey)))
		i--
		dAtA[i] = 0xa
	}
	return len(dAtA) - i, nil
}

func (m *RegionReplicationStatus) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalToSizedBuffer(dAtA[:size])
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *RegionReplicationStatus) MarshalTo(dAtA []byte) (int, error) {
	size := m.Size()
	return m.MarshalToSizedBuffer(dAtA[:size])
}

func (m *RegionReplicationStatus) MarshalToSizedBuffer(dAtA []byte) (int, error) {
	i := len(dAtA)
	_ = i
	var l int
	_ = l
	if m.XXX_unrecognized != nil {
		i -= len(m.XXX_unrecognized)
		copy(dAtA[i:], m.XXX_unrecognized)
	}
	if m.StateId != 0 {
		i = encodeVarintReplicationModepb(dAtA, i, uint64(m.StateId))
		i--
		dAtA[i] = 0x10
	}
	if m.State != 0 {
		i = encodeVarintReplicationModepb(dAtA, i, uint64(m.State))
		i--
		dAtA[i] = 0x8
	}
	return len(dAtA) - i, nil
}

func (m *StoreDRAutoSyncStatus) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalToSizedBuffer(dAtA[:size])
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *StoreDRAutoSyncStatus) MarshalTo(dAtA []byte) (int, error) {
	size := m.Size()
	return m.MarshalToSizedBuffer(dAtA[:size])
}

func (m *StoreDRAutoSyncStatus) MarshalToSizedBuffer(dAtA []byte) (int, error) {
	i := len(dAtA)
	_ = i
	var l int
	_ = l
	if m.XXX_unrecognized != nil {
		i -= len(m.XXX_unrecognized)
		copy(dAtA[i:], m.XXX_unrecognized)
	}
	if m.StateId != 0 {
		i = encodeVarintReplicationModepb(dAtA, i, uint64(m.StateId))
		i--
		dAtA[i] = 0x10
	}
	if m.State != 0 {
		i = encodeVarintReplicationModepb(dAtA, i, uint64(m.State))
		i--
		dAtA[i] = 0x8
	}
	return len(dAtA) - i, nil
}

func encodeVarintReplicationModepb(dAtA []byte, offset int, v uint64) int {
	offset -= sovReplicationModepb(v)
	base := offset
	for v >= 1<<7 {
		dAtA[offset] = uint8(v&0x7f | 0x80)
		v >>= 7
		offset++
	}
	dAtA[offset] = uint8(v)
	return base
}
func (m *ReplicationStatus) Size() (n int) {
	if m == nil {
		return 0
	}
	var l int
	_ = l
	if m.Mode != 0 {
		n += 1 + sovReplicationModepb(uint64(m.Mode))
	}
	if m.DrAutoSync != nil {
		l = m.DrAutoSync.Size()
		n += 1 + l + sovReplicationModepb(uint64(l))
	}
	if m.XXX_unrecognized != nil {
		n += len(m.XXX_unrecognized)
	}
	return n
}

func (m *DRAutoSync) Size() (n int) {
	if m == nil {
		return 0
	}
	var l int
	_ = l
	l = len(m.LabelKey)
	if l > 0 {
		n += 1 + l + sovReplicationModepb(uint64(l))
	}
	if m.State != 0 {
		n += 1 + sovReplicationModepb(uint64(m.State))
	}
	if m.StateId != 0 {
		n += 1 + sovReplicationModepb(uint64(m.StateId))
	}
	if m.WaitSyncTimeoutHint != 0 {
		n += 1 + sovReplicationModepb(uint64(m.WaitSyncTimeoutHint))
	}
	if len(m.AvailableStores) > 0 {
		l = 0
		for _, e := range m.AvailableStores {
			l += sovReplicationModepb(uint64(e))
		}
		n += 1 + sovReplicationModepb(uint64(l)) + l
	}
	if m.XXX_unrecognized != nil {
		n += len(m.XXX_unrecognized)
	}
	return n
}

func (m *RegionReplicationStatus) Size() (n int) {
	if m == nil {
		return 0
	}
	var l int
	_ = l
	if m.State != 0 {
		n += 1 + sovReplicationModepb(uint64(m.State))
	}
	if m.StateId != 0 {
		n += 1 + sovReplicationModepb(uint64(m.StateId))
	}
	if m.XXX_unrecognized != nil {
		n += len(m.XXX_unrecognized)
	}
	return n
}

func (m *StoreDRAutoSyncStatus) Size() (n int) {
	if m == nil {
		return 0
	}
	var l int
	_ = l
	if m.State != 0 {
		n += 1 + sovReplicationModepb(uint64(m.State))
	}
	if m.StateId != 0 {
		n += 1 + sovReplicationModepb(uint64(m.StateId))
	}
	if m.XXX_unrecognized != nil {
		n += len(m.XXX_unrecognized)
	}
	return n
}

func sovReplicationModepb(x uint64) (n int) {
	return (math_bits.Len64(x|1) + 6) / 7
}
func sozReplicationModepb(x uint64) (n int) {
	return sovReplicationModepb(uint64((x << 1) ^ uint64((int64(x) >> 63))))
}
func (m *ReplicationStatus) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowReplicationModepb
			}
			if iNdEx >= l {
				return io.ErrUnexpectedEOF
			}
			b := dAtA[iNdEx]
			iNdEx++
			wire |= uint64(b&0x7F) << shift
			if b < 0x80 {
				break
			}
		}
		fieldNum := int32(wire >> 3)
		wireType := int(wire & 0x7)
		if wireType == 4 {
			return fmt.Errorf("proto: ReplicationStatus: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: ReplicationStatus: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field Mode", wireType)
			}
			m.Mode = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowReplicationModepb
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				m.Mode |= ReplicationMode(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		case 2:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field DrAutoSync", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowReplicationModepb
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				msglen |= int(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if msglen < 0 {
				return ErrInvalidLengthReplicationModepb
			}
			postIndex := iNdEx + msglen
			if postIndex < 0 {
				return ErrInvalidLengthReplicationModepb
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			if m.DrAutoSync == nil {
				m.DrAutoSync = &DRAutoSync{}
			}
			if err := m.DrAutoSync.Unmarshal(dAtA[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		default:
			iNdEx = preIndex
			skippy, err := skipReplicationModepb(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if (skippy < 0) || (iNdEx+skippy) < 0 {
				return ErrInvalidLengthReplicationModepb
			}
			if (iNdEx + skippy) > l {
				return io.ErrUnexpectedEOF
			}
			m.XXX_unrecognized = append(m.XXX_unrecognized, dAtA[iNdEx:iNdEx+skippy]...)
			iNdEx += skippy
		}
	}

	if iNdEx > l {
		return io.ErrUnexpectedEOF
	}
	return nil
}
func (m *DRAutoSync) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowReplicationModepb
			}
			if iNdEx >= l {
				return io.ErrUnexpectedEOF
			}
			b := dAtA[iNdEx]
			iNdEx++
			wire |= uint64(b&0x7F) << shift
			if b < 0x80 {
				break
			}
		}
		fieldNum := int32(wire >> 3)
		wireType := int(wire & 0x7)
		if wireType == 4 {
			return fmt.Errorf("proto: DRAutoSync: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: DRAutoSync: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field LabelKey", wireType)
			}
			var stringLen uint64
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowReplicationModepb
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				stringLen |= uint64(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			intStringLen := int(stringLen)
			if intStringLen < 0 {
				return ErrInvalidLengthReplicationModepb
			}
			postIndex := iNdEx + intStringLen
			if postIndex < 0 {
				return ErrInvalidLengthReplicationModepb
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.LabelKey = string(dAtA[iNdEx:postIndex])
			iNdEx = postIndex
		case 2:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field State", wireType)
			}
			m.State = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowReplicationModepb
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				m.State |= DRAutoSyncState(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		case 3:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field StateId", wireType)
			}
			m.StateId = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowReplicationModepb
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				m.StateId |= uint64(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		case 4:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field WaitSyncTimeoutHint", wireType)
			}
			m.WaitSyncTimeoutHint = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowReplicationModepb
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				m.WaitSyncTimeoutHint |= int32(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		case 5:
			if wireType == 0 {
				var v uint64
				for shift := uint(0); ; shift += 7 {
					if shift >= 64 {
						return ErrIntOverflowReplicationModepb
					}
					if iNdEx >= l {
						return io.ErrUnexpectedEOF
					}
					b := dAtA[iNdEx]
					iNdEx++
					v |= uint64(b&0x7F) << shift
					if b < 0x80 {
						break
					}
				}
				m.AvailableStores = append(m.AvailableStores, v)
			} else if wireType == 2 {
				var packedLen int
				for shift := uint(0); ; shift += 7 {
					if shift >= 64 {
						return ErrIntOverflowReplicationModepb
					}
					if iNdEx >= l {
						return io.ErrUnexpectedEOF
					}
					b := dAtA[iNdEx]
					iNdEx++
					packedLen |= int(b&0x7F) << shift
					if b < 0x80 {
						break
					}
				}
				if packedLen < 0 {
					return ErrInvalidLengthReplicationModepb
				}
				postIndex := iNdEx + packedLen
				if postIndex < 0 {
					return ErrInvalidLengthReplicationModepb
				}
				if postIndex > l {
					return io.ErrUnexpectedEOF
				}
				var elementCount int
				var count int
				for _, integer := range dAtA[iNdEx:postIndex] {
					if integer < 128 {
						count++
					}
				}
				elementCount = count
				if elementCount != 0 && len(m.AvailableStores) == 0 {
					m.AvailableStores = make([]uint64, 0, elementCount)
				}
				for iNdEx < postIndex {
					var v uint64
					for shift := uint(0); ; shift += 7 {
						if shift >= 64 {
							return ErrIntOverflowReplicationModepb
						}
						if iNdEx >= l {
							return io.ErrUnexpectedEOF
						}
						b := dAtA[iNdEx]
						iNdEx++
						v |= uint64(b&0x7F) << shift
						if b < 0x80 {
							break
						}
					}
					m.AvailableStores = append(m.AvailableStores, v)
				}
			} else {
				return fmt.Errorf("proto: wrong wireType = %d for field AvailableStores", wireType)
			}
		default:
			iNdEx = preIndex
			skippy, err := skipReplicationModepb(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if (skippy < 0) || (iNdEx+skippy) < 0 {
				return ErrInvalidLengthReplicationModepb
			}
			if (iNdEx + skippy) > l {
				return io.ErrUnexpectedEOF
			}
			m.XXX_unrecognized = append(m.XXX_unrecognized, dAtA[iNdEx:iNdEx+skippy]...)
			iNdEx += skippy
		}
	}

	if iNdEx > l {
		return io.ErrUnexpectedEOF
	}
	return nil
}
func (m *RegionReplicationStatus) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowReplicationModepb
			}
			if iNdEx >= l {
				return io.ErrUnexpectedEOF
			}
			b := dAtA[iNdEx]
			iNdEx++
			wire |= uint64(b&0x7F) << shift
			if b < 0x80 {
				break
			}
		}
		fieldNum := int32(wire >> 3)
		wireType := int(wire & 0x7)
		if wireType == 4 {
			return fmt.Errorf("proto: RegionReplicationStatus: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: RegionReplicationStatus: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field State", wireType)
			}
			m.State = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowReplicationModepb
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				m.State |= RegionReplicationState(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		case 2:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field StateId", wireType)
			}
			m.StateId = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowReplicationModepb
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				m.StateId |= uint64(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		default:
			iNdEx = preIndex
			skippy, err := skipReplicationModepb(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if (skippy < 0) || (iNdEx+skippy) < 0 {
				return ErrInvalidLengthReplicationModepb
			}
			if (iNdEx + skippy) > l {
				return io.ErrUnexpectedEOF
			}
			m.XXX_unrecognized = append(m.XXX_unrecognized, dAtA[iNdEx:iNdEx+skippy]...)
			iNdEx += skippy
		}
	}

	if iNdEx > l {
		return io.ErrUnexpectedEOF
	}
	return nil
}
func (m *StoreDRAutoSyncStatus) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowReplicationModepb
			}
			if iNdEx >= l {
				return io.ErrUnexpectedEOF
			}
			b := dAtA[iNdEx]
			iNdEx++
			wire |= uint64(b&0x7F) << shift
			if b < 0x80 {
				break
			}
		}
		fieldNum := int32(wire >> 3)
		wireType := int(wire & 0x7)
		if wireType == 4 {
			return fmt.Errorf("proto: StoreDRAutoSyncStatus: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: StoreDRAutoSyncStatus: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field State", wireType)
			}
			m.State = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowReplicationModepb
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				m.State |= DRAutoSyncState(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		case 2:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field StateId", wireType)
			}
			m.StateId = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowReplicationModepb
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				m.StateId |= uint64(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		default:
			iNdEx = preIndex
			skippy, err := skipReplicationModepb(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if (skippy < 0) || (iNdEx+skippy) < 0 {
				return ErrInvalidLengthReplicationModepb
			}
			if (iNdEx + skippy) > l {
				return io.ErrUnexpectedEOF
			}
			m.XXX_unrecognized = append(m.XXX_unrecognized, dAtA[iNdEx:iNdEx+skippy]...)
			iNdEx += skippy
		}
	}

	if iNdEx > l {
		return io.ErrUnexpectedEOF
	}
	return nil
}
func skipReplicationModepb(dAtA []byte) (n int, err error) {
	l := len(dAtA)
	iNdEx := 0
	depth := 0
	for iNdEx < l {
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return 0, ErrIntOverflowReplicationModepb
			}
			if iNdEx >= l {
				return 0, io.ErrUnexpectedEOF
			}
			b := dAtA[iNdEx]
			iNdEx++
			wire |= (uint64(b) & 0x7F) << shift
			if b < 0x80 {
				break
			}
		}
		wireType := int(wire & 0x7)
		switch wireType {
		case 0:
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return 0, ErrIntOverflowReplicationModepb
				}
				if iNdEx >= l {
					return 0, io.ErrUnexpectedEOF
				}
				iNdEx++
				if dAtA[iNdEx-1] < 0x80 {
					break
				}
			}
		case 1:
			iNdEx += 8
		case 2:
			var length int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return 0, ErrIntOverflowReplicationModepb
				}
				if iNdEx >= l {
					return 0, io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				length |= (int(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if length < 0 {
				return 0, ErrInvalidLengthReplicationModepb
			}
			iNdEx += length
		case 3:
			depth++
		case 4:
			if depth == 0 {
				return 0, ErrUnexpectedEndOfGroupReplicationModepb
			}
			depth--
		case 5:
			iNdEx += 4
		default:
			return 0, fmt.Errorf("proto: illegal wireType %d", wireType)
		}
		if iNdEx < 0 {
			return 0, ErrInvalidLengthReplicationModepb
		}
		if depth == 0 {
			return iNdEx, nil
		}
	}
	return 0, io.ErrUnexpectedEOF
}

var (
	ErrInvalidLengthReplicationModepb        = fmt.Errorf("proto: negative length found during unmarshaling")
	ErrIntOverflowReplicationModepb          = fmt.Errorf("proto: integer overflow")
	ErrUnexpectedEndOfGroupReplicationModepb = fmt.Errorf("proto: unexpected end of group")
)
