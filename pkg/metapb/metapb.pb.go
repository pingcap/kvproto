// Code generated by protoc-gen-go.
// source: metapb.proto
// DO NOT EDIT!

/*
Package metapb is a generated protocol buffer package.

It is generated from these files:
	metapb.proto

It has these top-level messages:
	Cluster
	Store
	RegionEpoch
	Region
*/
package metapb

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

type Cluster struct {
	Id *uint64 `protobuf:"varint,1,opt,name=id" json:"id,omitempty"`
	// max peer number for a region.
	// pd will do the auto-balance if region peer number mismatches.
	MaxPeerNumber    *uint32 `protobuf:"varint,2,opt,name=max_peer_number,json=maxPeerNumber" json:"max_peer_number,omitempty"`
	XXX_unrecognized []byte  `json:"-"`
}

func (m *Cluster) Reset()                    { *m = Cluster{} }
func (m *Cluster) String() string            { return proto.CompactTextString(m) }
func (*Cluster) ProtoMessage()               {}
func (*Cluster) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{0} }

func (m *Cluster) GetId() uint64 {
	if m != nil && m.Id != nil {
		return *m.Id
	}
	return 0
}

func (m *Cluster) GetMaxPeerNumber() uint32 {
	if m != nil && m.MaxPeerNumber != nil {
		return *m.MaxPeerNumber
	}
	return 0
}

type Store struct {
	Id               *uint64 `protobuf:"varint,1,opt,name=id" json:"id,omitempty"`
	Address          *string `protobuf:"bytes,2,opt,name=address" json:"address,omitempty"`
	XXX_unrecognized []byte  `json:"-"`
}

func (m *Store) Reset()                    { *m = Store{} }
func (m *Store) String() string            { return proto.CompactTextString(m) }
func (*Store) ProtoMessage()               {}
func (*Store) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{1} }

func (m *Store) GetId() uint64 {
	if m != nil && m.Id != nil {
		return *m.Id
	}
	return 0
}

func (m *Store) GetAddress() string {
	if m != nil && m.Address != nil {
		return *m.Address
	}
	return ""
}

type RegionEpoch struct {
	// Conf change version, auto increment when add or remove peer
	ConfVer *uint64 `protobuf:"varint,1,opt,name=conf_ver,json=confVer" json:"conf_ver,omitempty"`
	// Region version, auto increment when split or merge
	Version          *uint64 `protobuf:"varint,2,opt,name=version" json:"version,omitempty"`
	XXX_unrecognized []byte  `json:"-"`
}

func (m *RegionEpoch) Reset()                    { *m = RegionEpoch{} }
func (m *RegionEpoch) String() string            { return proto.CompactTextString(m) }
func (*RegionEpoch) ProtoMessage()               {}
func (*RegionEpoch) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{2} }

func (m *RegionEpoch) GetConfVer() uint64 {
	if m != nil && m.ConfVer != nil {
		return *m.ConfVer
	}
	return 0
}

func (m *RegionEpoch) GetVersion() uint64 {
	if m != nil && m.Version != nil {
		return *m.Version
	}
	return 0
}

type Region struct {
	Id *uint64 `protobuf:"varint,1,opt,name=id" json:"id,omitempty"`
	// Region key range [start_key, end_key).
	StartKey         []byte       `protobuf:"bytes,2,opt,name=start_key,json=startKey" json:"start_key,omitempty"`
	EndKey           []byte       `protobuf:"bytes,3,opt,name=end_key,json=endKey" json:"end_key,omitempty"`
	RegionEpoch      *RegionEpoch `protobuf:"bytes,4,opt,name=region_epoch,json=regionEpoch" json:"region_epoch,omitempty"`
	StoreIds         []uint64     `protobuf:"varint,5,rep,name=store_ids,json=storeIds" json:"store_ids,omitempty"`
	XXX_unrecognized []byte       `json:"-"`
}

func (m *Region) Reset()                    { *m = Region{} }
func (m *Region) String() string            { return proto.CompactTextString(m) }
func (*Region) ProtoMessage()               {}
func (*Region) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{3} }

func (m *Region) GetId() uint64 {
	if m != nil && m.Id != nil {
		return *m.Id
	}
	return 0
}

func (m *Region) GetStartKey() []byte {
	if m != nil {
		return m.StartKey
	}
	return nil
}

func (m *Region) GetEndKey() []byte {
	if m != nil {
		return m.EndKey
	}
	return nil
}

