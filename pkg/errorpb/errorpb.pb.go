// Code generated by protoc-gen-go.
// source: errorpb.proto
// DO NOT EDIT!

/*
Package errorpb is a generated protocol buffer package.

It is generated from these files:
	errorpb.proto

It has these top-level messages:
	NotLeader
	RegionNotFound
	KeyNotInRegion
	StaleEpoch
	Error
*/
package errorpb

import proto "github.com/golang/protobuf/proto"
import fmt "fmt"
import math "math"

// Reference imports to suppress errors if they are not otherwise used.
var _ = proto.Marshal
var _ = fmt.Errorf
var _ = math.Inf

// This is a compile-time assertion to ensure that this generated file
// is compatible with the proto package it is being compiled against.
const _ = proto.ProtoPackageIsVersion1

type NotLeader struct {
	RegionId         *uint64 `protobuf:"varint,1,opt,name=region_id,json=regionId" json:"region_id,omitempty"`
	LeaderStoreId    *uint64 `protobuf:"varint,2,opt,name=leader_store_id,json=leaderStoreId" json:"leader_store_id,omitempty"`
	XXX_unrecognized []byte  `json:"-"`
}

func (m *NotLeader) Reset()                    { *m = NotLeader{} }
func (m *NotLeader) String() string            { return proto.CompactTextString(m) }
func (*NotLeader) ProtoMessage()               {}
func (*NotLeader) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{0} }

func (m *NotLeader) GetRegionId() uint64 {
	if m != nil && m.RegionId != nil {
		return *m.RegionId
	}
	return 0
}

func (m *NotLeader) GetLeaderStoreId() uint64 {
	if m != nil && m.LeaderStoreId != nil {
		return *m.LeaderStoreId
	}
	return 0
}

type RegionNotFound struct {
	RegionId         *uint64 `protobuf:"varint,1,opt,name=region_id,json=regionId" json:"region_id,omitempty"`
	XXX_unrecognized []byte  `json:"-"`
}

func (m *RegionNotFound) Reset()                    { *m = RegionNotFound{} }
func (m *RegionNotFound) String() string            { return proto.CompactTextString(m) }
func (*RegionNotFound) ProtoMessage()               {}
func (*RegionNotFound) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{1} }

func (m *RegionNotFound) GetRegionId() uint64 {
	if m != nil && m.RegionId != nil {
		return *m.RegionId
	}
	return 0
}

type KeyNotInRegion struct {
	Key              []byte  `protobuf:"bytes,1,opt,name=key" json:"key,omitempty"`
	RegionId         *uint64 `protobuf:"varint,2,opt,name=region_id,json=regionId" json:"region_id,omitempty"`
	StartKey         []byte  `protobuf:"bytes,3,opt,name=start_key,json=startKey" json:"start_key,omitempty"`
	EndKey           []byte  `protobuf:"bytes,4,opt,name=end_key,json=endKey" json:"end_key,omitempty"`
	XXX_unrecognized []byte  `json:"-"`
}

func (m *KeyNotInRegion) Reset()                    { *m = KeyNotInRegion{} }
func (m *KeyNotInRegion) String() string            { return proto.CompactTextString(m) }
func (*KeyNotInRegion) ProtoMessage()               {}
func (*KeyNotInRegion) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{2} }

func (m *KeyNotInRegion) GetKey() []byte {
	if m != nil {
		return m.Key
	}
	return nil
}

func (m *KeyNotInRegion) GetRegionId() uint64 {
	if m != nil && m.RegionId != nil {
		return *m.RegionId
	}
	return 0
}

func (m *KeyNotInRegion) GetStartKey() []byte {
	if m != nil {
		return m.StartKey
	}
	return nil
}

func (m *KeyNotInRegion) GetEndKey() []byte {
	if m != nil {
		return m.EndKey
	}
	return nil
}

type StaleEpoch struct {
	XXX_unrecognized []byte `json:"-"`
}

func (m *StaleEpoch) Reset()                    { *m = StaleEpoch{} }
func (m *StaleEpoch) String() string            { return proto.CompactTextString(m) }
func (*StaleEpoch) ProtoMessage()               {}
func (*StaleEpoch) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{3} }

type Error struct {
	Message          *string         `protobuf:"bytes,1,opt,name=message" json:"message,omitempty"`
	NotLeader        *NotLeader      `protobuf:"bytes,2,opt,name=not_leader,json=notLeader" json:"not_leader,omitempty"`
	RegionNotFound   *RegionNotFound `protobuf:"bytes,3,opt,name=region_not_found,json=regionNotFound" json:"region_not_found,omitempty"`
	KeyNotInRegion   *KeyNotInRegion `protobuf:"bytes,4,opt,name=key_not_in_region,json=keyNotInRegion" json:"key_not_in_region,omitempty"`
	StaleEpoch       *StaleEpoch     `protobuf:"bytes,5,opt,name=stale_epoch,json=staleEpoch" json:"stale_epoch,omitempty"`
	XXX_unrecognized []byte          `json:"-"`
}

func (m *Error) Reset()                    { *m = Error{} }
func (m *Error) String() string            { return proto.CompactTextString(m) }
func (*Error) ProtoMessage()               {}
func (*Error) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{4} }

func (m *Error) GetMessage() string {
	if m != nil && m.Message != nil {
		return *m.Message
	}
	return ""
}

func (m *Error) GetNotLeader() *NotLeader {
	if m != nil {
		return m.NotLeader
	}
	return nil
}

