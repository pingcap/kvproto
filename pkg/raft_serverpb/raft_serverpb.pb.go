// Code generated by protoc-gen-go.
// source: raft_serverpb.proto
// DO NOT EDIT!

/*
Package raft_serverpb is a generated protocol buffer package.

It is generated from these files:
	raft_serverpb.proto

It has these top-level messages:
	RaftMessage
	RaftTruncatedState
	KeyValue
	RaftSnapshotData
	StoreIdent
*/
package raft_serverpb

import proto "github.com/golang/protobuf/proto"
import fmt "fmt"
import math "math"
import raftpb "github.com/pingcap/kvproto/pkg/raftpb"
import metapb "github.com/pingcap/kvproto/pkg/metapb"

// Reference imports to suppress errors if they are not otherwise used.
var _ = proto.Marshal
var _ = fmt.Errorf
var _ = math.Inf

// This is a compile-time assertion to ensure that this generated file
// is compatible with the proto package it is being compiled against.
const _ = proto.ProtoPackageIsVersion1

type RaftMessage struct {
	RegionId         *uint64             `protobuf:"varint,1,opt,name=region_id,json=regionId" json:"region_id,omitempty"`
	Message          *raftpb.Message     `protobuf:"bytes,2,opt,name=message" json:"message,omitempty"`
	RegionEpoch      *metapb.RegionEpoch `protobuf:"bytes,3,opt,name=region_epoch,json=regionEpoch" json:"region_epoch,omitempty"`
	XXX_unrecognized []byte              `json:"-"`
}

func (m *RaftMessage) Reset()                    { *m = RaftMessage{} }
func (m *RaftMessage) String() string            { return proto.CompactTextString(m) }
func (*RaftMessage) ProtoMessage()               {}
func (*RaftMessage) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{0} }

func (m *RaftMessage) GetRegionId() uint64 {
	if m != nil && m.RegionId != nil {
		return *m.RegionId
	}
	return 0
}

func (m *RaftMessage) GetMessage() *raftpb.Message {
	if m != nil {
		return m.Message
	}
	return nil
}

func (m *RaftMessage) GetRegionEpoch() *metapb.RegionEpoch {
	if m != nil {
		return m.RegionEpoch
	}
	return nil
}

type RaftTruncatedState struct {
	Index            *uint64 `protobuf:"varint,1,opt,name=index" json:"index,omitempty"`
	Term             *uint64 `protobuf:"varint,2,opt,name=term" json:"term,omitempty"`
	XXX_unrecognized []byte  `json:"-"`
}

func (m *RaftTruncatedState) Reset()                    { *m = RaftTruncatedState{} }
func (m *RaftTruncatedState) String() string            { return proto.CompactTextString(m) }
func (*RaftTruncatedState) ProtoMessage()               {}
func (*RaftTruncatedState) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{1} }

func (m *RaftTruncatedState) GetIndex() uint64 {
	if m != nil && m.Index != nil {
		return *m.Index
	}
	return 0
}

func (m *RaftTruncatedState) GetTerm() uint64 {
	if m != nil && m.Term != nil {
		return *m.Term
	}
	return 0
}

type KeyValue struct {
	Key              []byte `protobuf:"bytes,1,opt,name=key" json:"key,omitempty"`
	Value            []byte `protobuf:"bytes,2,opt,name=value" json:"value,omitempty"`
	XXX_unrecognized []byte `json:"-"`
}

func (m *KeyValue) Reset()                    { *m = KeyValue{} }
func (m *KeyValue) String() string            { return proto.CompactTextString(m) }
func (*KeyValue) ProtoMessage()               {}
func (*KeyValue) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{2} }

func (m *KeyValue) GetKey() []byte {
	if m != nil {
		return m.Key
	}
	return nil
}

func (m *KeyValue) GetValue() []byte {
	if m != nil {
		return m.Value
	}
	return nil
}

type RaftSnapshotData struct {
	Region           *metapb.Region `protobuf:"bytes,1,opt,name=region" json:"region,omitempty"`
	Data             []*KeyValue    `protobuf:"bytes,2,rep,name=data" json:"data,omitempty"`
	XXX_unrecognized []byte         `json:"-"`
}

func (m *RaftSnapshotData) Reset()                    { *m = RaftSnapshotData{} }
func (m *RaftSnapshotData) String() string            { return proto.CompactTextString(m) }
func (*RaftSnapshotData) ProtoMessage()               {}
func (*RaftSnapshotData) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{3} }

func (m *RaftSnapshotData) GetRegion() *metapb.Region {
	if m != nil {
		return m.Region
	}
	return nil
}

func (m *RaftSnapshotData) GetData() []*KeyValue {
	if m != nil {
		return m.Data
	}
	return nil
}

type StoreIdent struct {
	ClusterId        *uint64 `protobuf:"varint,1,opt,name=cluster_id,json=clusterId" json:"cluster_id,omitempty"`
	StoreId          *uint64 `protobuf:"varint,2,opt,name=store_id,json=storeId" json:"store_id,omitempty"`
	XXX_unrecognized []byte  `json:"-"`
}

func (m *StoreIdent) Reset()                    { *m = StoreIdent{} }
func (m *StoreIdent) String() string            { return proto.CompactTextString(m) }
func (*StoreIdent) ProtoMessage()               {}
func (*StoreIdent) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{4} }

