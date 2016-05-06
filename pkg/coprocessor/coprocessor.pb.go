// Code generated by protoc-gen-go.
// source: coprocessor.proto
// DO NOT EDIT!

/*
Package coprocessor is a generated protocol buffer package.

It is generated from these files:
	coprocessor.proto

It has these top-level messages:
	KeyRange
	Request
	Response
*/
package coprocessor

import proto "github.com/golang/protobuf/proto"
import fmt "fmt"
import math "math"
import errorpb "github.com/pingcap/kvproto/pkg/errorpb"
import kvrpcpb "github.com/pingcap/kvproto/pkg/kvrpcpb"

// Reference imports to suppress errors if they are not otherwise used.
var _ = proto.Marshal
var _ = fmt.Errorf
var _ = math.Inf

// This is a compile-time assertion to ensure that this generated file
// is compatible with the proto package it is being compiled against.
const _ = proto.ProtoPackageIsVersion1

// [start, end)
type KeyRange struct {
	Start            []byte `protobuf:"bytes,1,opt,name=start" json:"start,omitempty"`
	End              []byte `protobuf:"bytes,2,opt,name=end" json:"end,omitempty"`
	XXX_unrecognized []byte `json:"-"`
}

func (m *KeyRange) Reset()                    { *m = KeyRange{} }
func (m *KeyRange) String() string            { return proto.CompactTextString(m) }
func (*KeyRange) ProtoMessage()               {}
func (*KeyRange) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{0} }

func (m *KeyRange) GetStart() []byte {
	if m != nil {
		return m.Start
	}
	return nil
}

func (m *KeyRange) GetEnd() []byte {
	if m != nil {
		return m.End
	}
	return nil
}

type Request struct {
	Context          *kvrpcpb.Context `protobuf:"bytes,1,opt,name=context" json:"context,omitempty"`
	Tp               *int64           `protobuf:"varint,2,opt,name=tp" json:"tp,omitempty"`
	Data             []byte           `protobuf:"bytes,3,opt,name=data" json:"data,omitempty"`
	Ranges           []*KeyRange      `protobuf:"bytes,4,rep,name=ranges" json:"ranges,omitempty"`
	XXX_unrecognized []byte           `json:"-"`
}

func (m *Request) Reset()                    { *m = Request{} }
func (m *Request) String() string            { return proto.CompactTextString(m) }
func (*Request) ProtoMessage()               {}
func (*Request) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{1} }

func (m *Request) GetContext() *kvrpcpb.Context {
	if m != nil {
		return m.Context
	}
	return nil
}

func (m *Request) GetTp() int64 {
	if m != nil && m.Tp != nil {
		return *m.Tp
	}
	return 0
}

func (m *Request) GetData() []byte {
	if m != nil {
		return m.Data
	}
	return nil
}

func (m *Request) GetRanges() []*KeyRange {
	if m != nil {
		return m.Ranges
	}
	return nil
}

type Response struct {
	Data             []byte            `protobuf:"bytes,1,opt,name=data" json:"data,omitempty"`
	RegionError      *errorpb.Error    `protobuf:"bytes,2,opt,name=region_error,json=regionError" json:"region_error,omitempty"`
	Locked           *kvrpcpb.LockInfo `protobuf:"bytes,3,opt,name=locked" json:"locked,omitempty"`
	OtherError       *string           `protobuf:"bytes,4,opt,name=other_error,json=otherError" json:"other_error,omitempty"`
	XXX_unrecognized []byte            `json:"-"`
}

func (m *Response) Reset()                    { *m = Response{} }
func (m *Response) String() string            { return proto.CompactTextString(m) }
func (*Response) ProtoMessage()               {}
func (*Response) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{2} }

func (m *Response) GetData() []byte {
	if m != nil {
		return m.Data
	}
	return nil
}

func (m *Response) GetRegionError() *errorpb.Error {
	if m != nil {
		return m.RegionError
	}
	return nil
}

func (m *Response) GetLocked() *kvrpcpb.LockInfo {
	if m != nil {
		return m.Locked
	}
	return nil
}

func (m *Response) GetOtherError() string {
	if m != nil && m.OtherError != nil {
		return *m.OtherError
	}
	return ""
}

