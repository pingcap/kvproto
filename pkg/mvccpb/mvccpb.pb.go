// Code generated by protoc-gen-gogo.
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

import (
	"fmt"
	"io"
	"math"

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
const _ = proto.ProtoPackageIsVersion2 // please upgrade the proto package

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
func (MetaLockType) EnumDescriptor() ([]byte, []int) { return fileDescriptorMvccpb, []int{0} }

type MetaItem struct {
	StartTs          *uint64 `protobuf:"varint,1,opt,name=start_ts" json:"start_ts,omitempty"`
	CommitTs         *uint64 `protobuf:"varint,2,opt,name=commit_ts" json:"commit_ts,omitempty"`
	XXX_unrecognized []byte  `json:"-"`
}

func (m *MetaItem) Reset()                    { *m = MetaItem{} }
func (m *MetaItem) String() string            { return proto.CompactTextString(m) }
func (*MetaItem) ProtoMessage()               {}
func (*MetaItem) Descriptor() ([]byte, []int) { return fileDescriptorMvccpb, []int{0} }

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
	StartTs          *uint64       `protobuf:"varint,2,opt,name=start_ts" json:"start_ts,omitempty"`
	PrimaryKey       []byte        `protobuf:"bytes,3,opt,name=primary_key" json:"primary_key,omitempty"`
	XXX_unrecognized []byte        `json:"-"`
}

func (m *MetaLock) Reset()                    { *m = MetaLock{} }
func (m *MetaLock) String() string            { return proto.CompactTextString(m) }
func (*MetaLock) ProtoMessage()               {}
func (*MetaLock) Descriptor() ([]byte, []int) { return fileDescriptorMvccpb, []int{1} }

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
func (*Meta) Descriptor() ([]byte, []int) { return fileDescriptorMvccpb, []int{2} }

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
func (m *MetaItem) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalTo(dAtA)
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *MetaItem) MarshalTo(dAtA []byte) (int, error) {
	var i int
	_ = i
	var l int
	_ = l
	if m.StartTs != nil {
		dAtA[i] = 0x8
		i++
		i = encodeVarintMvccpb(dAtA, i, uint64(*m.StartTs))
	}
	if m.CommitTs != nil {
		dAtA[i] = 0x10
		i++
		i = encodeVarintMvccpb(dAtA, i, uint64(*m.CommitTs))
	}
	if m.XXX_unrecognized != nil {
		i += copy(dAtA[i:], m.XXX_unrecognized)
	}
	return i, nil
}

func (m *MetaLock) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalTo(dAtA)
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *MetaLock) MarshalTo(dAtA []byte) (int, error) {
	var i int
	_ = i
	var l int
	_ = l
	if m.Type != nil {
		dAtA[i] = 0x8
		i++
		i = encodeVarintMvccpb(dAtA, i, uint64(*m.Type))
	}
	if m.StartTs != nil {
		dAtA[i] = 0x10
		i++
		i = encodeVarintMvccpb(dAtA, i, uint64(*m.StartTs))
	}
	if m.PrimaryKey != nil {
		dAtA[i] = 0x1a
		i++
		i = encodeVarintMvccpb(dAtA, i, uint64(len(m.PrimaryKey)))
		i += copy(dAtA[i:], m.PrimaryKey)
	}
	if m.XXX_unrecognized != nil {
		i += copy(dAtA[i:], m.XXX_unrecognized)
	}
	return i, nil
}

func (m *Meta) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalTo(dAtA)
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *Meta) MarshalTo(dAtA []byte) (int, error) {
	var i int
	_ = i
	var l int
	_ = l
	if m.Lock != nil {
		dAtA[i] = 0xa
		i++
		i = encodeVarintMvccpb(dAtA, i, uint64(m.Lock.Size()))
		n1, err := m.Lock.MarshalTo(dAtA[i:])
		if err != nil {
			return 0, err
		}
		i += n1
	}
	if len(m.Items) > 0 {
		for _, msg := range m.Items {
			dAtA[i] = 0x12
			i++
			i = encodeVarintMvccpb(dAtA, i, uint64(msg.Size()))
			n, err := msg.MarshalTo(dAtA[i:])
			if err != nil {
				return 0, err
			}
			i += n
		}
	}
	if m.Next != nil {
		dAtA[i] = 0x18
		i++
		i = encodeVarintMvccpb(dAtA, i, uint64(*m.Next))
	}
	if m.XXX_unrecognized != nil {
		i += copy(dAtA[i:], m.XXX_unrecognized)
	}
	return i, nil
}