func (m *Error) GetRegionNotFound() *RegionNotFound {
	if m != nil {
		return m.RegionNotFound
	}
	return nil
}

func (m *Error) GetKeyNotInRegion() *KeyNotInRegion {
	if m != nil {
		return m.KeyNotInRegion
	}
	return nil
}

func (m *Error) GetStaleEpoch() *StaleEpoch {
	if m != nil {
		return m.StaleEpoch
	}
	return nil
}

func init() {
	proto.RegisterType((*NotLeader)(nil), "errorpb.NotLeader")
	proto.RegisterType((*RegionNotFound)(nil), "errorpb.RegionNotFound")
	proto.RegisterType((*KeyNotInRegion)(nil), "errorpb.KeyNotInRegion")
	proto.RegisterType((*StaleEpoch)(nil), "errorpb.StaleEpoch")
	proto.RegisterType((*Error)(nil), "errorpb.Error")
}

var fileDescriptor0 = []byte{
	// 314 bytes of a gzipped FileDescriptorProto
	0x1f, 0x8b, 0x08, 0x00, 0x00, 0x09, 0x6e, 0x88, 0x02, 0xff, 0x7c, 0x90, 0x4f, 0x4e, 0xf3, 0x30,
	0x10, 0xc5, 0xd5, 0x7f, 0x5f, 0x9b, 0x49, 0x9b, 0xaf, 0x98, 0x45, 0x23, 0xb1, 0x41, 0x59, 0x20,
	0x36, 0x54, 0xa2, 0xe2, 0x02, 0x20, 0x15, 0x29, 0x02, 0x21, 0xe4, 0x1e, 0xc0, 0x0a, 0x78, 0x28,
	0x51, 0x8b, 0x1d, 0xd9, 0xee, 0xa2, 0xc7, 0xe0, 0xc6, 0xd8, 0x93, 0x36, 0x10, 0x16, 0xec, 0xec,
	0x99, 0xf7, 0x9e, 0xde, 0xfc, 0x60, 0x82, 0xc6, 0x68, 0x53, 0xbd, 0xcc, 0x2b, 0xa3, 0x9d, 0x66,
	0xc3, 0xc3, 0x37, 0x7b, 0x86, 0xe8, 0x49, 0xbb, 0x47, 0x2c, 0x24, 0x1a, 0x76, 0x06, 0x91, 0xc1,
	0x75, 0xa9, 0x95, 0x28, 0x65, 0xda, 0x39, 0xef, 0x5c, 0xf6, 0xf9, 0xa8, 0x1e, 0xe4, 0x92, 0x5d,
	0xc0, 0xff, 0x2d, 0xc9, 0x84, 0x75, 0xda, 0x60, 0x90, 0x74, 0x49, 0x32, 0xa9, 0xc7, 0xab, 0x30,
	0xcd, 0x65, 0x76, 0x05, 0x09, 0x27, 0x8f, 0xcf, 0xbd, 0xd7, 0x3b, 0x25, 0xff, 0x8c, 0xcd, 0x76,
	0x90, 0x3c, 0xe0, 0xde, 0x6b, 0x73, 0x55, 0xdb, 0xd8, 0x14, 0x7a, 0x1b, 0xdc, 0x93, 0x70, 0xcc,
	0xc3, 0xb3, 0x1d, 0xd0, 0xfd, 0xd5, 0xcb, 0x2f, 0xad, 0x2b, 0x8c, 0x13, 0xc1, 0xd4, 0x23, 0xd3,
	0x88, 0x06, 0x3e, 0x96, 0xcd, 0x60, 0x88, 0x4a, 0xd2, 0xaa, 0x4f, 0xab, 0x7f, 0xfe, 0xeb, 0x17,
	0xd9, 0x18, 0x60, 0xe5, 0x8a, 0x2d, 0x2e, 0x2b, 0xfd, 0xfa, 0x9e, 0x7d, 0x76, 0x61, 0xb0, 0x0c,
	0x44, 0x58, 0x0a, 0xc3, 0x0f, 0xb4, 0xb6, 0x58, 0x23, 0x15, 0x88, 0xf8, 0xf1, 0xcb, 0xae, 0x01,
	0x94, 0x76, 0xa2, 0x3e, 0x96, 0x5a, 0xc4, 0x0b, 0x36, 0x3f, 0x62, 0x6d, 0x20, 0xf2, 0x48, 0x35,
	0x3c, 0x6f, 0x61, 0x7a, 0xe8, 0x1d, 0x9c, 0x6f, 0x01, 0x06, 0x35, 0x8c, 0x17, 0xb3, 0xc6, 0xd8,
	0x66, 0xc5, 0x13, 0xd3, 0x66, 0x77, 0x07, 0x27, 0xbe, 0x3c, 0xf9, 0x4b, 0x25, 0xea, 0x25, 0x9d,
	0xf2, 0x33, 0xa3, 0x0d, 0x90, 0x27, 0x9b, 0x36, 0xd0, 0x1b, 0x88, 0x6d, 0xb8, 0x55, 0x60, 0x38,
	0x36, 0x1d, 0x90, 0xfb, 0xb4, 0x71, 0x7f, 0x73, 0xe0, 0x60, 0x9b, 0xf7, 0x57, 0x00, 0x00, 0x00,
	0xff, 0xff, 0x4a, 0x42, 0xe6, 0xfb, 0x32, 0x02, 0x00, 0x00,
}
