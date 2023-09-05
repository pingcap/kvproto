// Code generated by protoc-gen-gogo. DO NOT EDIT.
// source: autoid.proto

package autoid

import (
	"context"
	"fmt"
	"io"
	"math"
	math_bits "math/bits"

	_ "github.com/gogo/protobuf/gogoproto"
	proto "github.com/golang/protobuf/proto"
	grpc "google.golang.org/grpc"
	codes "google.golang.org/grpc/codes"
	status "google.golang.org/grpc/status"
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

type AutoIDRequest struct {
	DbID                 int64    `protobuf:"varint,1,opt,name=dbID,proto3" json:"dbID,omitempty"`
	TblID                int64    `protobuf:"varint,2,opt,name=tblID,proto3" json:"tblID,omitempty"`
	IsUnsigned           bool     `protobuf:"varint,3,opt,name=isUnsigned,proto3" json:"isUnsigned,omitempty"`
	N                    uint64   `protobuf:"varint,4,opt,name=n,proto3" json:"n,omitempty"`
	Increment            int64    `protobuf:"varint,5,opt,name=increment,proto3" json:"increment,omitempty"`
	Offset               int64    `protobuf:"varint,6,opt,name=offset,proto3" json:"offset,omitempty"`
	KeyspaceID           uint32   `protobuf:"varint,7,opt,name=keyspaceID,proto3" json:"keyspaceID,omitempty"`
	XXX_NoUnkeyedLiteral struct{} `json:"-"`
	XXX_unrecognized     []byte   `json:"-"`
	XXX_sizecache        int32    `json:"-"`
}

func (m *AutoIDRequest) Reset()         { *m = AutoIDRequest{} }
func (m *AutoIDRequest) String() string { return proto.CompactTextString(m) }
func (*AutoIDRequest) ProtoMessage()    {}
func (*AutoIDRequest) Descriptor() ([]byte, []int) {
	return fileDescriptor_d81e5bb779eac45f, []int{0}
}
func (m *AutoIDRequest) XXX_Unmarshal(b []byte) error {
	return m.Unmarshal(b)
}
func (m *AutoIDRequest) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	if deterministic {
		return xxx_messageInfo_AutoIDRequest.Marshal(b, m, deterministic)
	} else {
		b = b[:cap(b)]
		n, err := m.MarshalToSizedBuffer(b)
		if err != nil {
			return nil, err
		}
		return b[:n], nil
	}
}
func (m *AutoIDRequest) XXX_Merge(src proto.Message) {
	xxx_messageInfo_AutoIDRequest.Merge(m, src)
}
func (m *AutoIDRequest) XXX_Size() int {
	return m.Size()
}
func (m *AutoIDRequest) XXX_DiscardUnknown() {
	xxx_messageInfo_AutoIDRequest.DiscardUnknown(m)
}

var xxx_messageInfo_AutoIDRequest proto.InternalMessageInfo

func (m *AutoIDRequest) GetDbID() int64 {
	if m != nil {
		return m.DbID
	}
	return 0
}

func (m *AutoIDRequest) GetTblID() int64 {
	if m != nil {
		return m.TblID
	}
	return 0
}

func (m *AutoIDRequest) GetIsUnsigned() bool {
	if m != nil {
		return m.IsUnsigned
	}
	return false
}

func (m *AutoIDRequest) GetN() uint64 {
	if m != nil {
		return m.N
	}
	return 0
}

func (m *AutoIDRequest) GetIncrement() int64 {
	if m != nil {
		return m.Increment
	}
	return 0
}

func (m *AutoIDRequest) GetOffset() int64 {
	if m != nil {
		return m.Offset
	}
	return 0
}

func (m *AutoIDRequest) GetKeyspaceID() uint32 {
	if m != nil {
		return m.KeyspaceID
	}
	return 0
}

type AutoIDResponse struct {
	Min                  int64    `protobuf:"varint,1,opt,name=min,proto3" json:"min,omitempty"`
	Max                  int64    `protobuf:"varint,2,opt,name=max,proto3" json:"max,omitempty"`
	Errmsg               []byte   `protobuf:"bytes,3,opt,name=errmsg,proto3" json:"errmsg,omitempty"`
	XXX_NoUnkeyedLiteral struct{} `json:"-"`
	XXX_unrecognized     []byte   `json:"-"`
	XXX_sizecache        int32    `json:"-"`
}