func (m *Region) GetRegionEpoch() *RegionEpoch {
	if m != nil {
		return m.RegionEpoch
	}
	return nil
}

func (m *Region) GetStoreIds() []uint64 {
	if m != nil {
		return m.StoreIds
	}
	return nil
}

func init() {
	proto.RegisterType((*Cluster)(nil), "metapb.Cluster")
	proto.RegisterType((*Store)(nil), "metapb.Store")
	proto.RegisterType((*RegionEpoch)(nil), "metapb.RegionEpoch")
	proto.RegisterType((*Region)(nil), "metapb.Region")
}

var fileDescriptor0 = []byte{
	// 264 bytes of a gzipped FileDescriptorProto
	0x1f, 0x8b, 0x08, 0x00, 0x00, 0x09, 0x6e, 0x88, 0x02, 0xff, 0x64, 0x90, 0x4f, 0x4b, 0xc4, 0x30,
	0x10, 0xc5, 0xe9, 0xfe, 0x69, 0xbb, 0xd3, 0xae, 0x42, 0x3c, 0x18, 0xf1, 0xb2, 0xf4, 0x20, 0x9e,
	0x16, 0xf4, 0xe0, 0x5d, 0xc5, 0x83, 0x08, 0x22, 0x11, 0xbc, 0x86, 0xee, 0x66, 0xd4, 0xa2, 0x6d,
	0x4a, 0x92, 0x15, 0xf7, 0xf3, 0xf8, 0x45, 0x9d, 0x4e, 0x77, 0x61, 0xa1, 0xb7, 0xbc, 0xf7, 0xcb,
	0xbc, 0x79, 0x09, 0xe4, 0x35, 0x86, 0xb2, 0x5d, 0x2d, 0x5b, 0x67, 0x83, 0x15, 0x71, 0xaf, 0x8a,
	0x5b, 0x48, 0xee, 0xbf, 0x37, 0x3e, 0xa0, 0x13, 0x47, 0x30, 0xaa, 0x8c, 0x8c, 0x16, 0xd1, 0xe5,
	0x44, 0xd1, 0x49, 0x5c, 0xc0, 0x71, 0x5d, 0xfe, 0xea, 0x16, 0xd1, 0xe9, 0x66, 0x53, 0xaf, 0xd0,
	0xc9, 0x11, 0xc1, 0xb9, 0x9a, 0x93, 0xfd, 0x42, 0xee, 0x33, 0x9b, 0xc5, 0x15, 0x4c, 0x5f, 0x83,
	0x75, 0x38, 0x08, 0x90, 0x90, 0x94, 0xc6, 0x38, 0xf4, 0x9e, 0x07, 0x67, 0x6a, 0x2f, 0x8b, 0x3b,
	0xc8, 0x14, 0x7e, 0x54, 0xb6, 0x79, 0x68, 0xed, 0xfa, 0x53, 0x9c, 0x41, 0xba, 0xb6, 0xcd, 0xbb,
	0xfe, 0xa1, 0x15, 0xfd, 0x78, 0xd2, 0xe9, 0x37, 0x2a, 0x45, 0x19, 0xe4, 0x7a, 0xba, 0xca, 0x19,
	0x44, 0x76, 0xb2, 0xf8, 0x8b, 0x20, 0xee, 0x43, 0x06, 0x8b, 0xcf, 0x61, 0xe6, 0x43, 0xe9, 0x82,
	0xfe, 0xc2, 0x2d, 0x8f, 0xe5, 0x2a, 0x65, 0xe3, 0x09, 0xb7, 0xe2, 0x14, 0x12, 0x6c, 0x0c, 0xa3,
	0x31, 0xa3, 0x98, 0x64, 0x07, 0x6e, 0x20, 0x77, 0x9c, 0xa7, 0xb1, 0x6b, 0x25, 0x27, 0x44, 0xb3,
	0xeb, 0x93, 0xe5, 0xee, 0xdf, 0x0e, 0x0a, 0xab, 0xcc, 0x1d, 0xb4, 0xe7, 0x6d, 0xf4, 0x7e, 0x5d,
	0x19, 0x2f, 0xa7, 0x8b, 0x31, 0x95, 0x48, 0xd9, 0x78, 0x34, 0xfe, 0x3f, 0x00, 0x00, 0xff, 0xff,
	0x46, 0xef, 0xf1, 0xdf, 0x76, 0x01, 0x00, 0x00,
}
