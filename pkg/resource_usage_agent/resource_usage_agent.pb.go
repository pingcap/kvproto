// Code generated by protoc-gen-gogo. DO NOT EDIT.
// source: resource_usage_agent.proto

package resource_usage_agent

import (
	"fmt"
	"io"
	"math"

	proto "github.com/golang/protobuf/proto"

	_ "github.com/gogo/protobuf/gogoproto"

	context "golang.org/x/net/context"

	grpc "google.golang.org/grpc"
)

// Reference imports to suppress errors if they are not otherwise used.
var _ = proto.Marshal
var _ = fmt.Errorf
var _ = math.Inf

// This is a compile-time assertion to ensure that this generated file
// is compatible with the proto package it is being compiled against.
// A compilation error at this line likely means your copy of the
// proto package needs to be updated.
const _ = proto.ProtoPackageIsVersion2 // please upgrade the proto package

type ResourceMeteringRequest struct {
	XXX_NoUnkeyedLiteral struct{} `json:"-"`
	XXX_unrecognized     []byte   `json:"-"`
	XXX_sizecache        int32    `json:"-"`
}

func (m *ResourceMeteringRequest) Reset()         { *m = ResourceMeteringRequest{} }
func (m *ResourceMeteringRequest) String() string { return proto.CompactTextString(m) }
func (*ResourceMeteringRequest) ProtoMessage()    {}
func (*ResourceMeteringRequest) Descriptor() ([]byte, []int) {
	return fileDescriptor_resource_usage_agent_0653d980a61eb7ce, []int{0}
}
func (m *ResourceMeteringRequest) XXX_Unmarshal(b []byte) error {
	return m.Unmarshal(b)
}
func (m *ResourceMeteringRequest) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	if deterministic {
		return xxx_messageInfo_ResourceMeteringRequest.Marshal(b, m, deterministic)
	} else {
		b = b[:cap(b)]
		n, err := m.MarshalTo(b)
		if err != nil {
			return nil, err
		}
		return b[:n], nil
	}
}
func (dst *ResourceMeteringRequest) XXX_Merge(src proto.Message) {
	xxx_messageInfo_ResourceMeteringRequest.Merge(dst, src)
}
func (m *ResourceMeteringRequest) XXX_Size() int {
	return m.Size()
}
func (m *ResourceMeteringRequest) XXX_DiscardUnknown() {
	xxx_messageInfo_ResourceMeteringRequest.DiscardUnknown(m)
}

var xxx_messageInfo_ResourceMeteringRequest proto.InternalMessageInfo

type EmptyResponse struct {
	XXX_NoUnkeyedLiteral struct{} `json:"-"`
	XXX_unrecognized     []byte   `json:"-"`
	XXX_sizecache        int32    `json:"-"`
}

func (m *EmptyResponse) Reset()         { *m = EmptyResponse{} }
func (m *EmptyResponse) String() string { return proto.CompactTextString(m) }
func (*EmptyResponse) ProtoMessage()    {}
func (*EmptyResponse) Descriptor() ([]byte, []int) {
	return fileDescriptor_resource_usage_agent_0653d980a61eb7ce, []int{1}
}
func (m *EmptyResponse) XXX_Unmarshal(b []byte) error {
	return m.Unmarshal(b)
}
func (m *EmptyResponse) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	if deterministic {
		return xxx_messageInfo_EmptyResponse.Marshal(b, m, deterministic)
	} else {
		b = b[:cap(b)]
		n, err := m.MarshalTo(b)
		if err != nil {
			return nil, err
		}
		return b[:n], nil
	}
}
func (dst *EmptyResponse) XXX_Merge(src proto.Message) {
	xxx_messageInfo_EmptyResponse.Merge(dst, src)
}
func (m *EmptyResponse) XXX_Size() int {
	return m.Size()
}
func (m *EmptyResponse) XXX_DiscardUnknown() {
	xxx_messageInfo_EmptyResponse.DiscardUnknown(m)
}

var xxx_messageInfo_EmptyResponse proto.InternalMessageInfo

type ResourceUsageRecord struct {
	ResourceGroupTag     []byte                     `protobuf:"bytes,1,opt,name=resource_group_tag,json=resourceGroupTag,proto3" json:"resource_group_tag,omitempty"`
	Items                []*ResourceUsageRecordItem `protobuf:"bytes,2,rep,name=items" json:"items,omitempty"`
	XXX_NoUnkeyedLiteral struct{}                   `json:"-"`
	XXX_unrecognized     []byte                     `json:"-"`
	XXX_sizecache        int32                      `json:"-"`
}