func (m *AutoIDResponse) Reset()         { *m = AutoIDResponse{} }
func (m *AutoIDResponse) String() string { return proto.CompactTextString(m) }
func (*AutoIDResponse) ProtoMessage()    {}
func (*AutoIDResponse) Descriptor() ([]byte, []int) {
	return fileDescriptor_d81e5bb779eac45f, []int{1}
}
func (m *AutoIDResponse) XXX_Unmarshal(b []byte) error {
	return m.Unmarshal(b)
}
func (m *AutoIDResponse) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	if deterministic {
		return xxx_messageInfo_AutoIDResponse.Marshal(b, m, deterministic)
	} else {
		b = b[:cap(b)]
		n, err := m.MarshalToSizedBuffer(b)
		if err != nil {
			return nil, err
		}
		return b[:n], nil
	}
}
func (m *AutoIDResponse) XXX_Merge(src proto.Message) {
	xxx_messageInfo_AutoIDResponse.Merge(m, src)
}
func (m *AutoIDResponse) XXX_Size() int {
	return m.Size()
}
func (m *AutoIDResponse) XXX_DiscardUnknown() {
	xxx_messageInfo_AutoIDResponse.DiscardUnknown(m)
}

var xxx_messageInfo_AutoIDResponse proto.InternalMessageInfo

func (m *AutoIDResponse) GetMin() int64 {
	if m != nil {
		return m.Min
	}
	return 0
}

func (m *AutoIDResponse) GetMax() int64 {
	if m != nil {
		return m.Max
	}
	return 0
}

func (m *AutoIDResponse) GetErrmsg() []byte {
	if m != nil {
		return m.Errmsg
	}
	return nil
}

type RebaseRequest struct {
	DbID                 int64    `protobuf:"varint,1,opt,name=dbID,proto3" json:"dbID,omitempty"`
	TblID                int64    `protobuf:"varint,2,opt,name=tblID,proto3" json:"tblID,omitempty"`
	IsUnsigned           bool     `protobuf:"varint,3,opt,name=isUnsigned,proto3" json:"isUnsigned,omitempty"`
	Base                 int64    `protobuf:"varint,4,opt,name=base,proto3" json:"base,omitempty"`
	Force                bool     `protobuf:"varint,5,opt,name=force,proto3" json:"force,omitempty"`
	XXX_NoUnkeyedLiteral struct{} `json:"-"`
	XXX_unrecognized     []byte   `json:"-"`
	XXX_sizecache        int32    `json:"-"`
}

func (m *RebaseRequest) Reset()         { *m = RebaseRequest{} }
func (m *RebaseRequest) String() string { return proto.CompactTextString(m) }
func (*RebaseRequest) ProtoMessage()    {}
func (*RebaseRequest) Descriptor() ([]byte, []int) {
	return fileDescriptor_d81e5bb779eac45f, []int{2}
}
func (m *RebaseRequest) XXX_Unmarshal(b []byte) error {
	return m.Unmarshal(b)
}
func (m *RebaseRequest) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	if deterministic {
		return xxx_messageInfo_RebaseRequest.Marshal(b, m, deterministic)
	} else {
		b = b[:cap(b)]
		n, err := m.MarshalToSizedBuffer(b)
		if err != nil {
			return nil, err
		}
		return b[:n], nil
	}
}
func (m *RebaseRequest) XXX_Merge(src proto.Message) {
	xxx_messageInfo_RebaseRequest.Merge(m, src)
}
func (m *RebaseRequest) XXX_Size() int {
	return m.Size()
}
func (m *RebaseRequest) XXX_DiscardUnknown() {
	xxx_messageInfo_RebaseRequest.DiscardUnknown(m)
}

var xxx_messageInfo_RebaseRequest proto.InternalMessageInfo

func (m *RebaseRequest) GetDbID() int64 {
	if m != nil {
		return m.DbID
	}
	return 0
}

func (m *RebaseRequest) GetTblID() int64 {
	if m != nil {
		return m.TblID
	}
	return 0
}