func encodeFixed64Mvccpb(dAtA []byte, offset int, v uint64) int {
	dAtA[offset] = uint8(v)
	dAtA[offset+1] = uint8(v >> 8)
	dAtA[offset+2] = uint8(v >> 16)
	dAtA[offset+3] = uint8(v >> 24)
	dAtA[offset+4] = uint8(v >> 32)
	dAtA[offset+5] = uint8(v >> 40)
	dAtA[offset+6] = uint8(v >> 48)
	dAtA[offset+7] = uint8(v >> 56)
	return offset + 8
}
func encodeFixed32Mvccpb(dAtA []byte, offset int, v uint32) int {
	dAtA[offset] = uint8(v)
	dAtA[offset+1] = uint8(v >> 8)
	dAtA[offset+2] = uint8(v >> 16)
	dAtA[offset+3] = uint8(v >> 24)
	return offset + 4
}
func encodeVarintMvccpb(dAtA []byte, offset int, v uint64) int {
	for v >= 1<<7 {
		dAtA[offset] = uint8(v&0x7f | 0x80)
		v >>= 7
		offset++
	}
	dAtA[offset] = uint8(v)
	return offset + 1
}
func (m *MetaItem) Size() (n int) {
	var l int
	_ = l
	if m.StartTs != nil {
		n += 1 + sovMvccpb(uint64(*m.StartTs))
	}
	if m.CommitTs != nil {
		n += 1 + sovMvccpb(uint64(*m.CommitTs))
	}
	if m.XXX_unrecognized != nil {
		n += len(m.XXX_unrecognized)
	}
	return n
}

func (m *MetaLock) Size() (n int) {
	var l int
	_ = l
	if m.Type != nil {
		n += 1 + sovMvccpb(uint64(*m.Type))
	}
	if m.StartTs != nil {
		n += 1 + sovMvccpb(uint64(*m.StartTs))
	}
	if m.PrimaryKey != nil {
		l = len(m.PrimaryKey)
		n += 1 + l + sovMvccpb(uint64(l))
	}
	if m.XXX_unrecognized != nil {
		n += len(m.XXX_unrecognized)
	}
	return n
}

func (m *Meta) Size() (n int) {
	var l int
	_ = l
	if m.Lock != nil {
		l = m.Lock.Size()
		n += 1 + l + sovMvccpb(uint64(l))
	}
	if len(m.Items) > 0 {
		for _, e := range m.Items {
			l = e.Size()
			n += 1 + l + sovMvccpb(uint64(l))
		}
	}
	if m.Next != nil {
		n += 1 + sovMvccpb(uint64(*m.Next))
	}
	if m.XXX_unrecognized != nil {
		n += len(m.XXX_unrecognized)
	}
	return n
}