func (m *ResourceUsageRecord) Reset()         { *m = ResourceUsageRecord{} }
func (m *ResourceUsageRecord) String() string { return proto.CompactTextString(m) }
func (*ResourceUsageRecord) ProtoMessage()    {}
func (*ResourceUsageRecord) Descriptor() ([]byte, []int) {
	return fileDescriptor_resource_usage_agent_0653d980a61eb7ce, []int{2}
}
func (m *ResourceUsageRecord) XXX_Unmarshal(b []byte) error {
	return m.Unmarshal(b)
}
func (m *ResourceUsageRecord) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	if deterministic {
		return xxx_messageInfo_ResourceUsageRecord.Marshal(b, m, deterministic)
	} else {
		b = b[:cap(b)]
		n, err := m.MarshalTo(b)
		if err != nil {
			return nil, err
		}
		return b[:n], nil
	}
}
func (dst *ResourceUsageRecord) XXX_Merge(src proto.Message) {
	xxx_messageInfo_ResourceUsageRecord.Merge(dst, src)
}
func (m *ResourceUsageRecord) XXX_Size() int {
	return m.Size()
}
func (m *ResourceUsageRecord) XXX_DiscardUnknown() {
	xxx_messageInfo_ResourceUsageRecord.DiscardUnknown(m)
}

var xxx_messageInfo_ResourceUsageRecord proto.InternalMessageInfo

func (m *ResourceUsageRecord) GetResourceGroupTag() []byte {
	if m != nil {
		return m.ResourceGroupTag
	}
	return nil
}

func (m *ResourceUsageRecord) GetItems() []*ResourceUsageRecordItem {
	if m != nil {
		return m.Items
	}
	return nil
}

type ResourceUsageRecordItem struct {
	TimestampSec         uint64   `protobuf:"varint,1,opt,name=timestamp_sec,json=timestampSec,proto3" json:"timestamp_sec,omitempty"`
	CpuTimeMs            uint32   `protobuf:"varint,2,opt,name=cpu_time_ms,json=cpuTimeMs,proto3" json:"cpu_time_ms,omitempty"`
	ReadKeys             uint32   `protobuf:"varint,3,opt,name=read_keys,json=readKeys,proto3" json:"read_keys,omitempty"`
	WriteKeys            uint32   `protobuf:"varint,4,opt,name=write_keys,json=writeKeys,proto3" json:"write_keys,omitempty"`
	XXX_NoUnkeyedLiteral struct{} `json:"-"`
	XXX_unrecognized     []byte   `json:"-"`
	XXX_sizecache        int32    `json:"-"`
}

func (m *ResourceUsageRecordItem) Reset()         { *m = ResourceUsageRecordItem{} }
func (m *ResourceUsageRecordItem) String() string { return proto.CompactTextString(m) }
func (*ResourceUsageRecordItem) ProtoMessage()    {}
func (*ResourceUsageRecordItem) Descriptor() ([]byte, []int) {
	return fileDescriptor_resource_usage_agent_0653d980a61eb7ce, []int{3}
}
func (m *ResourceUsageRecordItem) XXX_Unmarshal(b []byte) error {
	return m.Unmarshal(b)
}
func (m *ResourceUsageRecordItem) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	if deterministic {
		return xxx_messageInfo_ResourceUsageRecordItem.Marshal(b, m, deterministic)
	} else {
		b = b[:cap(b)]
		n, err := m.MarshalTo(b)
		if err != nil {
			return nil, err
		}
		return b[:n], nil
	}
}
func (dst *ResourceUsageRecordItem) XXX_Merge(src proto.Message) {
	xxx_messageInfo_ResourceUsageRecordItem.Merge(dst, src)
}
func (m *ResourceUsageRecordItem) XXX_Size() int {
	return m.Size()
}
func (m *ResourceUsageRecordItem) XXX_DiscardUnknown() {
	xxx_messageInfo_ResourceUsageRecordItem.DiscardUnknown(m)
}

var xxx_messageInfo_ResourceUsageRecordItem proto.InternalMessageInfo