func (m *RebaseRequest) GetIsUnsigned() bool {
	if m != nil {
		return m.IsUnsigned
	}
	return false
}

func (m *RebaseRequest) GetBase() int64 {
	if m != nil {
		return m.Base
	}
	return 0
}

func (m *RebaseRequest) GetForce() bool {
	if m != nil {
		return m.Force
	}
	return false
}

type RebaseResponse struct {
	Errmsg               []byte   `protobuf:"bytes,1,opt,name=errmsg,proto3" json:"errmsg,omitempty"`
	XXX_NoUnkeyedLiteral struct{} `json:"-"`
	XXX_unrecognized     []byte   `json:"-"`
	XXX_sizecache        int32    `json:"-"`
}

func (m *RebaseResponse) Reset()         { *m = RebaseResponse{} }
func (m *RebaseResponse) String() string { return proto.CompactTextString(m) }
func (*RebaseResponse) ProtoMessage()    {}
func (*RebaseResponse) Descriptor() ([]byte, []int) {
	return fileDescriptor_d81e5bb779eac45f, []int{3}
}
func (m *RebaseResponse) XXX_Unmarshal(b []byte) error {
	return m.Unmarshal(b)
}
func (m *RebaseResponse) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	if deterministic {
		return xxx_messageInfo_RebaseResponse.Marshal(b, m, deterministic)
	} else {
		b = b[:cap(b)]
		n, err := m.MarshalToSizedBuffer(b)
		if err != nil {
			return nil, err
		}
		return b[:n], nil
	}
}
func (m *RebaseResponse) XXX_Merge(src proto.Message) {
	xxx_messageInfo_RebaseResponse.Merge(m, src)
}
func (m *RebaseResponse) XXX_Size() int {
	return m.Size()
}
func (m *RebaseResponse) XXX_DiscardUnknown() {
	xxx_messageInfo_RebaseResponse.DiscardUnknown(m)
}

var xxx_messageInfo_RebaseResponse proto.InternalMessageInfo

func (m *RebaseResponse) GetErrmsg() []byte {
	if m != nil {
		return m.Errmsg
	}
	return nil
}

func init() {
	proto.RegisterType((*AutoIDRequest)(nil), "autoid.AutoIDRequest")
	proto.RegisterType((*AutoIDResponse)(nil), "autoid.AutoIDResponse")
	proto.RegisterType((*RebaseRequest)(nil), "autoid.RebaseRequest")
	proto.RegisterType((*RebaseResponse)(nil), "autoid.RebaseResponse")
}

func init() { proto.RegisterFile("autoid.proto", fileDescriptor_d81e5bb779eac45f) }

