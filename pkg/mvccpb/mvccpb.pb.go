// Code generated by protoc-gen-go.
// source: mvccpb.proto
// DO NOT EDIT!

/*
Package mvccpb is a generated protocol buffer package.

It is generated from these files:
	mvccpb.proto

It has these top-level messages:
	MetaItem
	MetaLock
	Meta
*/
package mvccpb

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

type MetaLockType int32

const (
	MetaLockType_ReadOnly  MetaLockType = 1
	MetaLockType_ReadWrite MetaLockType = 2
)

var MetaLockType_name = map[int32]string{
	1: "ReadOnly",
	2: "ReadWrite",
}
var MetaLockType_value = map[string]int32{
	"ReadOnly":  1,
	"ReadWrite": 2,
}

func (x MetaLockType) Enum() *MetaLockType {
	p := new(MetaLockType)
	*p = x
	return p
}
func (x MetaLockType) String() string {
	return proto.EnumName(MetaLockType_name, int32(x))
}
func (x *MetaLockType) UnmarshalJSON(data []byte) error {
	value, err := proto.UnmarshalJSONEnum(MetaLockType_value, data, "MetaLockType")
	if err != nil {
		return err
	}
	*x = MetaLockType(value)
	return nil
}
func (MetaLockType) EnumDescriptor() ([]byte, []int) { return fileDescriptor0, []int{0} }

type MetaItem struct {
	StartTs          *uint64 `protobuf:"varint,1,opt,name=start_ts,json=startTs" json:"start_ts,omitempty"`
	CommitTs         *uint64 `protobuf:"varint,2,opt,name=commit_ts,json=commitTs" json:"commit_ts,omitempty"`
	XXX_unrecognized []byte  `json:"-"`
}

func (m *MetaItem) Reset()                    { *m = MetaItem{} }
func (m *MetaItem) String() string            { return proto.CompactTextString(m) }
func (*MetaItem) ProtoMessage()               {}
func (*MetaItem) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{0} }

func (m *MetaItem) GetStartTs() uint64 {
	if m != nil && m.StartTs != nil {
		return *m.StartTs
	}
	return 0
}

func (m *MetaItem) GetCommitTs() uint64 {
	if m != nil && m.CommitTs != nil {
		return *m.CommitTs
	}
	return 0
}

type MetaLock struct {
	Type             *MetaLockType `protobuf:"varint,1,opt,name=type,enum=mvccpb.MetaLockType" json:"type,omitempty"`
	StartTs          *uint64       `protobuf:"varint,2,opt,name=start_ts,json=startTs" json:"start_ts,omitempty"`
	PrimaryKey       []byte        `protobuf:"bytes,3,opt,name=primary_key,json=primaryKey" json:"primary_key,omitempty"`
	XXX_unrecognized []byte        `json:"-"`
}

func (m *MetaLock) Reset()                    { *m = MetaLock{} }
func (m *MetaLock) String() string            { return proto.CompactTextString(m) }
func (*MetaLock) ProtoMessage()               {}
func (*MetaLock) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{1} }

func (m *MetaLock) GetType() MetaLockType {
	if m != nil && m.Type != nil {
		return *m.Type
	}
	return MetaLockType_ReadOnly
}

func (m *MetaLock) GetStartTs() uint64 {
	if m != nil && m.StartTs != nil {
		return *m.StartTs
	}
	return 0
}

func (m *MetaLock) GetPrimaryKey() []byte {
	if m != nil {
		return m.PrimaryKey
	}
	return nil
}

// Meta will be splitted into a list if it gets too big.
// An auto increased index will be assigned to each Meta node. The field `next`
// stores next Meta's index and next is 0 if no more Meta exists.
// Meta0 always contains the Lock(if any) and the latest versions.
type Meta struct {
	Lock             *MetaLock   `protobuf:"bytes,1,opt,name=lock" json:"lock,omitempty"`
	Items            []*MetaItem `protobuf:"bytes,2,rep,name=items" json:"items,omitempty"`
	Next             *uint64     `protobuf:"varint,3,opt,name=next" json:"next,omitempty"`
	XXX_unrecognized []byte      `json:"-"`
}

func (m *Meta) Reset()                    { *m = Meta{} }
func (m *Meta) String() string            { return proto.CompactTextString(m) }
func (*Meta) ProtoMessage()               {}
func (*Meta) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{2} }