func (m *ResourceUsageRecordItem) GetTimestampSec() uint64 {
	if m != nil {
		return m.TimestampSec
	}
	return 0
}

func (m *ResourceUsageRecordItem) GetCpuTimeMs() uint32 {
	if m != nil {
		return m.CpuTimeMs
	}
	return 0
}

func (m *ResourceUsageRecordItem) GetReadKeys() uint32 {
	if m != nil {
		return m.ReadKeys
	}
	return 0
}

func (m *ResourceUsageRecordItem) GetWriteKeys() uint32 {
	if m != nil {
		return m.WriteKeys
	}
	return 0
}

func init() {
	proto.RegisterType((*ResourceMeteringRequest)(nil), "resource_usage_agent.ResourceMeteringRequest")
	proto.RegisterType((*EmptyResponse)(nil), "resource_usage_agent.EmptyResponse")
	proto.RegisterType((*ResourceUsageRecord)(nil), "resource_usage_agent.ResourceUsageRecord")
	proto.RegisterType((*ResourceUsageRecordItem)(nil), "resource_usage_agent.ResourceUsageRecordItem")
}

// Reference imports to suppress errors if they are not otherwise used.
var _ context.Context
var _ grpc.ClientConn

// This is a compile-time assertion to ensure that this generated file
// is compatible with the grpc package it is being compiled against.
const _ = grpc.SupportPackageIsVersion4

// Client API for ResourceUsageAgent service

type ResourceUsageAgentClient interface {
	// Report the resource usage records. By default, the records with the same
	// resource group tag will be batched by minute.
	Report(ctx context.Context, opts ...grpc.CallOption) (ResourceUsageAgent_ReportClient, error)
}

type resourceUsageAgentClient struct {
	cc *grpc.ClientConn
}

func NewResourceUsageAgentClient(cc *grpc.ClientConn) ResourceUsageAgentClient {
	return &resourceUsageAgentClient{cc}
}

func (c *resourceUsageAgentClient) Report(ctx context.Context, opts ...grpc.CallOption) (ResourceUsageAgent_ReportClient, error) {
	stream, err := c.cc.NewStream(ctx, &_ResourceUsageAgent_serviceDesc.Streams[0], "/resource_usage_agent.ResourceUsageAgent/Report", opts...)
	if err != nil {
		return nil, err
	}
	x := &resourceUsageAgentReportClient{stream}
	return x, nil
}

type ResourceUsageAgent_ReportClient interface {
	Send(*ResourceUsageRecord) error
	CloseAndRecv() (*EmptyResponse, error)
	grpc.ClientStream
}

type resourceUsageAgentReportClient struct {
	grpc.ClientStream
}

func (x *resourceUsageAgentReportClient) Send(m *ResourceUsageRecord) error {
	return x.ClientStream.SendMsg(m)
}

func (x *resourceUsageAgentReportClient) CloseAndRecv() (*EmptyResponse, error) {
	if err := x.ClientStream.CloseSend(); err != nil {
		return nil, err
	}
	m := new(EmptyResponse)
	if err := x.ClientStream.RecvMsg(m); err != nil {
		return nil, err
	}
	return m, nil
}

// Server API for ResourceUsageAgent service

type ResourceUsageAgentServer interface {
	// Report the resource usage records. By default, the records with the same
	// resource group tag will be batched by minute.
	Report(ResourceUsageAgent_ReportServer) error
}

func RegisterResourceUsageAgentServer(s *grpc.Server, srv ResourceUsageAgentServer) {
	s.RegisterService(&_ResourceUsageAgent_serviceDesc, srv)
}

func _ResourceUsageAgent_Report_Handler(srv interface{}, stream grpc.ServerStream) error {
	return srv.(ResourceUsageAgentServer).Report(&resourceUsageAgentReportServer{stream})
}

type ResourceUsageAgent_ReportServer interface {
	SendAndClose(*EmptyResponse) error
	Recv() (*ResourceUsageRecord, error)
	grpc.ServerStream
}

type resourceUsageAgentReportServer struct {
	grpc.ServerStream
}

func (x *resourceUsageAgentReportServer) SendAndClose(m *EmptyResponse) error {
	return x.ServerStream.SendMsg(m)
}

func (x *resourceUsageAgentReportServer) Recv() (*ResourceUsageRecord, error) {
	m := new(ResourceUsageRecord)
	if err := x.ServerStream.RecvMsg(m); err != nil {
		return nil, err
	}
	return m, nil
}