var fileDescriptor_d81e5bb779eac45f = []byte{
	// 384 bytes of a gzipped FileDescriptorProto
	0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0xff, 0xac, 0x92, 0xbf, 0x4e, 0xf3, 0x30,
	0x14, 0xc5, 0xeb, 0x2f, 0x69, 0xbe, 0x72, 0xfb, 0x87, 0xca, 0x2a, 0x55, 0x54, 0xa1, 0x28, 0xca,
	0x80, 0x32, 0x05, 0x09, 0x26, 0x16, 0xa4, 0xa2, 0x2c, 0x91, 0x98, 0x2c, 0xf1, 0x00, 0x49, 0xea,
	0x46, 0x51, 0x5b, 0xbb, 0xc4, 0x4e, 0x05, 0x33, 0x0b, 0x8f, 0xc0, 0x23, 0xf0, 0x12, 0xec, 0x8c,
	0x8c, 0x8c, 0xa8, 0xbc, 0x08, 0x8a, 0x9d, 0x2a, 0x2d, 0x33, 0x53, 0xce, 0xf9, 0x59, 0xbe, 0xf7,
	0xf8, 0xe6, 0x42, 0x2f, 0x2e, 0x25, 0xcf, 0x67, 0xc1, 0xba, 0xe0, 0x92, 0x63, 0x4b, 0xbb, 0xc9,
	0x28, 0xe3, 0x19, 0x57, 0xe8, 0xbc, 0x52, 0xfa, 0x74, 0x72, 0x5c, 0x94, 0x42, 0x2a, 0xa9, 0x81,
	0xf7, 0x86, 0xa0, 0x3f, 0x2d, 0x25, 0x8f, 0x42, 0x42, 0xef, 0x4b, 0x2a, 0x24, 0xc6, 0x60, 0xce,
	0x92, 0x28, 0xb4, 0x91, 0x8b, 0x7c, 0x83, 0x28, 0x8d, 0x47, 0xd0, 0x96, 0xc9, 0x32, 0x0a, 0xed,
	0x7f, 0x0a, 0x6a, 0x83, 0x1d, 0x80, 0x5c, 0xdc, 0x31, 0x91, 0x67, 0x8c, 0xce, 0x6c, 0xc3, 0x45,
	0x7e, 0x87, 0xec, 0x11, 0xdc, 0x03, 0xc4, 0x6c, 0xd3, 0x45, 0xbe, 0x49, 0x10, 0xc3, 0xa7, 0x70,
	0x94, 0xb3, 0xb4, 0xa0, 0x2b, 0xca, 0xa4, 0xdd, 0x56, 0x75, 0x1a, 0x80, 0xc7, 0x60, 0xf1, 0xf9,
	0x5c, 0x50, 0x69, 0x5b, 0xea, 0xa8, 0x76, 0x55, 0x8f, 0x05, 0x7d, 0x14, 0xeb, 0x38, 0xa5, 0x51,
	0x68, 0xff, 0x77, 0x91, 0xdf, 0x27, 0x7b, 0xc4, 0xbb, 0x85, 0xc1, 0x2e, 0xbe, 0x58, 0x73, 0x26,
	0x28, 0x1e, 0x82, 0xb1, 0xca, 0x59, 0x1d, 0xbf, 0x92, 0x8a, 0xc4, 0x0f, 0x75, 0xf6, 0x4a, 0x56,
	0xdd, 0x68, 0x51, 0xac, 0x44, 0xa6, 0x52, 0xf7, 0x48, 0xed, 0xbc, 0x27, 0x04, 0x7d, 0x42, 0x93,
	0x58, 0xd0, 0xbf, 0x9f, 0x06, 0x06, 0xb3, 0x2a, 0xac, 0x06, 0x62, 0x10, 0xa5, 0xab, 0x4a, 0x73,
	0x5e, 0xa4, 0x54, 0xcd, 0xa3, 0x43, 0xb4, 0xf1, 0x7c, 0x18, 0xec, 0x42, 0xd4, 0x6f, 0x6a, 0xf2,
	0xa2, 0xfd, 0xbc, 0x17, 0xcf, 0x08, 0xba, 0xfa, 0xf9, 0xd3, 0xe5, 0x92, 0xa7, 0xf8, 0x1a, 0xba,
	0x4a, 0x68, 0x86, 0x4f, 0x82, 0x7a, 0x35, 0x0e, 0xfe, 0xf0, 0x64, 0xfc, 0x1b, 0xeb, 0x2e, 0x5e,
	0x0b, 0x5f, 0x81, 0xa5, 0x3b, 0x37, 0x57, 0x0f, 0xc6, 0xd1, 0x5c, 0x3d, 0x0c, 0xe8, 0xb5, 0x6e,
	0xce, 0x3e, 0x5f, 0x3b, 0xe8, 0x7d, 0xeb, 0xa0, 0x8f, 0xad, 0x83, 0xbe, 0xb6, 0x0e, 0x7a, 0xf9,
	0x76, 0x5a, 0x30, 0xe4, 0x45, 0x16, 0xc8, 0x7c, 0xb1, 0x09, 0x16, 0x1b, 0xb5, 0x70, 0x89, 0xa5,
	0x3e, 0x97, 0x3f, 0x01, 0x00, 0x00, 0xff, 0xff, 0x86, 0xe8, 0x66, 0x90, 0xb6, 0x02, 0x00, 0x00,
}

// Reference imports to suppress errors if they are not otherwise used.
var _ context.Context
var _ grpc.ClientConn

// This is a compile-time assertion to ensure that this generated file
// is compatible with the grpc package it is being compiled against.
const _ = grpc.SupportPackageIsVersion4

