// Code generated by protoc-gen-gogo.
// source: msgpb.proto
// DO NOT EDIT!

/*
	Package msgpb is a generated protocol buffer package.

	It is generated from these files:
		msgpb.proto

	It has these top-level messages:
		Message
*/
package msgpb

import (
	"fmt"
	"io"
	"math"

	proto "github.com/golang/protobuf/proto"

	raft_cmdpb "github.com/pingcap/kvproto/pkg/raft_cmdpb"

	raft_serverpb "github.com/pingcap/kvproto/pkg/raft_serverpb"

	kvrpcpb "github.com/pingcap/kvproto/pkg/kvrpcpb"

	coprocessor "github.com/pingcap/kvproto/pkg/coprocessor"

	pdpb "github.com/pingcap/kvproto/pkg/pdpb"
)

// Reference imports to suppress errors if they are not otherwise used.
var _ = proto.Marshal
var _ = fmt.Errorf
var _ = math.Inf

// This is a compile-time assertion to ensure that this generated file
// is compatible with the proto package it is being compiled against.
const _ = proto.ProtoPackageIsVersion1

type MessageType int32

const (
	MessageType_None    MessageType = 0
	MessageType_Cmd     MessageType = 1
	MessageType_CmdResp MessageType = 2
	MessageType_Raft    MessageType = 3
	MessageType_KvReq   MessageType = 4
	MessageType_KvResp  MessageType = 5
	MessageType_CopReq  MessageType = 6
	MessageType_CopResp MessageType = 7
	MessageType_PdReq   MessageType = 8
	MessageType_PdResp  MessageType = 9
)