var _ResourceUsageAgent_serviceDesc = grpc.ServiceDesc{
	ServiceName: "resource_usage_agent.ResourceUsageAgent",
	HandlerType: (*ResourceUsageAgentServer)(nil),
	Methods:     []grpc.MethodDesc{},
	Streams: []grpc.StreamDesc{
		{
			StreamName:    "Report",
			Handler:       _ResourceUsageAgent_Report_Handler,
			ClientStreams: true,
		},
	},
	Metadata: "resource_usage_agent.proto",
}

// Client API for ResourceMeteringPubSub service

type ResourceMeteringPubSubClient interface {
	// Clients subscribe to resource metering records through this RPC, and TiKV periodically (e.g. per minute)
	// publishes resource metering records to clients via gRPC stream.
	Subscribe(ctx context.Context, in *ResourceMeteringRequest, opts ...grpc.CallOption) (ResourceMeteringPubSub_SubscribeClient, error)
}

type resourceMeteringPubSubClient struct {
	cc *grpc.ClientConn
}

func NewResourceMeteringPubSubClient(cc *grpc.ClientConn) ResourceMeteringPubSubClient {
	return &resourceMeteringPubSubClient{cc}
}

func (c *resourceMeteringPubSubClient) Subscribe(ctx context.Context, in *ResourceMeteringRequest, opts ...grpc.CallOption) (ResourceMeteringPubSub_SubscribeClient, error) {
	stream, err := c.cc.NewStream(ctx, &_ResourceMeteringPubSub_serviceDesc.Streams[0], "/resource_usage_agent.ResourceMeteringPubSub/Subscribe", opts...)
	if err != nil {
		return nil, err
	}
	x := &resourceMeteringPubSubSubscribeClient{stream}
	if err := x.ClientStream.SendMsg(in); err != nil {
		return nil, err
	}
	if err := x.ClientStream.CloseSend(); err != nil {
		return nil, err
	}
	return x, nil
}

type ResourceMeteringPubSub_SubscribeClient interface {
	Recv() (*ResourceUsageRecord, error)
	grpc.ClientStream
}

type resourceMeteringPubSubSubscribeClient struct {
	grpc.ClientStream
}

func (x *resourceMeteringPubSubSubscribeClient) Recv() (*ResourceUsageRecord, error) {
	m := new(ResourceUsageRecord)
	if err := x.ClientStream.RecvMsg(m); err != nil {
		return nil, err
	}
	return m, nil
}

// Server API for ResourceMeteringPubSub service

type ResourceMeteringPubSubServer interface {
	// Clients subscribe to resource metering records through this RPC, and TiKV periodically (e.g. per minute)
	// publishes resource metering records to clients via gRPC stream.
	Subscribe(*ResourceMeteringRequest, ResourceMeteringPubSub_SubscribeServer) error
}

func RegisterResourceMeteringPubSubServer(s *grpc.Server, srv ResourceMeteringPubSubServer) {
	s.RegisterService(&_ResourceMeteringPubSub_serviceDesc, srv)
}

func _ResourceMeteringPubSub_Subscribe_Handler(srv interface{}, stream grpc.ServerStream) error {
	m := new(ResourceMeteringRequest)
	if err := stream.RecvMsg(m); err != nil {
		return err
	}
	return srv.(ResourceMeteringPubSubServer).Subscribe(m, &resourceMeteringPubSubSubscribeServer{stream})
}

type ResourceMeteringPubSub_SubscribeServer interface {
	Send(*ResourceUsageRecord) error
	grpc.ServerStream
}

type resourceMeteringPubSubSubscribeServer struct {
	grpc.ServerStream
}

func (x *resourceMeteringPubSubSubscribeServer) Send(m *ResourceUsageRecord) error {
	return x.ServerStream.SendMsg(m)
}

var _ResourceMeteringPubSub_serviceDesc = grpc.ServiceDesc{
	ServiceName: "resource_usage_agent.ResourceMeteringPubSub",
	HandlerType: (*ResourceMeteringPubSubServer)(nil),
	Methods:     []grpc.MethodDesc{},
	Streams: []grpc.StreamDesc{
		{
			StreamName:    "Subscribe",
			Handler:       _ResourceMeteringPubSub_Subscribe_Handler,
			ServerStreams: true,
		},
	},
	Metadata: "resource_usage_agent.proto",
}