// AutoIDAllocClient is the client API for AutoIDAlloc service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://godoc.org/google.golang.org/grpc#ClientConn.NewStream.
type AutoIDAllocClient interface {
	AllocAutoID(ctx context.Context, in *AutoIDRequest, opts ...grpc.CallOption) (*AutoIDResponse, error)
	Rebase(ctx context.Context, in *RebaseRequest, opts ...grpc.CallOption) (*RebaseResponse, error)
}

type autoIDAllocClient struct {
	cc *grpc.ClientConn
}

func NewAutoIDAllocClient(cc *grpc.ClientConn) AutoIDAllocClient {
	return &autoIDAllocClient{cc}
}

func (c *autoIDAllocClient) AllocAutoID(ctx context.Context, in *AutoIDRequest, opts ...grpc.CallOption) (*AutoIDResponse, error) {
	out := new(AutoIDResponse)
	err := c.cc.Invoke(ctx, "/autoid.AutoIDAlloc/AllocAutoID", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *autoIDAllocClient) Rebase(ctx context.Context, in *RebaseRequest, opts ...grpc.CallOption) (*RebaseResponse, error) {
	out := new(RebaseResponse)
	err := c.cc.Invoke(ctx, "/autoid.AutoIDAlloc/Rebase", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

// AutoIDAllocServer is the server API for AutoIDAlloc service.
type AutoIDAllocServer interface {
	AllocAutoID(context.Context, *AutoIDRequest) (*AutoIDResponse, error)
	Rebase(context.Context, *RebaseRequest) (*RebaseResponse, error)
}

// UnimplementedAutoIDAllocServer can be embedded to have forward compatible implementations.
type UnimplementedAutoIDAllocServer struct {
}

func (*UnimplementedAutoIDAllocServer) AllocAutoID(ctx context.Context, req *AutoIDRequest) (*AutoIDResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method AllocAutoID not implemented")
}
func (*UnimplementedAutoIDAllocServer) Rebase(ctx context.Context, req *RebaseRequest) (*RebaseResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Rebase not implemented")
}

func RegisterAutoIDAllocServer(s *grpc.Server, srv AutoIDAllocServer) {
	s.RegisterService(&_AutoIDAlloc_serviceDesc, srv)
}

func _AutoIDAlloc_AllocAutoID_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(AutoIDRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(AutoIDAllocServer).AllocAutoID(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/autoid.AutoIDAlloc/AllocAutoID",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(AutoIDAllocServer).AllocAutoID(ctx, req.(*AutoIDRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _AutoIDAlloc_Rebase_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(RebaseRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(AutoIDAllocServer).Rebase(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/autoid.AutoIDAlloc/Rebase",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(AutoIDAllocServer).Rebase(ctx, req.(*RebaseRequest))
	}
	return interceptor(ctx, in, info, handler)
}

var _AutoIDAlloc_serviceDesc = grpc.ServiceDesc{
	ServiceName: "autoid.AutoIDAlloc",
	HandlerType: (*AutoIDAllocServer)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "AllocAutoID",
			Handler:    _AutoIDAlloc_AllocAutoID_Handler,
		},
		{
			MethodName: "Rebase",
			Handler:    _AutoIDAlloc_Rebase_Handler,
		},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "autoid.proto",
}

func (m *AutoIDRequest) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalToSizedBuffer(dAtA[:size])
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *AutoIDRequest) MarshalTo(dAtA []byte) (int, error) {
	size := m.Size()
	return m.MarshalToSizedBuffer(dAtA[:size])
}

func (m *AutoIDRequest) MarshalToSizedBuffer(dAtA []byte) (int, error) {
	i := len(dAtA)
	_ = i
	var l int
	_ = l
	if m.XXX_unrecognized != nil {
		i -= len(m.XXX_unrecognized)
		copy(dAtA[i:], m.XXX_unrecognized)
	}
	if m.KeyspaceID != 0 {
		i = encodeVarintAutoid(dAtA, i, uint64(m.KeyspaceID))
		i--
		dAtA[i] = 0x38
	}
	if m.Offset != 0 {
		i = encodeVarintAutoid(dAtA, i, uint64(m.Offset))
		i--
		dAtA[i] = 0x30
	}
	if m.Increment != 0 {
		i = encodeVarintAutoid(dAtA, i, uint64(m.Increment))
		i--
		dAtA[i] = 0x28
	}
	if m.N != 0 {
		i = encodeVarintAutoid(dAtA, i, uint64(m.N))
		i--
		dAtA[i] = 0x20
	}
	if m.IsUnsigned {
		i--
		if m.IsUnsigned {
			dAtA[i] = 1
		} else {
			dAtA[i] = 0
		}
		i--
		dAtA[i] = 0x18
	}
	if m.TblID != 0 {
		i = encodeVarintAutoid(dAtA, i, uint64(m.TblID))
		i--
		dAtA[i] = 0x10
	}
	if m.DbID != 0 {
		i = encodeVarintAutoid(dAtA, i, uint64(m.DbID))
		i--
		dAtA[i] = 0x8
	}
	return len(dAtA) - i, nil
}

func (m *AutoIDResponse) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalToSizedBuffer(dAtA[:size])
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *AutoIDResponse) MarshalTo(dAtA []byte) (int, error) {
	size := m.Size()
	return m.MarshalToSizedBuffer(dAtA[:size])
}