func init() {
	proto.RegisterType((*KeyRange)(nil), "coprocessor.KeyRange")
	proto.RegisterType((*Request)(nil), "coprocessor.Request")
	proto.RegisterType((*Response)(nil), "coprocessor.Response")
}

var fileDescriptor0 = []byte{
	// 276 bytes of a gzipped FileDescriptorProto
	0x1f, 0x8b, 0x08, 0x00, 0x00, 0x09, 0x6e, 0x88, 0x02, 0xff, 0x4c, 0x90, 0xdf, 0x4e, 0xb3, 0x40,
	0x10, 0xc5, 0x43, 0xe1, 0x6b, 0xfb, 0xcd, 0xb6, 0x4d, 0xbb, 0xd1, 0x84, 0xf4, 0xc6, 0x86, 0x2b,
	0x35, 0x91, 0x44, 0x5e, 0xc1, 0x78, 0x61, 0xf4, 0x6a, 0x5f, 0xc0, 0x20, 0x8c, 0xd5, 0xd4, 0xec,
	0xe0, 0xec, 0x6a, 0xf4, 0x0d, 0x7c, 0x09, 0xdf, 0x55, 0x18, 0xd8, 0x86, 0x2b, 0xce, 0x1c, 0xce,
	0xfe, 0xe6, 0x0f, 0x6c, 0x2a, 0x6a, 0x98, 0x2a, 0x74, 0x8e, 0x38, 0x6f, 0x95, 0x27, 0xad, 0x46,
	0xd6, 0x76, 0x89, 0xcc, 0xc4, 0xcd, 0x53, 0xff, 0x6f, 0xbb, 0x3c, 0x7c, 0x72, 0x53, 0x85, 0x32,
	0x2b, 0x60, 0x7e, 0x8f, 0xdf, 0xa6, 0xb4, 0x7b, 0xd4, 0x27, 0xf0, 0xcf, 0xf9, 0x92, 0x7d, 0x1a,
	0xed, 0xa2, 0xf3, 0x85, 0xe9, 0x0b, 0xbd, 0x86, 0x18, 0x6d, 0x9d, 0x4e, 0xc4, 0xeb, 0x64, 0xf6,
	0x13, 0xc1, 0xcc, 0xe0, 0xfb, 0x07, 0x3a, 0xaf, 0x2f, 0x61, 0x56, 0x91, 0xf5, 0xf8, 0xd5, 0xbf,
	0x52, 0xc5, 0x3a, 0x0f, 0x0d, 0x6e, 0x7a, 0xdf, 0x84, 0x80, 0x5e, 0xc1, 0xc4, 0x37, 0x02, 0x8a,
	0x4d, 0xab, 0xb4, 0x86, 0xa4, 0x2e, 0x7d, 0x99, 0xc6, 0x82, 0x16, 0xad, 0xaf, 0x60, 0xca, 0xdd,
	0x30, 0x2e, 0x4d, 0x76, 0x71, 0x8b, 0x3b, 0xcd, 0xc7, 0xeb, 0x85, 0x51, 0xcd, 0x10, 0xca, 0x7e,
	0x23, 0x98, 0x1b, 0x74, 0x0d, 0x59, 0x87, 0x47, 0x5e, 0x34, 0xe2, 0x5d, 0xc3, 0x82, 0x71, 0xff,
	0x4a, 0xf6, 0x51, 0xce, 0x20, 0xdd, 0x55, 0xb1, 0xca, 0xc3, 0x51, 0x6e, 0xbb, 0xaf, 0x51, 0x7d,
	0x46, 0x0a, 0x7d, 0x01, 0xd3, 0x37, 0xaa, 0x0e, 0x58, 0xcb, 0x60, 0xaa, 0xd8, 0x1c, 0x37, 0x7a,
	0x68, 0xed, 0x3b, 0xfb, 0x4c, 0x66, 0x08, 0xe8, 0x33, 0x50, 0xe4, 0x5f, 0x90, 0x07, 0x78, 0xd2,
	0xe6, 0xff, 0x1b, 0x10, 0x4b, 0x58, 0x7f, 0x01, 0x00, 0x00, 0xff, 0xff, 0xd6, 0x93, 0x8e, 0xdd,
	0x9d, 0x01, 0x00, 0x00,
}