func (m *ResourceMeteringRequest) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalTo(dAtA)
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *ResourceMeteringRequest) MarshalTo(dAtA []byte) (int, error) {
	var i int
	_ = i
	var l int
	_ = l
	if m.XXX_unrecognized != nil {
		i += copy(dAtA[i:], m.XXX_unrecognized)
	}
	return i, nil
}

func (m *EmptyResponse) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalTo(dAtA)
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *EmptyResponse) MarshalTo(dAtA []byte) (int, error) {
	var i int
	_ = i
	var l int
	_ = l
	if m.XXX_unrecognized != nil {
		i += copy(dAtA[i:], m.XXX_unrecognized)
	}
	return i, nil
}

func (m *ResourceUsageRecord) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalTo(dAtA)
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *ResourceUsageRecord) MarshalTo(dAtA []byte) (int, error) {
	var i int
	_ = i
	var l int
	_ = l
	if len(m.ResourceGroupTag) > 0 {
		dAtA[i] = 0xa
		i++
		i = encodeVarintResourceUsageAgent(dAtA, i, uint64(len(m.ResourceGroupTag)))
		i += copy(dAtA[i:], m.ResourceGroupTag)
	}
	if len(m.Items) > 0 {
		for _, msg := range m.Items {
			dAtA[i] = 0x12
			i++
			i = encodeVarintResourceUsageAgent(dAtA, i, uint64(msg.Size()))
			n, err := msg.MarshalTo(dAtA[i:])
			if err != nil {
				return 0, err
			}
			i += n
		}
	}
	if m.XXX_unrecognized != nil {
		i += copy(dAtA[i:], m.XXX_unrecognized)
	}
	return i, nil
}

func (m *ResourceUsageRecordItem) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalTo(dAtA)
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *ResourceUsageRecordItem) MarshalTo(dAtA []byte) (int, error) {
	var i int
	_ = i
	var l int
	_ = l
	if m.TimestampSec != 0 {
		dAtA[i] = 0x8
		i++
		i = encodeVarintResourceUsageAgent(dAtA, i, uint64(m.TimestampSec))
	}
	if m.CpuTimeMs != 0 {
		dAtA[i] = 0x10
		i++
		i = encodeVarintResourceUsageAgent(dAtA, i, uint64(m.CpuTimeMs))
	}
	if m.ReadKeys != 0 {
		dAtA[i] = 0x18
		i++
		i = encodeVarintResourceUsageAgent(dAtA, i, uint64(m.ReadKeys))
	}
	if m.WriteKeys != 0 {
		dAtA[i] = 0x20
		i++
		i = encodeVarintResourceUsageAgent(dAtA, i, uint64(m.WriteKeys))
	}
	if m.XXX_unrecognized != nil {
		i += copy(dAtA[i:], m.XXX_unrecognized)
	}
	return i, nil
}

func encodeVarintResourceUsageAgent(dAtA []byte, offset int, v uint64) int {
	for v >= 1<<7 {
		dAtA[offset] = uint8(v&0x7f | 0x80)
		v >>= 7
		offset++
	}
	dAtA[offset] = uint8(v)
	return offset + 1
}
func (m *ResourceMeteringRequest) Size() (n int) {
	var l int
	_ = l
	if m.XXX_unrecognized != nil {
		n += len(m.XXX_unrecognized)
	}
	return n
}

func (m *EmptyResponse) Size() (n int) {
	var l int
	_ = l
	if m.XXX_unrecognized != nil {
		n += len(m.XXX_unrecognized)
	}
	return n
}

func (m *ResourceUsageRecord) Size() (n int) {
	var l int
	_ = l
	l = len(m.ResourceGroupTag)
	if l > 0 {
		n += 1 + l + sovResourceUsageAgent(uint64(l))
	}
	if len(m.Items) > 0 {
		for _, e := range m.Items {
			l = e.Size()
			n += 1 + l + sovResourceUsageAgent(uint64(l))
		}
	}
	if m.XXX_unrecognized != nil {
		n += len(m.XXX_unrecognized)
	}
	return n
}