func (m *AutoIDResponse) MarshalToSizedBuffer(dAtA []byte) (int, error) {
	i := len(dAtA)
	_ = i
	var l int
	_ = l
	if m.XXX_unrecognized != nil {
		i -= len(m.XXX_unrecognized)
		copy(dAtA[i:], m.XXX_unrecognized)
	}
	if len(m.Errmsg) > 0 {
		i -= len(m.Errmsg)
		copy(dAtA[i:], m.Errmsg)
		i = encodeVarintAutoid(dAtA, i, uint64(len(m.Errmsg)))
		i--
		dAtA[i] = 0x1a
	}
	if m.Max != 0 {
		i = encodeVarintAutoid(dAtA, i, uint64(m.Max))
		i--
		dAtA[i] = 0x10
	}
	if m.Min != 0 {
		i = encodeVarintAutoid(dAtA, i, uint64(m.Min))
		i--
		dAtA[i] = 0x8
	}
	return len(dAtA) - i, nil
}

func (m *RebaseRequest) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalToSizedBuffer(dAtA[:size])
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *RebaseRequest) MarshalTo(dAtA []byte) (int, error) {
	size := m.Size()
	return m.MarshalToSizedBuffer(dAtA[:size])
}

func (m *RebaseRequest) MarshalToSizedBuffer(dAtA []byte) (int, error) {
	i := len(dAtA)
	_ = i
	var l int
	_ = l
	if m.XXX_unrecognized != nil {
		i -= len(m.XXX_unrecognized)
		copy(dAtA[i:], m.XXX_unrecognized)
	}
	if m.Force {
		i--
		if m.Force {
			dAtA[i] = 1
		} else {
			dAtA[i] = 0
		}
		i--
		dAtA[i] = 0x28
	}
	if m.Base != 0 {
		i = encodeVarintAutoid(dAtA, i, uint64(m.Base))
		i--
		dAtA[i] = 0x20
	}
	if m.IsUnsigned {
		i--
		if m.IsUnsigned {
			dAtA[i] = 1
		} else {
			dAtA[i] = 0
		}
		i--
		dAtA[i] = 0x18
	}
	if m.TblID != 0 {
		i = encodeVarintAutoid(dAtA, i, uint64(m.TblID))
		i--
		dAtA[i] = 0x10
	}
	if m.DbID != 0 {
		i = encodeVarintAutoid(dAtA, i, uint64(m.DbID))
		i--
		dAtA[i] = 0x8
	}
	return len(dAtA) - i, nil
}

func (m *RebaseResponse) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalToSizedBuffer(dAtA[:size])
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *RebaseResponse) MarshalTo(dAtA []byte) (int, error) {
	size := m.Size()
	return m.MarshalToSizedBuffer(dAtA[:size])
}

func (m *RebaseResponse) MarshalToSizedBuffer(dAtA []byte) (int, error) {
	i := len(dAtA)
	_ = i
	var l int
	_ = l
	if m.XXX_unrecognized != nil {
		i -= len(m.XXX_unrecognized)
		copy(dAtA[i:], m.XXX_unrecognized)
	}
	if len(m.Errmsg) > 0 {
		i -= len(m.Errmsg)
		copy(dAtA[i:], m.Errmsg)
		i = encodeVarintAutoid(dAtA, i, uint64(len(m.Errmsg)))
		i--
		dAtA[i] = 0xa
	}
	return len(dAtA) - i, nil
}