var MessageType_name = map[int32]string{
	0: "None",
	1: "Cmd",
	2: "CmdResp",
	3: "Raft",
	4: "KvReq",
	5: "KvResp",
	6: "CopReq",
	7: "CopResp",
	8: "PdReq",
	9: "PdResp",
}
var MessageType_value = map[string]int32{
	"None":    0,
	"Cmd":     1,
	"CmdResp": 2,
	"Raft":    3,
	"KvReq":   4,
	"KvResp":  5,
	"CopReq":  6,
	"CopResp": 7,
	"PdReq":   8,
	"PdResp":  9,
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
func (MessageType) EnumDescriptor() ([]byte, []int) { return fileDescriptorMsgpb, []int{0} }

// Message holds all messages communicating with TiKV.
type Message struct {
	MsgType          MessageType                 `protobuf:"varint,1,opt,name=msg_type,enum=msgpb.MessageType" json:"msg_type"`
	CmdReq           *raft_cmdpb.RaftCmdRequest  `protobuf:"bytes,2,opt,name=cmd_req" json:"cmd_req,omitempty"`
	CmdResp          *raft_cmdpb.RaftCmdResponse `protobuf:"bytes,3,opt,name=cmd_resp" json:"cmd_resp,omitempty"`
	Raft             *raft_serverpb.RaftMessage  `protobuf:"bytes,4,opt,name=raft" json:"raft,omitempty"`
	KvReq            *kvrpcpb.Request            `protobuf:"bytes,5,opt,name=kv_req" json:"kv_req,omitempty"`
	KvResp           *kvrpcpb.Response           `protobuf:"bytes,6,opt,name=kv_resp" json:"kv_resp,omitempty"`
	CopReq           *coprocessor.Request        `protobuf:"bytes,7,opt,name=cop_req" json:"cop_req,omitempty"`
	CopResp          *coprocessor.Response       `protobuf:"bytes,8,opt,name=cop_resp" json:"cop_resp,omitempty"`
	PdReq            *pdpb.Request               `protobuf:"bytes,9,opt,name=pd_req" json:"pd_req,omitempty"`
	PdResp           *pdpb.Response              `protobuf:"bytes,10,opt,name=pd_resp" json:"pd_resp,omitempty"`
	XXX_unrecognized []byte                      `json:"-"`
}

func (m *Message) Reset()                    { *m = Message{} }
func (m *Message) String() string            { return proto.CompactTextString(m) }
func (*Message) ProtoMessage()               {}
func (*Message) Descriptor() ([]byte, []int) { return fileDescriptorMsgpb, []int{0} }

func (m *Message) GetMsgType() MessageType {
	if m != nil {
		return m.MsgType
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

func init() {
	proto.RegisterType((*Message)(nil), "msgpb.Message")
	proto.RegisterEnum("msgpb.MessageType", MessageType_name, MessageType_value)
}
func (m *Message) Marshal() (data []byte, err error) {
	size := m.Size()
	data = make([]byte, size)
	n, err := m.MarshalTo(data)
	if err != nil {
		return nil, err
	}
	return data[:n], nil
}

func (m *Message) MarshalTo(data []byte) (int, error) {
	var i int
	_ = i
	var l int
	_ = l
	data[i] = 0x8
	i++
	i = encodeVarintMsgpb(data, i, uint64(m.MsgType))
	if m.CmdReq != nil {
		data[i] = 0x12
		i++
		i = encodeVarintMsgpb(data, i, uint64(m.CmdReq.Size()))
		n1, err := m.CmdReq.MarshalTo(data[i:])
		if err != nil {
			return 0, err
		}
		i += n1
	}
	if m.CmdResp != nil {
		data[i] = 0x1a
		i++
		i = encodeVarintMsgpb(data, i, uint64(m.CmdResp.Size()))
		n2, err := m.CmdResp.MarshalTo(data[i:])
		if err != nil {
			return 0, err
		}
		i += n2
	}
	if m.Raft != nil {
		data[i] = 0x22
		i++
		i = encodeVarintMsgpb(data, i, uint64(m.Raft.Size()))
		n3, err := m.Raft.MarshalTo(data[i:])
		if err != nil {
			return 0, err
		}
		i += n3
	}
	if m.KvReq != nil {
		data[i] = 0x2a
		i++
		i = encodeVarintMsgpb(data, i, uint64(m.KvReq.Size()))
		n4, err := m.KvReq.MarshalTo(data[i:])
		if err != nil {
			return 0, err
		}
		i += n4
	}
	if m.KvResp != nil {
		data[i] = 0x32
		i++
		i = encodeVarintMsgpb(data, i, uint64(m.KvResp.Size()))
		n5, err := m.KvResp.MarshalTo(data[i:])
		if err != nil {
			return 0, err
		}
		i += n5
	}
	if m.CopReq != nil {
		data[i] = 0x3a
		i++
		i = encodeVarintMsgpb(data, i, uint64(m.CopReq.Size()))
		n6, err := m.CopReq.MarshalTo(data[i:])
		if err != nil {
			return 0, err
		}
		i += n6
	}
	if m.CopResp != nil {
		data[i] = 0x42
		i++
		i = encodeVarintMsgpb(data, i, uint64(m.CopResp.Size()))
		n7, err := m.CopResp.MarshalTo(data[i:])
		if err != nil {
			return 0, err
		}
		i += n7
	}
	if m.PdReq != nil {
		data[i] = 0x4a
		i++
		i = encodeVarintMsgpb(data, i, uint64(m.PdReq.Size()))
		n8, err := m.PdReq.MarshalTo(data[i:])
		if err != nil {
			return 0, err
		}
		i += n8
	}
	if m.PdResp != nil {
		data[i] = 0x52
		i++
		i = encodeVarintMsgpb(data, i, uint64(m.PdResp.Size()))
		n9, err := m.PdResp.MarshalTo(data[i:])
		if err != nil {
			return 0, err
		}
		i += n9
	}
	if m.XXX_unrecognized != nil {
		i += copy(data[i:], m.XXX_unrecognized)
	}
	return i, nil
}

func encodeFixed64Msgpb(data []byte, offset int, v uint64) int {
	data[offset] = uint8(v)
	data[offset+1] = uint8(v >> 8)
	data[offset+2] = uint8(v >> 16)
	data[offset+3] = uint8(v >> 24)
	data[offset+4] = uint8(v >> 32)
	data[offset+5] = uint8(v >> 40)
	data[offset+6] = uint8(v >> 48)
	data[offset+7] = uint8(v >> 56)
	return offset + 8
}
func encodeFixed32Msgpb(data []byte, offset int, v uint32) int {
	data[offset] = uint8(v)
	data[offset+1] = uint8(v >> 8)
	data[offset+2] = uint8(v >> 16)
	data[offset+3] = uint8(v >> 24)
	return offset + 4
}
func encodeVarintMsgpb(data []byte, offset int, v uint64) int {
	for v >= 1<<7 {
		data[offset] = uint8(v&0x7f | 0x80)
		v >>= 7
		offset++
	}
	data[offset] = uint8(v)
	return offset + 1
}
func (m *Message) Size() (n int) {
	var l int
	_ = l
	n += 1 + sovMsgpb(uint64(m.MsgType))
	if m.CmdReq != nil {
		l = m.CmdReq.Size()
		n += 1 + l + sovMsgpb(uint64(l))
	}
	if m.CmdResp != nil {
		l = m.CmdResp.Size()
		n += 1 + l + sovMsgpb(uint64(l))
	}
	if m.Raft != nil {
		l = m.Raft.Size()
		n += 1 + l + sovMsgpb(uint64(l))
	}
	if m.KvReq != nil {
		l = m.KvReq.Size()
		n += 1 + l + sovMsgpb(uint64(l))
	}
	if m.KvResp != nil {
		l = m.KvResp.Size()
		n += 1 + l + sovMsgpb(uint64(l))
	}
	if m.CopReq != nil {
		l = m.CopReq.Size()
		n += 1 + l + sovMsgpb(uint64(l))
	}
	if m.CopResp != nil {
		l = m.CopResp.Size()
		n += 1 + l + sovMsgpb(uint64(l))
	}
	if m.PdReq != nil {
		l = m.PdReq.Size()
		n += 1 + l + sovMsgpb(uint64(l))
	}
	if m.PdResp != nil {
		l = m.PdResp.Size()
		n += 1 + l + sovMsgpb(uint64(l))
	}
	if m.XXX_unrecognized != nil {
		n += len(m.XXX_unrecognized)
	}
	return n
}

func sovMsgpb(x uint64) (n int) {
	for {
		n++
		x >>= 7
		if x == 0 {
			break
		}
	}
	return n
}
func sozMsgpb(x uint64) (n int) {
	return sovMsgpb(uint64((x << 1) ^ uint64((int64(x) >> 63))))
}
func (m *Message) Unmarshal(data []byte) error {
	l := len(data)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowMsgpb
			}
			if iNdEx >= l {
				return io.ErrUnexpectedEOF
			}
			b := data[iNdEx]
			iNdEx++
			wire |= (uint64(b) & 0x7F) << shift
			if b < 0x80 {
				break
			}
		}
		fieldNum := int32(wire >> 3)
		wireType := int(wire & 0x7)
		if wireType == 4 {
			return fmt.Errorf("proto: Message: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: Message: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field MsgType", wireType)
			}
			m.MsgType = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowMsgpb
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := data[iNdEx]
				iNdEx++
				m.MsgType |= (MessageType(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		case 2:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field CmdReq", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowMsgpb
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := data[iNdEx]
				iNdEx++
				msglen |= (int(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if msglen < 0 {
				return ErrInvalidLengthMsgpb
			}
			postIndex := iNdEx + msglen
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			if m.CmdReq == nil {
				m.CmdReq = &raft_cmdpb.RaftCmdRequest{}
			}
			if err := m.CmdReq.Unmarshal(data[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		case 3:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field CmdResp", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowMsgpb
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := data[iNdEx]
				iNdEx++
				msglen |= (int(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if msglen < 0 {
				return ErrInvalidLengthMsgpb
			}
			postIndex := iNdEx + msglen
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			if m.CmdResp == nil {
				m.CmdResp = &raft_cmdpb.RaftCmdResponse{}
			}
			if err := m.CmdResp.Unmarshal(data[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		case 4:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field Raft", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowMsgpb
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := data[iNdEx]
				iNdEx++
				msglen |= (int(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if msglen < 0 {
				return ErrInvalidLengthMsgpb
			}
			postIndex := iNdEx + msglen
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			if m.Raft == nil {
				m.Raft = &raft_serverpb.RaftMessage{}
			}
			if err := m.Raft.Unmarshal(data[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		case 5:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field KvReq", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowMsgpb
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := data[iNdEx]
				iNdEx++
				msglen |= (int(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if msglen < 0 {
				return ErrInvalidLengthMsgpb
			}
			postIndex := iNdEx + msglen
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			if m.KvReq == nil {
				m.KvReq = &kvrpcpb.Request{}
			}
			if err := m.KvReq.Unmarshal(data[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		case 6:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field KvResp", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowMsgpb
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := data[iNdEx]
				iNdEx++
				msglen |= (int(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if msglen < 0 {
				return ErrInvalidLengthMsgpb
			}
			postIndex := iNdEx + msglen
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			if m.KvResp == nil {
				m.KvResp = &kvrpcpb.Response{}
			}
			if err := m.KvResp.Unmarshal(data[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		case 7:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field CopReq", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowMsgpb
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := data[iNdEx]
				iNdEx++
				msglen |= (int(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if msglen < 0 {
				return ErrInvalidLengthMsgpb
			}
			postIndex := iNdEx + msglen
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			if m.CopReq == nil {
				m.CopReq = &coprocessor.Request{}
			}
			if err := m.CopReq.Unmarshal(data[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		case 8:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field CopResp", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowMsgpb
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := data[iNdEx]
				iNdEx++
				msglen |= (int(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if msglen < 0 {
				return ErrInvalidLengthMsgpb
			}
			postIndex := iNdEx + msglen
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			if m.CopResp == nil {
				m.CopResp = &coprocessor.Response{}
			}
			if err := m.CopResp.Unmarshal(data[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		case 9:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field PdReq", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowMsgpb
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := data[iNdEx]
				iNdEx++
				msglen |= (int(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if msglen < 0 {
				return ErrInvalidLengthMsgpb
			}
			postIndex := iNdEx + msglen
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			if m.PdReq == nil {
				m.PdReq = &pdpb.Request{}
			}
			if err := m.PdReq.Unmarshal(data[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		case 10:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field PdResp", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowMsgpb
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := data[iNdEx]
				iNdEx++
				msglen |= (int(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if msglen < 0 {
				return ErrInvalidLengthMsgpb
			}
			postIndex := iNdEx + msglen
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			if m.PdResp == nil {
				m.PdResp = &pdpb.Response{}
			}
			if err := m.PdResp.Unmarshal(data[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		default:
			iNdEx = preIndex
			skippy, err := skipMsgpb(data[iNdEx:])
			if err != nil {
				return err
			}
			if skippy < 0 {
				return ErrInvalidLengthMsgpb
			}
			if (iNdEx + skippy) > l {
				return io.ErrUnexpectedEOF
			}
			m.XXX_unrecognized = append(m.XXX_unrecognized, data[iNdEx:iNdEx+skippy]...)
			iNdEx += skippy
		}
	}

	if iNdEx > l {
		return io.ErrUnexpectedEOF
	}
	return nil
}
func skipMsgpb(data []byte) (n int, err error) {
	l := len(data)
	iNdEx := 0
	for iNdEx < l {
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return 0, ErrIntOverflowMsgpb
			}
			if iNdEx >= l {
				return 0, io.ErrUnexpectedEOF
			}
			b := data[iNdEx]
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
					return 0, ErrIntOverflowMsgpb
				}
				if iNdEx >= l {
					return 0, io.ErrUnexpectedEOF
				}
				iNdEx++
				if data[iNdEx-1] < 0x80 {
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
					return 0, ErrIntOverflowMsgpb
				}
				if iNdEx >= l {
					return 0, io.ErrUnexpectedEOF
				}
				b := data[iNdEx]
				iNdEx++
				length |= (int(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			iNdEx += length
			if length < 0 {
				return 0, ErrInvalidLengthMsgpb
			}
			return iNdEx, nil
		case 3:
			for {
				var innerWire uint64
				var start int = iNdEx
				for shift := uint(0); ; shift += 7 {
					if shift >= 64 {
						return 0, ErrIntOverflowMsgpb
					}
					if iNdEx >= l {
						return 0, io.ErrUnexpectedEOF
					}
					b := data[iNdEx]
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
				next, err := skipMsgpb(data[start:])
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
	ErrInvalidLengthMsgpb = fmt.Errorf("proto: negative length found during unmarshaling")
	ErrIntOverflowMsgpb   = fmt.Errorf("proto: integer overflow")
)

var fileDescriptorMsgpb = []byte{
	// 399 bytes of a gzipped FileDescriptorProto
	0x1f, 0x8b, 0x08, 0x00, 0x00, 0x09, 0x6e, 0x88, 0x02, 0xff, 0x6c, 0x91, 0xcf, 0xaa, 0xda, 0x40,
	0x14, 0xc6, 0x8d, 0xf9, 0x33, 0xf1, 0x04, 0x65, 0x9c, 0x5a, 0x08, 0x96, 0x46, 0x11, 0x4a, 0xc5,
	0xd2, 0x14, 0x7c, 0x04, 0x5d, 0x96, 0x16, 0x91, 0xee, 0xc5, 0x26, 0x63, 0x16, 0x12, 0xe7, 0x38,
	0x93, 0x06, 0xdc, 0xf4, 0x1d, 0xba, 0xeb, 0x23, 0xb9, 0xec, 0x13, 0x94, 0x8b, 0xf7, 0x45, 0x2e,
	0x33, 0x49, 0xbc, 0xe1, 0x72, 0x77, 0x27, 0xdf, 0xf7, 0x9b, 0x6f, 0x4e, 0xbe, 0x81, 0x20, 0x57,
	0x19, 0xfe, 0x8c, 0x51, 0x8a, 0x42, 0x30, 0xd7, 0x7c, 0x8c, 0xa9, 0xdc, 0x1f, 0x8a, 0x5d, 0x92,
	0xa7, 0x8d, 0x31, 0x7e, 0x63, 0x14, 0xc5, 0x65, 0xc9, 0xe5, 0x5d, 0xec, 0x1f, 0x4b, 0x89, 0xc9,
	0xfd, 0x73, 0x98, 0x08, 0x94, 0x22, 0xe1, 0x4a, 0x09, 0x59, 0x4b, 0x80, 0xcf, 0x11, 0xa3, 0x4c,
	0x64, 0xc2, 0x8c, 0x5f, 0xf4, 0x54, 0xa9, 0xb3, 0x3f, 0x36, 0x90, 0x6f, 0x5c, 0xa9, 0x7d, 0xc6,
	0xd9, 0x02, 0xfc, 0x5c, 0x65, 0xbb, 0xe2, 0x82, 0x3c, 0xb4, 0xa6, 0xd6, 0x7c, 0xb0, 0x64, 0x71,
	0xb5, 0x5d, 0x4d, 0xfc, 0xb8, 0x20, 0x5f, 0x39, 0xd7, 0xff, 0x93, 0x0e, 0xfb, 0x04, 0x24, 0xc9,
	0xd3, 0x9d, 0xe4, 0xe7, 0xb0, 0x3b, 0xb5, 0xe6, 0xc1, 0x72, 0x1c, 0xb7, 0x96, 0xde, 0xee, 0x0f,
	0xc5, 0x3a, 0x4f, 0xb7, 0xfc, 0xfc, 0x8b, 0xab, 0x82, 0x7d, 0x06, 0xbf, 0x82, 0x15, 0x86, 0xb6,
	0xa1, 0xdf, 0xbd, 0x4a, 0x2b, 0x14, 0x27, 0xc5, 0xd9, 0x1c, 0x1c, 0xed, 0x86, 0x4e, 0x3b, 0xf8,
	0xfe, 0xef, 0x9a, 0x6e, 0x36, 0x9e, 0x82, 0x77, 0x2c, 0xcd, 0x12, 0xae, 0x61, 0x69, 0xdc, 0x54,
	0xd2, 0x5c, 0x3d, 0x03, 0x62, 0x08, 0x85, 0xa1, 0x67, 0x90, 0x61, 0x0b, 0xa9, 0xef, 0xfb, 0x00,
	0x24, 0x11, 0x68, 0x62, 0x88, 0x61, 0x46, 0x71, 0xbb, 0xca, 0x26, 0xea, 0x23, 0xf8, 0x15, 0xa6,
	0x30, 0xf4, 0x0d, 0xf7, 0xf6, 0x05, 0x57, 0xe7, 0xbd, 0x07, 0x0f, 0xab, 0x6a, 0x7a, 0x06, 0xeb,
	0xc7, 0xe6, 0x19, 0x9a, 0x9c, 0x09, 0x10, 0xac, 0xcb, 0x00, 0xe3, 0x0f, 0x1a, 0xbf, 0x3a, 0xbf,
	0xf8, 0x0d, 0x41, 0xab, 0x70, 0xe6, 0x83, 0xf3, 0x5d, 0x9c, 0x38, 0xed, 0x30, 0x02, 0xf6, 0x3a,
	0x4f, 0xa9, 0xc5, 0x02, 0x20, 0x75, 0x61, 0xb4, 0xab, 0x7d, 0xdd, 0x09, 0xb5, 0x59, 0x0f, 0xdc,
	0xaf, 0xe5, 0x96, 0x9f, 0xa9, 0xc3, 0x00, 0x3c, 0x3d, 0x2a, 0xa4, 0xae, 0x9e, 0xd7, 0x02, 0xb5,
	0xee, 0x99, 0x93, 0x7a, 0x56, 0x48, 0x89, 0xe6, 0x37, 0xfa, 0x91, 0xa8, 0xaf, 0x99, 0x4d, 0x15,
	0xd8, 0x5b, 0xd1, 0xeb, 0x2d, 0xb2, 0xfe, 0xdd, 0x22, 0xeb, 0xe1, 0x16, 0x59, 0x7f, 0x1f, 0xa3,
	0xce, 0x53, 0x00, 0x00, 0x00, 0xff, 0xff, 0x62, 0x73, 0xa8, 0x29, 0xa5, 0x02, 0x00, 0x00,
}