func (m *ResourceUsageRecordItem) Size() (n int) {
	var l int
	_ = l
	if m.TimestampSec != 0 {
		n += 1 + sovResourceUsageAgent(uint64(m.TimestampSec))
	}
	if m.CpuTimeMs != 0 {
		n += 1 + sovResourceUsageAgent(uint64(m.CpuTimeMs))
	}
	if m.ReadKeys != 0 {
		n += 1 + sovResourceUsageAgent(uint64(m.ReadKeys))
	}
	if m.WriteKeys != 0 {
		n += 1 + sovResourceUsageAgent(uint64(m.WriteKeys))
	}
	if m.XXX_unrecognized != nil {
		n += len(m.XXX_unrecognized)
	}
	return n
}

func sovResourceUsageAgent(x uint64) (n int) {
	for {
		n++
		x >>= 7
		if x == 0 {
			break
		}
	}
	return n
}
func sozResourceUsageAgent(x uint64) (n int) {
	return sovResourceUsageAgent(uint64((x << 1) ^ uint64((int64(x) >> 63))))
}
func (m *ResourceMeteringRequest) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowResourceUsageAgent
			}
			if iNdEx >= l {
				return io.ErrUnexpectedEOF
			}
			b := dAtA[iNdEx]
			iNdEx++
			wire |= (uint64(b) & 0x7F) << shift
			if b < 0x80 {
				break
			}
		}
		fieldNum := int32(wire >> 3)
		wireType := int(wire & 0x7)
		if wireType == 4 {
			return fmt.Errorf("proto: ResourceMeteringRequest: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: ResourceMeteringRequest: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		default:
			iNdEx = preIndex
			skippy, err := skipResourceUsageAgent(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if skippy < 0 {
				return ErrInvalidLengthResourceUsageAgent
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
func (m *EmptyResponse) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowResourceUsageAgent
			}
			if iNdEx >= l {
				return io.ErrUnexpectedEOF
			}
			b := dAtA[iNdEx]
			iNdEx++
			wire |= (uint64(b) & 0x7F) << shift
			if b < 0x80 {
				break
			}
		}
		fieldNum := int32(wire >> 3)
		wireType := int(wire & 0x7)
		if wireType == 4 {
			return fmt.Errorf("proto: EmptyResponse: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: EmptyResponse: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		default:
			iNdEx = preIndex
			skippy, err := skipResourceUsageAgent(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if skippy < 0 {
				return ErrInvalidLengthResourceUsageAgent
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
func (m *ResourceUsageRecord) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowResourceUsageAgent
			}
			if iNdEx >= l {
				return io.ErrUnexpectedEOF
			}
			b := dAtA[iNdEx]
			iNdEx++
			wire |= (uint64(b) & 0x7F) << shift
			if b < 0x80 {
				break
			}
		}
		fieldNum := int32(wire >> 3)
		wireType := int(wire & 0x7)
		if wireType == 4 {
			return fmt.Errorf("proto: ResourceUsageRecord: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: ResourceUsageRecord: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field ResourceGroupTag", wireType)
			}
			var byteLen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowResourceUsageAgent
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				byteLen |= (int(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if byteLen < 0 {
				return ErrInvalidLengthResourceUsageAgent
			}
			postIndex := iNdEx + byteLen
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.ResourceGroupTag = append(m.ResourceGroupTag[:0], dAtA[iNdEx:postIndex]...)
			if m.ResourceGroupTag == nil {
				m.ResourceGroupTag = []byte{}
			}
			iNdEx = postIndex
		case 2:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field Items", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowResourceUsageAgent
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				msglen |= (int(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if msglen < 0 {
				return ErrInvalidLengthResourceUsageAgent
			}
			postIndex := iNdEx + msglen
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.Items = append(m.Items, &ResourceUsageRecordItem{})
			if err := m.Items[len(m.Items)-1].Unmarshal(dAtA[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		default:
			iNdEx = preIndex
			skippy, err := skipResourceUsageAgent(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if skippy < 0 {
				return ErrInvalidLengthResourceUsageAgent
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
func (m *ResourceUsageRecordItem) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowResourceUsageAgent
			}
			if iNdEx >= l {
				return io.ErrUnexpectedEOF
			}
			b := dAtA[iNdEx]
			iNdEx++
			wire |= (uint64(b) & 0x7F) << shift
			if b < 0x80 {
				break
			}
		}
		fieldNum := int32(wire >> 3)
		wireType := int(wire & 0x7)
		if wireType == 4 {
			return fmt.Errorf("proto: ResourceUsageRecordItem: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: ResourceUsageRecordItem: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field TimestampSec", wireType)
			}
			m.TimestampSec = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowResourceUsageAgent
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				m.TimestampSec |= (uint64(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		case 2:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field CpuTimeMs", wireType)
			}
			m.CpuTimeMs = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowResourceUsageAgent
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				m.CpuTimeMs |= (uint32(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		case 3:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field ReadKeys", wireType)
			}
			m.ReadKeys = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowResourceUsageAgent
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				m.ReadKeys |= (uint32(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		case 4:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field WriteKeys", wireType)
			}
			m.WriteKeys = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowResourceUsageAgent
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				m.WriteKeys |= (uint32(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		default:
			iNdEx = preIndex
			skippy, err := skipResourceUsageAgent(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if skippy < 0 {
				return ErrInvalidLengthResourceUsageAgent
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
func skipResourceUsageAgent(dAtA []byte) (n int, err error) {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return 0, ErrIntOverflowResourceUsageAgent
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
					return 0, ErrIntOverflowResourceUsageAgent
				}
				if iNdEx >= l {
					return 0, io.ErrUnexpectedEOF
				}
				iNdEx++
				if dAtA[iNdEx-1] < 0x80 {
					break
				}
			}
			return iNdEx, nil
		case 1:
			iNdEx += 8
			return iNdEx, nil
		case 2:
			var length int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return 0, ErrIntOverflowResourceUsageAgent
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
			iNdEx += length
			if length < 0 {
				return 0, ErrInvalidLengthResourceUsageAgent
			}
			return iNdEx, nil
		case 3:
			for {
				var innerWire uint64
				var start int = iNdEx
				for shift := uint(0); ; shift += 7 {
					if shift >= 64 {
						return 0, ErrIntOverflowResourceUsageAgent
					}
					if iNdEx >= l {
						return 0, io.ErrUnexpectedEOF
					}
					b := dAtA[iNdEx]
					iNdEx++
					innerWire |= (uint64(b) & 0x7F) << shift
					if b < 0x80 {
						break
					}
				}
				innerWireType := int(innerWire & 0x7)
				if innerWireType == 4 {
					break
				}
				next, err := skipResourceUsageAgent(dAtA[start:])
				if err != nil {
					return 0, err
				}
				iNdEx = start + next
			}
			return iNdEx, nil
		case 4:
			return iNdEx, nil
		case 5:
			iNdEx += 4
			return iNdEx, nil
		default:
			return 0, fmt.Errorf("proto: illegal wireType %d", wireType)
		}
	}
	panic("unreachable")
}

var (
	ErrInvalidLengthResourceUsageAgent = fmt.Errorf("proto: negative length found during unmarshaling")
	ErrIntOverflowResourceUsageAgent   = fmt.Errorf("proto: integer overflow")
)

func init() {
	proto.RegisterFile("resource_usage_agent.proto", fileDescriptor_resource_usage_agent_0653d980a61eb7ce)
}

var fileDescriptor_resource_usage_agent_0653d980a61eb7ce = []byte{
	// 397 bytes of a gzipped FileDescriptorProto
	0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0xff, 0x8c, 0x52, 0xcd, 0xaa, 0xd3, 0x40,
	0x14, 0xee, 0x78, 0xaf, 0x97, 0xdb, 0x73, 0x5b, 0x5a, 0xc6, 0xa2, 0x35, 0x62, 0x28, 0x29, 0x48,
	0x04, 0x8d, 0x52, 0x9f, 0x40, 0x45, 0x44, 0xa4, 0x20, 0xd3, 0xba, 0x13, 0x42, 0x92, 0x1e, 0x86,
	0xa1, 0xa4, 0x33, 0xce, 0x4f, 0xa5, 0x6b, 0x37, 0x3e, 0x82, 0xf8, 0x04, 0x3e, 0x8a, 0x4b, 0x97,
	0x2e, 0xa5, 0xbe, 0x88, 0x64, 0x62, 0x2b, 0x2d, 0x81, 0xdb, 0xd5, 0x1c, 0xbe, 0xef, 0x3b, 0x9c,
	0xef, 0x9b, 0x73, 0x20, 0xd0, 0x68, 0xa4, 0xd3, 0x05, 0xa6, 0xce, 0x64, 0x1c, 0xd3, 0x8c, 0xe3,
	0xca, 0x26, 0x4a, 0x4b, 0x2b, 0xe9, 0xa0, 0x89, 0x0b, 0x06, 0x5c, 0x72, 0xe9, 0x05, 0x4f, 0xaa,
	0xaa, 0xd6, 0x06, 0x3d, 0xed, 0x8c, 0xf5, 0x65, 0x0d, 0x44, 0x77, 0xe1, 0x0e, 0xfb, 0xd7, 0x3e,
	0x45, 0x8b, 0x5a, 0xac, 0x38, 0xc3, 0x8f, 0x0e, 0x8d, 0x8d, 0x7a, 0xd0, 0x7d, 0x55, 0x2a, 0xbb,
	0x61, 0x68, 0x94, 0x5c, 0x19, 0x8c, 0xbe, 0x10, 0xb8, 0xb5, 0x13, 0xbf, 0xaf, 0x46, 0x31, 0x2c,
	0xa4, 0x5e, 0xd0, 0x47, 0x40, 0xf7, 0x16, 0xb8, 0x96, 0x4e, 0xa5, 0x36, 0xe3, 0x43, 0x32, 0x22,
	0x71, 0x87, 0xf5, 0x77, 0xcc, 0xeb, 0x8a, 0x98, 0x67, 0x9c, 0xbe, 0x84, 0x9b, 0xc2, 0x62, 0x69,
	0x86, 0x37, 0x46, 0x67, 0xf1, 0xd5, 0xe4, 0x71, 0xd2, 0x18, 0xad, 0x61, 0xce, 0x1b, 0x8b, 0x25,
	0xab, 0x7b, 0xa3, 0x6f, 0xe4, 0xbf, 0xef, 0x23, 0x09, 0x1d, 0x43, 0xd7, 0x8a, 0x12, 0x8d, 0xcd,
	0x4a, 0x95, 0x1a, 0x2c, 0xbc, 0x93, 0x73, 0xd6, 0xd9, 0x83, 0x33, 0x2c, 0x68, 0x08, 0x57, 0x85,
	0x72, 0x69, 0x85, 0xa5, 0xde, 0x0b, 0x89, 0xbb, 0xac, 0x5d, 0x28, 0x37, 0x17, 0x25, 0x4e, 0x0d,
	0xbd, 0x07, 0x6d, 0x8d, 0xd9, 0x22, 0x5d, 0xe2, 0xc6, 0x0c, 0xcf, 0x3c, 0x7b, 0x59, 0x01, 0x6f,
	0x71, 0x63, 0xe8, 0x7d, 0x80, 0x4f, 0x5a, 0x58, 0xac, 0xd9, 0xf3, 0xba, 0xd7, 0x23, 0x15, 0x3d,
	0xd1, 0x40, 0x0f, 0xbc, 0x3d, 0xaf, 0x12, 0xd1, 0x0f, 0x70, 0xc1, 0x50, 0x49, 0x6d, 0xe9, 0xc3,
	0x93, 0x23, 0x07, 0xe3, 0x66, 0xe9, 0xe1, 0x5e, 0x5a, 0x31, 0x99, 0x7c, 0x26, 0x70, 0xfb, 0x78,
	0x91, 0xef, 0x5c, 0x3e, 0x73, 0x39, 0x15, 0xd0, 0x9e, 0xb9, 0xdc, 0x14, 0x5a, 0xe4, 0x48, 0xaf,
	0xf9, 0xee, 0xa3, 0x1b, 0x08, 0x4e, 0xb7, 0x1a, 0xb5, 0x9e, 0x92, 0x17, 0x0f, 0x7e, 0x7d, 0xbf,
	0x24, 0x3f, 0xb6, 0x21, 0xf9, 0xb9, 0x0d, 0xc9, 0xef, 0x6d, 0x48, 0xbe, 0xfe, 0x09, 0x5b, 0xd0,
	0x97, 0x9a, 0x27, 0x56, 0x2c, 0xd7, 0xc9, 0x72, 0xed, 0xaf, 0x2e, 0xbf, 0xf0, 0xcf, 0xb3, 0xbf,
	0x01, 0x00, 0x00, 0xff, 0xff, 0xe7, 0x5c, 0xc6, 0x61, 0xd7, 0x02, 0x00, 0x00,
}