func encodeVarintAutoid(dAtA []byte, offset int, v uint64) int {
	offset -= sovAutoid(v)
	base := offset
	for v >= 1<<7 {
		dAtA[offset] = uint8(v&0x7f | 0x80)
		v >>= 7
		offset++
	}
	dAtA[offset] = uint8(v)
	return base
}
func (m *AutoIDRequest) Size() (n int) {
	if m == nil {
		return 0
	}
	var l int
	_ = l
	if m.DbID != 0 {
		n += 1 + sovAutoid(uint64(m.DbID))
	}
	if m.TblID != 0 {
		n += 1 + sovAutoid(uint64(m.TblID))
	}
	if m.IsUnsigned {
		n += 2
	}
	if m.N != 0 {
		n += 1 + sovAutoid(uint64(m.N))
	}
	if m.Increment != 0 {
		n += 1 + sovAutoid(uint64(m.Increment))
	}
	if m.Offset != 0 {
		n += 1 + sovAutoid(uint64(m.Offset))
	}
	if m.KeyspaceID != 0 {
		n += 1 + sovAutoid(uint64(m.KeyspaceID))
	}
	if m.XXX_unrecognized != nil {
		n += len(m.XXX_unrecognized)
	}
	return n
}

func (m *AutoIDResponse) Size() (n int) {
	if m == nil {
		return 0
	}
	var l int
	_ = l
	if m.Min != 0 {
		n += 1 + sovAutoid(uint64(m.Min))
	}
	if m.Max != 0 {
		n += 1 + sovAutoid(uint64(m.Max))
	}
	l = len(m.Errmsg)
	if l > 0 {
		n += 1 + l + sovAutoid(uint64(l))
	}
	if m.XXX_unrecognized != nil {
		n += len(m.XXX_unrecognized)
	}
	return n
}

func (m *RebaseRequest) Size() (n int) {
	if m == nil {
		return 0
	}
	var l int
	_ = l
	if m.DbID != 0 {
		n += 1 + sovAutoid(uint64(m.DbID))
	}
	if m.TblID != 0 {
		n += 1 + sovAutoid(uint64(m.TblID))
	}
	if m.IsUnsigned {
		n += 2
	}
	if m.Base != 0 {
		n += 1 + sovAutoid(uint64(m.Base))
	}
	if m.Force {
		n += 2
	}
	if m.XXX_unrecognized != nil {
		n += len(m.XXX_unrecognized)
	}
	return n
}

func (m *RebaseResponse) Size() (n int) {
	if m == nil {
		return 0
	}
	var l int
	_ = l
	l = len(m.Errmsg)
	if l > 0 {
		n += 1 + l + sovAutoid(uint64(l))
	}
	if m.XXX_unrecognized != nil {
		n += len(m.XXX_unrecognized)
	}
	return n
}

