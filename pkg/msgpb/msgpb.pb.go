// Code generated by protoc-gen-go.
// source: msgpb.proto
// DO NOT EDIT!

/*
Package msgpb is a generated protocol buffer package.

It is generated from these files:
	msgpb.proto

It has these top-level messages:
	Message
	SnapFileMeta
*/
package msgpb

import proto "github.com/golang/protobuf/proto"
import fmt "fmt"
import math "math"
import raft_cmdpb "github.com/pingcap/kvproto/pkg/raft_cmdpb"
import raft_serverpb "github.com/pingcap/kvproto/pkg/raft_serverpb"
import kvrpcpb "github.com/pingcap/kvproto/pkg/kvrpcpb"
import coprocessor "github.com/pingcap/kvproto/pkg/coprocessor"
import pdpb "github.com/pingcap/kvproto/pkg/pdpb"

// Reference imports to suppress errors if they are not otherwise used.
var _ = proto.Marshal
var _ = fmt.Errorf
var _ = math.Inf

// This is a compile-time assertion to ensure that this generated file
// is compatible with the proto package it is being compiled against.
const _ = proto.ProtoPackageIsVersion1

type MessageType int32

const (
	MessageType_None     MessageType = 0
	MessageType_Cmd      MessageType = 1
	MessageType_CmdResp  MessageType = 2
	MessageType_Raft     MessageType = 3
	MessageType_KvReq    MessageType = 4
	MessageType_KvResp   MessageType = 5
	MessageType_CopReq   MessageType = 6
	MessageType_CopResp  MessageType = 7
	MessageType_PdReq    MessageType = 8
	MessageType_PdResp   MessageType = 9
	MessageType_Snapshot MessageType = 10
)

var MessageType_name = map[int32]string{
	0:  "None",
	1:  "Cmd",
	2:  "CmdResp",
	3:  "Raft",
	4:  "KvReq",
	5:  "KvResp",
	6:  "CopReq",
	7:  "CopResp",
	8:  "PdReq",
	9:  "PdResp",
	10: "Snapshot",
}
var MessageType_value = map[string]int32{
	"None":     0,
	"Cmd":      1,
	"CmdResp":  2,
	"Raft":     3,
	"KvReq":    4,
	"KvResp":   5,
	"CopReq":   6,
	"CopResp":  7,
	"PdReq":    8,
	"PdResp":   9,
	"Snapshot": 10,
}

func (x MessageType) Enum() *MessageType {
	p := new(MessageType)
	*p = x
	return p
}
func (x MessageType) String() string {
	return proto.EnumName(MessageType_name, int32(x))
}
func (x *MessageType) UnmarshalJSON(data []byte) error {
	value, err := proto.UnmarshalJSONEnum(MessageType_value, data, "MessageType")
	if err != nil {
		return err
	}
	*x = MessageType(value)
	return nil
}
func (MessageType) EnumDescriptor() ([]byte, []int) { return fileDescriptor0, []int{0} }

// Message holds all messages communicating with TiKV.
type Message struct {
	MsgType          *MessageType                `protobuf:"varint,1,opt,name=msg_type,enum=msgpb.MessageType" json:"msg_type,omitempty"`
	CmdReq           *raft_cmdpb.RaftCmdRequest  `protobuf:"bytes,2,opt,name=cmd_req" json:"cmd_req,omitempty"`
	CmdResp          *raft_cmdpb.RaftCmdResponse `protobuf:"bytes,3,opt,name=cmd_resp" json:"cmd_resp,omitempty"`
	Raft             *raft_serverpb.RaftMessage  `protobuf:"bytes,4,opt,name=raft" json:"raft,omitempty"`
	KvReq            *kvrpcpb.Request            `protobuf:"bytes,5,opt,name=kv_req" json:"kv_req,omitempty"`
	KvResp           *kvrpcpb.Response           `protobuf:"bytes,6,opt,name=kv_resp" json:"kv_resp,omitempty"`
	CopReq           *coprocessor.Request        `protobuf:"bytes,7,opt,name=cop_req" json:"cop_req,omitempty"`
	CopResp          *coprocessor.Response       `protobuf:"bytes,8,opt,name=cop_resp" json:"cop_resp,omitempty"`
	PdReq            *pdpb.Request               `protobuf:"bytes,9,opt,name=pd_req" json:"pd_req,omitempty"`
	PdResp           *pdpb.Response              `protobuf:"bytes,10,opt,name=pd_resp" json:"pd_resp,omitempty"`
	SnapFileMeta     *SnapFileMeta               `protobuf:"bytes,11,opt,name=snap_file_meta" json:"snap_file_meta,omitempty"`
	XXX_unrecognized []byte                      `json:"-"`
}