func sovMvccpb(x uint64) (n int) {
	for {
		n++
		x >>= 7
		if x == 0 {
			break
		}
	}
	return n
}
func sozMvccpb(x uint64) (n int) {
	return sovMvccpb(uint64((x << 1) ^ uint64((int64(x) >> 63))))
}
func (m *MetaItem) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowMvccpb
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
			return fmt.Errorf("proto: MetaItem: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: MetaItem: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field StartTs", wireType)
			}
			var v uint64
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowMvccpb
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				v |= (uint64(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			m.StartTs = &v
		case 2:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field CommitTs", wireType)
			}
			var v uint64
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowMvccpb
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				v |= (uint64(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			m.CommitTs = &v
		default:
			iNdEx = preIndex
			skippy, err := skipMvccpb(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if skippy < 0 {
				return ErrInvalidLengthMvccpb
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
func (m *MetaLock) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowMvccpb
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
			return fmt.Errorf("proto: MetaLock: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: MetaLock: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field Type", wireType)
			}
			var v MetaLockType
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowMvccpb
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				v |= (MetaLockType(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			m.Type = &v
		case 2:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field StartTs", wireType)
			}
			var v uint64
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowMvccpb
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				v |= (uint64(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			m.StartTs = &v
		case 3:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field PrimaryKey", wireType)
			}
			var byteLen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowMvccpb
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
				return ErrInvalidLengthMvccpb
			}
			postIndex := iNdEx + byteLen
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.PrimaryKey = append(m.PrimaryKey[:0], dAtA[iNdEx:postIndex]...)
			if m.PrimaryKey == nil {
				m.PrimaryKey = []byte{}
			}
			iNdEx = postIndex
		default:
			iNdEx = preIndex
			skippy, err := skipMvccpb(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if skippy < 0 {
				return ErrInvalidLengthMvccpb
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
func (m *Meta) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowMvccpb
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
			return fmt.Errorf("proto: Meta: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: Meta: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field Lock", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowMvccpb
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
				return ErrInvalidLengthMvccpb
			}
			postIndex := iNdEx + msglen
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			if m.Lock == nil {
				m.Lock = &MetaLock{}
			}
			if err := m.Lock.Unmarshal(dAtA[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		case 2:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field Items", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowMvccpb
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
				return ErrInvalidLengthMvccpb
			}
			postIndex := iNdEx + msglen
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.Items = append(m.Items, &MetaItem{})
			if err := m.Items[len(m.Items)-1].Unmarshal(dAtA[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		case 3:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field Next", wireType)
			}
			var v uint64
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowMvccpb
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				v |= (uint64(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			m.Next = &v
		default:
			iNdEx = preIndex
			skippy, err := skipMvccpb(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if skippy < 0 {
				return ErrInvalidLengthMvccpb
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
func skipMvccpb(dAtA []byte) (n int, err error) {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return 0, ErrIntOverflowMvccpb
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
					return 0, ErrIntOverflowMvccpb
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
					return 0, ErrIntOverflowMvccpb
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
				return 0, ErrInvalidLengthMvccpb
			}
			return iNdEx, nil
		case 3:
			for {
				var innerWire uint64
				var start int = iNdEx
				for shift := uint(0); ; shift += 7 {
					if shift >= 64 {
						return 0, ErrIntOverflowMvccpb
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
				next, err := skipMvccpb(dAtA[start:])
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
	ErrInvalidLengthMvccpb = fmt.Errorf("proto: negative length found during unmarshaling")
	ErrIntOverflowMvccpb   = fmt.Errorf("proto: integer overflow")
)

func init() { proto.RegisterFile("mvccpb.proto", fileDescriptorMvccpb) }

var fileDescriptorMvccpb = []byte{
	// 239 bytes of a gzipped FileDescriptorProto
	0x1f, 0x8b, 0x08, 0x00, 0x00, 0x09, 0x6e, 0x88, 0x02, 0xff, 0xe2, 0xe2, 0xc9, 0x2d, 0x4b, 0x4e,
	0x2e, 0x48, 0xd2, 0x2b, 0x28, 0xca, 0x2f, 0xc9, 0x17, 0x62, 0x83, 0xf0, 0x94, 0xf4, 0xb9, 0x38,
	0x7c, 0x53, 0x4b, 0x12, 0x3d, 0x4b, 0x52, 0x73, 0x85, 0x04, 0xb8, 0x38, 0x8a, 0x4b, 0x12, 0x8b,
	0x4a, 0xe2, 0x4b, 0x8a, 0x25, 0x18, 0x15, 0x18, 0x35, 0x58, 0x84, 0x04, 0xb9, 0x38, 0x93, 0xf3,
	0x73, 0x73, 0x33, 0xc1, 0x42, 0x4c, 0x20, 0x21, 0xa5, 0x50, 0x88, 0x06, 0x9f, 0xfc, 0xe4, 0x6c,
	0x21, 0x25, 0x2e, 0x96, 0x92, 0xca, 0x82, 0x54, 0xb0, 0x62, 0x3e, 0x23, 0x11, 0x3d, 0xa8, 0x0d,
	0x30, 0xf9, 0x90, 0xca, 0x82, 0x54, 0x14, 0x43, 0xc1, 0x26, 0x08, 0x09, 0x73, 0x71, 0x17, 0x14,
	0x65, 0xe6, 0x26, 0x16, 0x55, 0xc6, 0x67, 0xa7, 0x56, 0x4a, 0x30, 0x2b, 0x30, 0x6a, 0xf0, 0x28,
	0x85, 0x72, 0xb1, 0x80, 0xb4, 0x09, 0xc9, 0x71, 0xb1, 0xe4, 0xe4, 0x27, 0x67, 0x83, 0x8d, 0xe4,
	0x36, 0x12, 0x40, 0x37, 0x52, 0x48, 0x9e, 0x8b, 0x35, 0xb3, 0x24, 0x35, 0x17, 0x64, 0x16, 0x33,
	0xba, 0x02, 0xb0, 0x27, 0x78, 0xb8, 0x58, 0xf2, 0x52, 0x2b, 0x4a, 0xc0, 0xc6, 0xb2, 0x68, 0x69,
	0x73, 0xf1, 0xa0, 0xb8, 0x86, 0x87, 0x8b, 0x23, 0x28, 0x35, 0x31, 0xc5, 0x3f, 0x2f, 0xa7, 0x52,
	0x80, 0x51, 0x88, 0x97, 0x8b, 0x13, 0xc4, 0x0b, 0x2f, 0xca, 0x2c, 0x49, 0x15, 0x60, 0x72, 0x12,
	0x38, 0xf1, 0x48, 0x8e, 0xf1, 0xc2, 0x23, 0x39, 0xc6, 0x07, 0x8f, 0xe4, 0x18, 0x67, 0x3c, 0x96,
	0x63, 0x00, 0x04, 0x00, 0x00, 0xff, 0xff, 0xea, 0xe0, 0x06, 0x4d, 0x34, 0x01, 0x00, 0x00,
}