func (m *Meta) GetLock() *MetaLock {
	if m != nil {
		return m.Lock
	}
	return nil
}

func (m *Meta) GetItems() []*MetaItem {
	if m != nil {
		return m.Items
	}
	return nil
}

func (m *Meta) GetNext() uint64 {
	if m != nil && m.Next != nil {
		return *m.Next
	}
	return 0
}

func init() {
	proto.RegisterType((*MetaItem)(nil), "mvccpb.MetaItem")
	proto.RegisterType((*MetaLock)(nil), "mvccpb.MetaLock")
	proto.RegisterType((*Meta)(nil), "mvccpb.Meta")
	proto.RegisterEnum("mvccpb.MetaLockType", MetaLockType_name, MetaLockType_value)
}

var fileDescriptor0 = []byte{
	// 241 bytes of a gzipped FileDescriptorProto
	0x1f, 0x8b, 0x08, 0x00, 0x00, 0x09, 0x6e, 0x88, 0x02, 0xff, 0x5c, 0x8f, 0x51, 0x4b, 0xc3, 0x30,
	0x10, 0xc7, 0x69, 0x17, 0xb5, 0xbb, 0x55, 0x29, 0x87, 0x0f, 0x13, 0x1f, 0x94, 0x22, 0x32, 0x14,
	0xf6, 0xb0, 0x8f, 0xe0, 0x9b, 0xa8, 0x08, 0xa1, 0xe0, 0xe3, 0xa8, 0x35, 0x60, 0xd9, 0xb2, 0x86,
	0xe4, 0x10, 0xf3, 0xed, 0x4d, 0x2e, 0x13, 0xb6, 0xbe, 0xf5, 0x7f, 0xbf, 0xeb, 0xff, 0x97, 0x83,
	0x52, 0xff, 0x74, 0x9d, 0xf9, 0x5c, 0x1a, 0x3b, 0xd0, 0x80, 0xa7, 0x29, 0xd5, 0x4f, 0x50, 0xbc,
	0x29, 0x6a, 0x9f, 0x49, 0x69, 0xbc, 0x82, 0xc2, 0x51, 0x6b, 0x69, 0x4d, 0x6e, 0x9e, 0xdd, 0x66,
	0x0b, 0x21, 0xcf, 0x38, 0x37, 0x0e, 0xaf, 0x61, 0xda, 0x0d, 0x5a, 0xf7, 0xcc, 0x72, 0x66, 0x45,
	0x1a, 0x34, 0xae, 0x36, 0xa9, 0xe3, 0x75, 0xe8, 0x36, 0xb8, 0x00, 0x41, 0xde, 0x28, 0xfe, 0xff,
	0x62, 0x75, 0xb9, 0xdc, 0x4b, 0xff, 0x79, 0x13, 0x98, 0xe4, 0x8d, 0x23, 0x5b, 0x7e, 0x6c, 0xbb,
	0x81, 0x99, 0xb1, 0xbd, 0x6e, 0xad, 0x5f, 0x6f, 0x94, 0x9f, 0x4f, 0x02, 0x2d, 0x25, 0xec, 0x47,
	0x2f, 0xca, 0xd7, 0xdf, 0x20, 0x62, 0x23, 0xde, 0x81, 0xd8, 0x86, 0x56, 0xb6, 0xcd, 0x56, 0xd5,
	0xd8, 0x26, 0x99, 0xe2, 0x3d, 0x9c, 0xf4, 0xe1, 0xbe, 0xa8, 0x99, 0x8c, 0xd7, 0xe2, 0xe1, 0x32,
	0x61, 0x44, 0x10, 0x3b, 0xf5, 0x4b, 0xec, 0x13, 0x92, 0xbf, 0x1f, 0x1e, 0xa1, 0x3c, 0x7c, 0x3b,
	0x96, 0x50, 0x48, 0xd5, 0x7e, 0xbd, 0xef, 0xb6, 0xbe, 0xca, 0xf0, 0x1c, 0xa6, 0x31, 0x7d, 0xd8,
	0x50, 0x50, 0xe5, 0x7f, 0x01, 0x00, 0x00, 0xff, 0xff, 0x89, 0xe4, 0xe2, 0x77, 0x63, 0x01, 0x00,
	0x00,
}