func (m *Message) Reset()                    { *m = Message{} }
func (m *Message) String() string            { return proto.CompactTextString(m) }
func (*Message) ProtoMessage()               {}
func (*Message) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{0} }

func (m *Message) GetMsgType() MessageType {
	if m != nil && m.MsgType != nil {
		return *m.MsgType
	}
	return MessageType_None
}

func (m *Message) GetCmdReq() *raft_cmdpb.RaftCmdRequest {
	if m != nil {
		return m.CmdReq
	}
	return nil
}

func (m *Message) GetCmdResp() *raft_cmdpb.RaftCmdResponse {
	if m != nil {
		return m.CmdResp
	}
	return nil
}

func (m *Message) GetRaft() *raft_serverpb.RaftMessage {
	if m != nil {
		return m.Raft
	}
	return nil
}

func (m *Message) GetKvReq() *kvrpcpb.Request {
	if m != nil {
		return m.KvReq
	}
	return nil
}

func (m *Message) GetKvResp() *kvrpcpb.Response {
	if m != nil {
		return m.KvResp
	}
	return nil
}

func (m *Message) GetCopReq() *coprocessor.Request {
	if m != nil {
		return m.CopReq
	}
	return nil
}

func (m *Message) GetCopResp() *coprocessor.Response {
	if m != nil {
		return m.CopResp
	}
	return nil
}

func (m *Message) GetPdReq() *pdpb.Request {
	if m != nil {
		return m.PdReq
	}
	return nil
}

func (m *Message) GetPdResp() *pdpb.Response {
	if m != nil {
		return m.PdResp
	}
	return nil
}

func (m *Message) GetSnapFileMeta() *SnapFileMeta {
	if m != nil {
		return m.SnapFileMeta
	}
	return nil
}

type SnapFileMeta struct {
	Region           *uint64 `protobuf:"varint,1,opt,name=region" json:"region,omitempty"`
	Term             *uint64 `protobuf:"varint,2,opt,name=term" json:"term,omitempty"`
	Index            *uint64 `protobuf:"varint,3,opt,name=index" json:"index,omitempty"`
	Size             *uint64 `protobuf:"varint,4,opt,name=size" json:"size,omitempty"`
	XXX_unrecognized []byte  `json:"-"`
}

func (m *SnapFileMeta) Reset()                    { *m = SnapFileMeta{} }
func (m *SnapFileMeta) String() string            { return proto.CompactTextString(m) }
func (*SnapFileMeta) ProtoMessage()               {}
func (*SnapFileMeta) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{1} }

func (m *SnapFileMeta) GetRegion() uint64 {
	if m != nil && m.Region != nil {
		return *m.Region
	}
	return 0
}

func (m *SnapFileMeta) GetTerm() uint64 {
	if m != nil && m.Term != nil {
		return *m.Term
	}
	return 0
}

func (m *SnapFileMeta) GetIndex() uint64 {
	if m != nil && m.Index != nil {
		return *m.Index
	}
	return 0
}

func (m *SnapFileMeta) GetSize() uint64 {
	if m != nil && m.Size != nil {
		return *m.Size
	}
	return 0
}

func init() {
	proto.RegisterType((*Message)(nil), "msgpb.Message")
	proto.RegisterType((*SnapFileMeta)(nil), "msgpb.SnapFileMeta")
	proto.RegisterEnum("msgpb.MessageType", MessageType_name, MessageType_value)
}