func (m *StoreIdent) GetClusterId() uint64 {
	if m != nil && m.ClusterId != nil {
		return *m.ClusterId
	}
	return 0
}

func (m *StoreIdent) GetStoreId() uint64 {
	if m != nil && m.StoreId != nil {
		return *m.StoreId
	}
	return 0
}

func init() {
	proto.RegisterType((*RaftMessage)(nil), "raft_serverpb.RaftMessage")
	proto.RegisterType((*RaftTruncatedState)(nil), "raft_serverpb.RaftTruncatedState")
	proto.RegisterType((*KeyValue)(nil), "raft_serverpb.KeyValue")
	proto.RegisterType((*RaftSnapshotData)(nil), "raft_serverpb.RaftSnapshotData")
	proto.RegisterType((*StoreIdent)(nil), "raft_serverpb.StoreIdent")
}

var fileDescriptor0 = []byte{
	// 318 bytes of a gzipped FileDescriptorProto
	0x1f, 0x8b, 0x08, 0x00, 0x00, 0x09, 0x6e, 0x88, 0x02, 0xff, 0x54, 0x91, 0xcf, 0x4e, 0x32, 0x31,
	0x14, 0xc5, 0xc3, 0xc7, 0x7c, 0x02, 0x77, 0x46, 0x25, 0x17, 0x13, 0x11, 0x63, 0x62, 0x66, 0x61,
	0x34, 0x26, 0xb3, 0x60, 0xe1, 0xd2, 0x95, 0x9a, 0x10, 0xe3, 0xa6, 0x18, 0xb7, 0xa4, 0xd2, 0x2b,
	0x10, 0x61, 0x3a, 0x69, 0x0b, 0x91, 0x87, 0xf0, 0x9d, 0xed, 0xbf, 0x09, 0x99, 0x0d, 0xe9, 0xb9,
	0xbf, 0x73, 0x6e, 0x0f, 0x1d, 0x18, 0x28, 0xfe, 0x65, 0x66, 0x9a, 0xd4, 0x8e, 0x54, 0xf5, 0x59,
	0x54, 0x4a, 0x1a, 0x89, 0xc7, 0x8d, 0xe1, 0x28, 0x73, 0xb2, 0x86, 0xa3, 0x6c, 0x43, 0x86, 0xd7,
	0x2a, 0xff, 0x6d, 0x41, 0xca, 0x2c, 0x7e, 0x23, 0xad, 0xf9, 0x82, 0xf0, 0x12, 0x7a, 0x8a, 0x16,
	0x2b, 0x59, 0xce, 0x56, 0x62, 0xd8, 0xba, 0x6e, 0xdd, 0x26, 0xac, 0x1b, 0x06, 0x13, 0x81, 0x77,
	0xd0, 0xd9, 0x04, 0xdf, 0xf0, 0x9f, 0x45, 0xe9, 0xf8, 0xb4, 0x88, 0xab, 0x63, 0x9c, 0xd5, 0x1c,
	0x1f, 0x20, 0x8b, 0x7b, 0xa8, 0x92, 0xf3, 0xe5, 0xb0, 0xed, 0xfd, 0x83, 0x22, 0x5e, 0xce, 0x3c,
	0x7b, 0x76, 0x88, 0xa5, 0xea, 0x20, 0xf2, 0x47, 0x40, 0x57, 0xe7, 0x5d, 0x6d, 0xcb, 0x39, 0x37,
	0x24, 0xa6, 0xc6, 0xfe, 0xe2, 0x19, 0xfc, 0x5f, 0x95, 0x82, 0x7e, 0x62, 0xa3, 0x20, 0x10, 0x21,
	0x31, 0xa4, 0x36, 0xbe, 0x4b, 0xc2, 0xfc, 0x39, 0x1f, 0x43, 0xf7, 0x95, 0xf6, 0x1f, 0x7c, 0xbd,
	0x25, 0xec, 0x43, 0xfb, 0x9b, 0xf6, 0x3e, 0x93, 0x31, 0x77, 0x74, 0x7b, 0x76, 0x0e, 0xf9, 0x48,
	0xc6, 0x82, 0xc8, 0x17, 0xd0, 0x77, 0x77, 0x4e, 0x4b, 0x5e, 0xe9, 0xa5, 0x34, 0x4f, 0xdc, 0x70,
	0xbc, 0x81, 0xa3, 0x50, 0xcb, 0xc7, 0xd3, 0xf1, 0x49, 0xb3, 0x39, 0x8b, 0x14, 0xef, 0x21, 0x11,
	0xd6, 0x6f, 0x17, 0xb6, 0xad, 0xeb, 0xbc, 0x68, 0x7e, 0x8e, 0xba, 0x0a, 0xf3, 0xa6, 0xfc, 0x05,
	0x60, 0x6a, 0xa4, 0xa2, 0x89, 0xa0, 0xd2, 0xe0, 0x15, 0xc0, 0x7c, 0xbd, 0xd5, 0xb6, 0xf6, 0xe1,
	0xad, 0x7b, 0x71, 0x62, 0x1f, 0xfb, 0x02, 0xba, 0xda, 0x99, 0x1d, 0x0c, 0xff, 0xb0, 0xa3, 0x43,
	0xf8, 0x2f, 0x00, 0x00, 0xff, 0xff, 0x45, 0xe8, 0x94, 0x57, 0xf5, 0x01, 0x00, 0x00,
}