func sovAutoid(x uint64) (n int) {
	return (math_bits.Len64(x|1) + 6) / 7
}
func sozAutoid(x uint64) (n int) {
	return sovAutoid(uint64((x << 1) ^ uint64((int64(x) >> 63))))
}
func (m *AutoIDRequest) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowAutoid
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
			return fmt.Errorf("proto: AutoIDRequest: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: AutoIDRequest: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field DbID", wireType)
			}
			m.DbID = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowAutoid
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				m.DbID |= int64(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		case 2:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field TblID", wireType)
			}
			m.TblID = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowAutoid
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				m.TblID |= int64(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		case 3:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field IsUnsigned", wireType)
			}
			var v int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowAutoid
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				v |= int(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			m.IsUnsigned = bool(v != 0)
		case 4:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field N", wireType)
			}
			m.N = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowAutoid
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				m.N |= uint64(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		case 5:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field Increment", wireType)
			}
			m.Increment = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowAutoid
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				m.Increment |= int64(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		case 6:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field Offset", wireType)
			}
			m.Offset = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowAutoid
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				m.Offset |= int64(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		case 7:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field KeyspaceID", wireType)
			}
			m.KeyspaceID = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowAutoid
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				m.KeyspaceID |= uint32(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		default:
			iNdEx = preIndex
			skippy, err := skipAutoid(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if (skippy < 0) || (iNdEx+skippy) < 0 {
				return ErrInvalidLengthAutoid
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
func (m *AutoIDResponse) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowAutoid
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
			return fmt.Errorf("proto: AutoIDResponse: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: AutoIDResponse: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field Min", wireType)
			}
			m.Min = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowAutoid
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				m.Min |= int64(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		case 2:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field Max", wireType)
			}
			m.Max = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowAutoid
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				m.Max |= int64(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		case 3:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field Errmsg", wireType)
			}
			var byteLen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowAutoid
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				byteLen |= int(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if byteLen < 0 {
				return ErrInvalidLengthAutoid
			}
			postIndex := iNdEx + byteLen
			if postIndex < 0 {
				return ErrInvalidLengthAutoid
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.Errmsg = append(m.Errmsg[:0], dAtA[iNdEx:postIndex]...)
			if m.Errmsg == nil {
				m.Errmsg = []byte{}
			}
			iNdEx = postIndex
		default:
			iNdEx = preIndex
			skippy, err := skipAutoid(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if (skippy < 0) || (iNdEx+skippy) < 0 {
				return ErrInvalidLengthAutoid
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
func (m *RebaseRequest) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowAutoid
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
			return fmt.Errorf("proto: RebaseRequest: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: RebaseRequest: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field DbID", wireType)
			}
			m.DbID = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowAutoid
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				m.DbID |= int64(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		case 2:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field TblID", wireType)
			}
			m.TblID = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowAutoid
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				m.TblID |= int64(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		case 3:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field IsUnsigned", wireType)
			}
			var v int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowAutoid
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				v |= int(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			m.IsUnsigned = bool(v != 0)
		case 4:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field Base", wireType)
			}
			m.Base = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowAutoid
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				m.Base |= int64(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		case 5:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field Force", wireType)
			}
			var v int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowAutoid
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				v |= int(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			m.Force = bool(v != 0)
		default:
			iNdEx = preIndex
			skippy, err := skipAutoid(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if (skippy < 0) || (iNdEx+skippy) < 0 {
				return ErrInvalidLengthAutoid
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
func (m *RebaseResponse) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowAutoid
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
			return fmt.Errorf("proto: RebaseResponse: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: RebaseResponse: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field Errmsg", wireType)
			}
			var byteLen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowAutoid
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				byteLen |= int(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if byteLen < 0 {
				return ErrInvalidLengthAutoid
			}
			postIndex := iNdEx + byteLen
			if postIndex < 0 {
				return ErrInvalidLengthAutoid
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.Errmsg = append(m.Errmsg[:0], dAtA[iNdEx:postIndex]...)
			if m.Errmsg == nil {
				m.Errmsg = []byte{}
			}
			iNdEx = postIndex
		default:
			iNdEx = preIndex
			skippy, err := skipAutoid(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if (skippy < 0) || (iNdEx+skippy) < 0 {
				return ErrInvalidLengthAutoid
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
func skipAutoid(dAtA []byte) (n int, err error) {
	l := len(dAtA)
	iNdEx := 0
	depth := 0
	for iNdEx < l {
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return 0, ErrIntOverflowAutoid
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
					return 0, ErrIntOverflowAutoid
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
					return 0, ErrIntOverflowAutoid
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
				return 0, ErrInvalidLengthAutoid
			}
			iNdEx += length
		case 3:
			depth++
		case 4:
			if depth == 0 {
				return 0, ErrUnexpectedEndOfGroupAutoid
			}
			depth--
		case 5:
			iNdEx += 4
		default:
			return 0, fmt.Errorf("proto: illegal wireType %d", wireType)
		}
		if iNdEx < 0 {
			return 0, ErrInvalidLengthAutoid
		}
		if depth == 0 {
			return iNdEx, nil
		}
	}
	return 0, io.ErrUnexpectedEOF
}

var (
	ErrInvalidLengthAutoid        = fmt.Errorf("proto: negative length found during unmarshaling")
	ErrIntOverflowAutoid          = fmt.Errorf("proto: integer overflow")
	ErrUnexpectedEndOfGroupAutoid = fmt.Errorf("proto: unexpected end of group")
)