var fileDescriptor0 = []byte{
	// 433 bytes of a gzipped FileDescriptorProto
	0x1f, 0x8b, 0x08, 0x00, 0x00, 0x09, 0x6e, 0x88, 0x02, 0xff, 0x6c, 0x92, 0xe1, 0x6e, 0xd3, 0x30,
	0x10, 0xc7, 0xe9, 0xea, 0xc4, 0xe9, 0x65, 0xad, 0x3c, 0x0f, 0xa4, 0xa8, 0x08, 0x31, 0x55, 0x20,
	0x26, 0x26, 0xf2, 0x61, 0xaf, 0x30, 0x09, 0x09, 0xa1, 0xa1, 0x69, 0xf0, 0x3d, 0x2a, 0xed, 0xad,
	0x54, 0x23, 0xb1, 0x67, 0x87, 0x0a, 0x78, 0x06, 0x1e, 0x88, 0xc7, 0xe3, 0x7c, 0x71, 0xa2, 0x08,
	0xed, 0xdb, 0xd9, 0xff, 0x5f, 0xfe, 0xbe, 0xbb, 0x7f, 0x20, 0xaf, 0xfd, 0xce, 0x7e, 0x2d, 0xad,
	0x33, 0xad, 0xd1, 0x09, 0x1f, 0x96, 0xca, 0xad, 0xef, 0xda, 0x6a, 0x53, 0x6f, 0x7b, 0x61, 0x79,
	0xca, 0x37, 0x1e, 0xdd, 0x01, 0xdd, 0x70, 0x39, 0xbf, 0x3f, 0x38, 0xbb, 0x19, 0x8e, 0x27, 0x1b,
	0x43, 0xc5, 0x06, 0xbd, 0x37, 0x2e, 0x5e, 0x81, 0x1d, 0x2c, 0x56, 0x7f, 0xa7, 0x20, 0xaf, 0x49,
	0x5c, 0xef, 0x50, 0xbf, 0x82, 0x8c, 0x5e, 0xaa, 0xda, 0x5f, 0x16, 0x8b, 0xc9, 0xd9, 0xe4, 0x7c,
	0x71, 0xa9, 0xcb, 0xae, 0x8f, 0x48, 0x7c, 0x21, 0x45, 0x5f, 0x80, 0xa4, 0x1e, 0x2a, 0x87, 0x0f,
	0xc5, 0x11, 0x41, 0xf9, 0xe5, 0xb2, 0x1c, 0x35, 0x76, 0x4b, 0xe5, 0x55, 0xbd, 0xbd, 0xc5, 0x87,
	0x1f, 0xe8, 0x5b, 0xfd, 0x0e, 0xb2, 0x0e, 0xf6, 0xb6, 0x98, 0x32, 0xfd, 0xfc, 0x51, 0xda, 0x5b,
	0xd3, 0x78, 0xd4, 0xe7, 0x20, 0x82, 0x5a, 0x88, 0xb1, 0xf1, 0x30, 0x5f, 0xa0, 0xfb, 0x5e, 0xcf,
	0x20, 0xbd, 0x3f, 0x70, 0x13, 0x09, 0xb3, 0xaa, 0xec, 0xc7, 0xee, 0x9f, 0x5e, 0x81, 0x64, 0x82,
	0x5e, 0x4e, 0x19, 0x39, 0x19, 0x21, 0xf1, 0xbd, 0xd7, 0x34, 0x8b, 0xb1, 0x6c, 0x23, 0x99, 0x79,
	0x5a, 0x8e, 0xd7, 0xd5, 0x5b, 0xbd, 0xa1, 0x29, 0x18, 0x23, 0xaf, 0x8c, 0xb9, 0x67, 0xff, 0x71,
	0xd1, 0xef, 0x05, 0xa4, 0xb6, 0x5b, 0xcd, 0x8c, 0xb1, 0x79, 0xc9, 0xab, 0xee, 0x7d, 0x5e, 0x82,
	0xb4, 0x71, 0x19, 0xc0, 0xfa, 0xa2, 0xd7, 0xe3, 0xf7, 0x17, 0xb0, 0xf0, 0xcd, 0xda, 0x56, 0x77,
	0xfb, 0xef, 0x58, 0xd5, 0xd8, 0xae, 0x8b, 0x9c, 0xb9, 0xd3, 0x98, 0xc3, 0x67, 0x12, 0xdf, 0x93,
	0x76, 0x4d, 0xd2, 0xea, 0x03, 0x1c, 0x8f, 0xcf, 0x7a, 0x01, 0xa9, 0xc3, 0xdd, 0xde, 0x34, 0x1c,
	0x9e, 0xd0, 0xc7, 0x20, 0x5a, 0x74, 0x35, 0xa7, 0x24, 0xf4, 0x1c, 0x92, 0x7d, 0xb3, 0xc5, 0x9f,
	0x1c, 0x03, 0x8b, 0x7e, 0xff, 0x1b, 0x79, 0xd3, 0xe2, 0xed, 0x9f, 0x09, 0xe4, 0xe3, 0x8c, 0x33,
	0x10, 0x9f, 0x4c, 0x83, 0xea, 0x89, 0x96, 0x30, 0xa5, 0x80, 0xd4, 0x44, 0xe7, 0x20, 0x63, 0x52,
	0xea, 0x28, 0xe8, 0x21, 0x0c, 0x35, 0xd5, 0x33, 0x48, 0x3e, 0x1e, 0x68, 0x3e, 0x25, 0x34, 0x40,
	0x1a, 0x4a, 0x02, 0x92, 0x50, 0x5f, 0x19, 0x1b, 0xee, 0x53, 0xfe, 0x32, 0xd4, 0x24, 0xc8, 0xc0,
	0xdf, 0x84, 0xbf, 0x43, 0x65, 0x81, 0xb9, 0xe9, 0x0c, 0x67, 0xd4, 0x4e, 0x16, 0x66, 0xf1, 0xdf,
	0x4c, 0xab, 0xe0, 0x5f, 0x00, 0x00, 0x00, 0xff, 0xff, 0xea, 0xc3, 0xb2, 0x6f, 0xfe, 0x02, 0x00,
	0x00,
}
