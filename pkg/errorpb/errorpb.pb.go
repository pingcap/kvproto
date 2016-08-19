// Code generated by protoc-gen-gogo.
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

import (
	"fmt"
	"io"
	"math"

	proto "github.com/golang/protobuf/proto"
)

import metapb "github.com/pingcap/kvproto/pkg/metapb"

// Reference imports to suppress errors if they are not otherwise used.
var _ = proto.Marshal
var _ = fmt.Errorf
var _ = math.Inf

// This is a compile-time assertion to ensure that this generated file
// is compatible with the proto package it is being compiled against.
// A compilation error at this line likely means your copy of the
// proto package needs to be updated.
const _ = proto.ProtoPackageIsVersion2 // please upgrade the proto package

type NotLeader struct {
	RegionId         *uint64      `protobuf:"varint,1,opt,name=region_id" json:"region_id,omitempty"`
	Leader           *metapb.Peer `protobuf:"bytes,2,opt,name=leader" json:"leader,omitempty"`
	XXX_unrecognized []byte       `json:"-"`
}

func (m *NotLeader) Reset()                    { *m = NotLeader{} }
func (m *NotLeader) String() string            { return proto.CompactTextString(m) }
func (*NotLeader) ProtoMessage()               {}
func (*NotLeader) Descriptor() ([]byte, []int) { return fileDescriptorErrorpb, []int{0} }

func (m *NotLeader) GetRegionId() uint64 {
	if m != nil && m.RegionId != nil {
		return *m.RegionId
	}
	return 0
}

func (m *NotLeader) GetLeader() *metapb.Peer {
	if m != nil {
		return m.Leader
	}
	return nil
}

type RegionNotFound struct {
	RegionId         *uint64 `protobuf:"varint,1,opt,name=region_id" json:"region_id,omitempty"`
	XXX_unrecognized []byte  `json:"-"`
}

func (m *RegionNotFound) Reset()                    { *m = RegionNotFound{} }
func (m *RegionNotFound) String() string            { return proto.CompactTextString(m) }
func (*RegionNotFound) ProtoMessage()               {}
func (*RegionNotFound) Descriptor() ([]byte, []int) { return fileDescriptorErrorpb, []int{1} }

func (m *RegionNotFound) GetRegionId() uint64 {
	if m != nil && m.RegionId != nil {
		return *m.RegionId
	}
	return 0
}

type KeyNotInRegion struct {
	Key              []byte  `protobuf:"bytes,1,opt,name=key" json:"key,omitempty"`
	RegionId         *uint64 `protobuf:"varint,2,opt,name=region_id" json:"region_id,omitempty"`
	StartKey         []byte  `protobuf:"bytes,3,opt,name=start_key" json:"start_key,omitempty"`
	EndKey           []byte  `protobuf:"bytes,4,opt,name=end_key" json:"end_key,omitempty"`
	XXX_unrecognized []byte  `json:"-"`
}

func (m *KeyNotInRegion) Reset()                    { *m = KeyNotInRegion{} }
func (m *KeyNotInRegion) String() string            { return proto.CompactTextString(m) }
func (*KeyNotInRegion) ProtoMessage()               {}
func (*KeyNotInRegion) Descriptor() ([]byte, []int) { return fileDescriptorErrorpb, []int{2} }

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
func (*StaleEpoch) Descriptor() ([]byte, []int) { return fileDescriptorErrorpb, []int{3} }

type Error struct {
	Message          *string         `protobuf:"bytes,1,opt,name=message" json:"message,omitempty"`
	NotLeader        *NotLeader      `protobuf:"bytes,2,opt,name=not_leader" json:"not_leader,omitempty"`
	RegionNotFound   *RegionNotFound `protobuf:"bytes,3,opt,name=region_not_found" json:"region_not_found,omitempty"`
	KeyNotInRegion   *KeyNotInRegion `protobuf:"bytes,4,opt,name=key_not_in_region" json:"key_not_in_region,omitempty"`
	StaleEpoch       *StaleEpoch     `protobuf:"bytes,5,opt,name=stale_epoch" json:"stale_epoch,omitempty"`
	XXX_unrecognized []byte          `json:"-"`
}

func (m *Error) Reset()                    { *m = Error{} }
func (m *Error) String() string            { return proto.CompactTextString(m) }
func (*Error) ProtoMessage()               {}
func (*Error) Descriptor() ([]byte, []int) { return fileDescriptorErrorpb, []int{4} }

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
func (m *NotLeader) Marshal() (data []byte, err error) {
	size := m.Size()
	data = make([]byte, size)
	n, err := m.MarshalTo(data)
	if err != nil {
		return nil, err
	}
	return data[:n], nil
}

func (m *NotLeader) MarshalTo(data []byte) (int, error) {
	var i int
	_ = i
	var l int
	_ = l
	if m.RegionId != nil {
		data[i] = 0x8
		i++
		i = encodeVarintErrorpb(data, i, uint64(*m.RegionId))
	}
	if m.Leader != nil {
		data[i] = 0x12
		i++
		i = encodeVarintErrorpb(data, i, uint64(m.Leader.Size()))
		n1, err := m.Leader.MarshalTo(data[i:])
		if err != nil {
			return 0, err
		}
		i += n1
	}
	if m.XXX_unrecognized != nil {
		i += copy(data[i:], m.XXX_unrecognized)
	}
	return i, nil
}

func (m *RegionNotFound) Marshal() (data []byte, err error) {
	size := m.Size()
	data = make([]byte, size)
	n, err := m.MarshalTo(data)
	if err != nil {
		return nil, err
	}
	return data[:n], nil
}

func (m *RegionNotFound) MarshalTo(data []byte) (int, error) {
	var i int
	_ = i
	var l int
	_ = l
	if m.RegionId != nil {
		data[i] = 0x8
		i++
		i = encodeVarintErrorpb(data, i, uint64(*m.RegionId))
	}
	if m.XXX_unrecognized != nil {
		i += copy(data[i:], m.XXX_unrecognized)
	}
	return i, nil
}

func (m *KeyNotInRegion) Marshal() (data []byte, err error) {
	size := m.Size()
	data = make([]byte, size)
	n, err := m.MarshalTo(data)
	if err != nil {
		return nil, err
	}
	return data[:n], nil
}

func (m *KeyNotInRegion) MarshalTo(data []byte) (int, error) {
	var i int
	_ = i
	var l int
	_ = l
	if m.Key != nil {
		data[i] = 0xa
		i++
		i = encodeVarintErrorpb(data, i, uint64(len(m.Key)))
		i += copy(data[i:], m.Key)
	}
	if m.RegionId != nil {
		data[i] = 0x10
		i++
		i = encodeVarintErrorpb(data, i, uint64(*m.RegionId))
	}
	if m.StartKey != nil {
		data[i] = 0x1a
		i++
		i = encodeVarintErrorpb(data, i, uint64(len(m.StartKey)))
		i += copy(data[i:], m.StartKey)
	}
	if m.EndKey != nil {
		data[i] = 0x22
		i++
		i = encodeVarintErrorpb(data, i, uint64(len(m.EndKey)))
		i += copy(data[i:], m.EndKey)
	}
	if m.XXX_unrecognized != nil {
		i += copy(data[i:], m.XXX_unrecognized)
	}
	return i, nil
}

func (m *StaleEpoch) Marshal() (data []byte, err error) {
	size := m.Size()
	data = make([]byte, size)
	n, err := m.MarshalTo(data)
	if err != nil {
		return nil, err
	}
	return data[:n], nil
}

func (m *StaleEpoch) MarshalTo(data []byte) (int, error) {
	var i int
	_ = i
	var l int
	_ = l
	if m.XXX_unrecognized != nil {
		i += copy(data[i:], m.XXX_unrecognized)
	}
	return i, nil
}

func (m *Error) Marshal() (data []byte, err error) {
	size := m.Size()
	data = make([]byte, size)
	n, err := m.MarshalTo(data)
	if err != nil {
		return nil, err
	}
	return data[:n], nil
}

func (m *Error) MarshalTo(data []byte) (int, error) {
	var i int
	_ = i
	var l int
	_ = l
	if m.Message != nil {
		data[i] = 0xa
		i++
		i = encodeVarintErrorpb(data, i, uint64(len(*m.Message)))
		i += copy(data[i:], *m.Message)
	}
	if m.NotLeader != nil {
		data[i] = 0x12
		i++
		i = encodeVarintErrorpb(data, i, uint64(m.NotLeader.Size()))
		n2, err := m.NotLeader.MarshalTo(data[i:])
		if err != nil {
			return 0, err
		}
		i += n2
	}
	if m.RegionNotFound != nil {
		data[i] = 0x1a
		i++
		i = encodeVarintErrorpb(data, i, uint64(m.RegionNotFound.Size()))
		n3, err := m.RegionNotFound.MarshalTo(data[i:])
		if err != nil {
			return 0, err
		}
		i += n3
	}
	if m.KeyNotInRegion != nil {
		data[i] = 0x22
		i++
		i = encodeVarintErrorpb(data, i, uint64(m.KeyNotInRegion.Size()))
		n4, err := m.KeyNotInRegion.MarshalTo(data[i:])
		if err != nil {
			return 0, err
		}
		i += n4
	}
	if m.StaleEpoch != nil {
		data[i] = 0x2a
		i++
		i = encodeVarintErrorpb(data, i, uint64(m.StaleEpoch.Size()))
		n5, err := m.StaleEpoch.MarshalTo(data[i:])
		if err != nil {
			return 0, err
		}
		i += n5
	}
	if m.XXX_unrecognized != nil {
		i += copy(data[i:], m.XXX_unrecognized)
	}
	return i, nil
}

func encodeFixed64Errorpb(data []byte, offset int, v uint64) int {
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
func encodeFixed32Errorpb(data []byte, offset int, v uint32) int {
	data[offset] = uint8(v)
	data[offset+1] = uint8(v >> 8)
	data[offset+2] = uint8(v >> 16)
	data[offset+3] = uint8(v >> 24)
	return offset + 4
}
func encodeVarintErrorpb(data []byte, offset int, v uint64) int {
	for v >= 1<<7 {
		data[offset] = uint8(v&0x7f | 0x80)
		v >>= 7
		offset++
	}
	data[offset] = uint8(v)
	return offset + 1
}
func (m *NotLeader) Size() (n int) {
	var l int
	_ = l
	if m.RegionId != nil {
		n += 1 + sovErrorpb(uint64(*m.RegionId))
	}
	if m.Leader != nil {
		l = m.Leader.Size()
		n += 1 + l + sovErrorpb(uint64(l))
	}
	if m.XXX_unrecognized != nil {
		n += len(m.XXX_unrecognized)
	}
	return n
}

func (m *RegionNotFound) Size() (n int) {
	var l int
	_ = l
	if m.RegionId != nil {
		n += 1 + sovErrorpb(uint64(*m.RegionId))
	}
	if m.XXX_unrecognized != nil {
		n += len(m.XXX_unrecognized)
	}
	return n
}

func (m *KeyNotInRegion) Size() (n int) {
	var l int
	_ = l
	if m.Key != nil {
		l = len(m.Key)
		n += 1 + l + sovErrorpb(uint64(l))
	}
	if m.RegionId != nil {
		n += 1 + sovErrorpb(uint64(*m.RegionId))
	}
	if m.StartKey != nil {
		l = len(m.StartKey)
		n += 1 + l + sovErrorpb(uint64(l))
	}
	if m.EndKey != nil {
		l = len(m.EndKey)
		n += 1 + l + sovErrorpb(uint64(l))
	}
	if m.XXX_unrecognized != nil {
		n += len(m.XXX_unrecognized)
	}
	return n
}

func (m *StaleEpoch) Size() (n int) {
	var l int
	_ = l
	if m.XXX_unrecognized != nil {
		n += len(m.XXX_unrecognized)
	}
	return n
}

func (m *Error) Size() (n int) {
	var l int
	_ = l
	if m.Message != nil {
		l = len(*m.Message)
		n += 1 + l + sovErrorpb(uint64(l))
	}
	if m.NotLeader != nil {
		l = m.NotLeader.Size()
		n += 1 + l + sovErrorpb(uint64(l))
	}
	if m.RegionNotFound != nil {
		l = m.RegionNotFound.Size()
		n += 1 + l + sovErrorpb(uint64(l))
	}
	if m.KeyNotInRegion != nil {
		l = m.KeyNotInRegion.Size()
		n += 1 + l + sovErrorpb(uint64(l))
	}
	if m.StaleEpoch != nil {
		l = m.StaleEpoch.Size()
		n += 1 + l + sovErrorpb(uint64(l))
	}
	if m.XXX_unrecognized != nil {
		n += len(m.XXX_unrecognized)
	}
	return n
}

func sovErrorpb(x uint64) (n int) {
	for {
		n++
		x >>= 7
		if x == 0 {
			break
		}
	}
	return n
}
func sozErrorpb(x uint64) (n int) {
	return sovErrorpb(uint64((x << 1) ^ uint64((int64(x) >> 63))))
}
func (m *NotLeader) Unmarshal(data []byte) error {
	l := len(data)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowErrorpb
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
			return fmt.Errorf("proto: NotLeader: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: NotLeader: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field RegionId", wireType)
			}
			var v uint64
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowErrorpb
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := data[iNdEx]
				iNdEx++
				v |= (uint64(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			m.RegionId = &v
		case 2:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field Leader", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowErrorpb
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
				return ErrInvalidLengthErrorpb
			}
			postIndex := iNdEx + msglen
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			if m.Leader == nil {
				m.Leader = &metapb.Peer{}
			}
			if err := m.Leader.Unmarshal(data[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		default:
			iNdEx = preIndex
			skippy, err := skipErrorpb(data[iNdEx:])
			if err != nil {
				return err
			}
			if skippy < 0 {
				return ErrInvalidLengthErrorpb
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
func (m *RegionNotFound) Unmarshal(data []byte) error {
	l := len(data)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowErrorpb
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
			return fmt.Errorf("proto: RegionNotFound: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: RegionNotFound: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field RegionId", wireType)
			}
			var v uint64
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowErrorpb
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := data[iNdEx]
				iNdEx++
				v |= (uint64(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			m.RegionId = &v
		default:
			iNdEx = preIndex
			skippy, err := skipErrorpb(data[iNdEx:])
			if err != nil {
				return err
			}
			if skippy < 0 {
				return ErrInvalidLengthErrorpb
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
func (m *KeyNotInRegion) Unmarshal(data []byte) error {
	l := len(data)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowErrorpb
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
			return fmt.Errorf("proto: KeyNotInRegion: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: KeyNotInRegion: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field Key", wireType)
			}
			var byteLen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowErrorpb
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := data[iNdEx]
				iNdEx++
				byteLen |= (int(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if byteLen < 0 {
				return ErrInvalidLengthErrorpb
			}
			postIndex := iNdEx + byteLen
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.Key = append(m.Key[:0], data[iNdEx:postIndex]...)
			if m.Key == nil {
				m.Key = []byte{}
			}
			iNdEx = postIndex
		case 2:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field RegionId", wireType)
			}
			var v uint64
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowErrorpb
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := data[iNdEx]
				iNdEx++
				v |= (uint64(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			m.RegionId = &v
		case 3:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field StartKey", wireType)
			}
			var byteLen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowErrorpb
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := data[iNdEx]
				iNdEx++
				byteLen |= (int(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if byteLen < 0 {
				return ErrInvalidLengthErrorpb
			}
			postIndex := iNdEx + byteLen
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.StartKey = append(m.StartKey[:0], data[iNdEx:postIndex]...)
			if m.StartKey == nil {
				m.StartKey = []byte{}
			}
			iNdEx = postIndex
		case 4:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field EndKey", wireType)
			}
			var byteLen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowErrorpb
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := data[iNdEx]
				iNdEx++
				byteLen |= (int(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if byteLen < 0 {
				return ErrInvalidLengthErrorpb
			}
			postIndex := iNdEx + byteLen
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.EndKey = append(m.EndKey[:0], data[iNdEx:postIndex]...)
			if m.EndKey == nil {
				m.EndKey = []byte{}
			}
			iNdEx = postIndex
		default:
			iNdEx = preIndex
			skippy, err := skipErrorpb(data[iNdEx:])
			if err != nil {
				return err
			}
			if skippy < 0 {
				return ErrInvalidLengthErrorpb
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
func (m *StaleEpoch) Unmarshal(data []byte) error {
	l := len(data)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowErrorpb
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
			return fmt.Errorf("proto: StaleEpoch: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: StaleEpoch: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		default:
			iNdEx = preIndex
			skippy, err := skipErrorpb(data[iNdEx:])
			if err != nil {
				return err
			}
			if skippy < 0 {
				return ErrInvalidLengthErrorpb
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
func (m *Error) Unmarshal(data []byte) error {
	l := len(data)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowErrorpb
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
			return fmt.Errorf("proto: Error: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: Error: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field Message", wireType)
			}
			var stringLen uint64
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowErrorpb
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := data[iNdEx]
				iNdEx++
				stringLen |= (uint64(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			intStringLen := int(stringLen)
			if intStringLen < 0 {
				return ErrInvalidLengthErrorpb
			}
			postIndex := iNdEx + intStringLen
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			s := string(data[iNdEx:postIndex])
			m.Message = &s
			iNdEx = postIndex
		case 2:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field NotLeader", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowErrorpb
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
				return ErrInvalidLengthErrorpb
			}
			postIndex := iNdEx + msglen
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			if m.NotLeader == nil {
				m.NotLeader = &NotLeader{}
			}
			if err := m.NotLeader.Unmarshal(data[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		case 3:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field RegionNotFound", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowErrorpb
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
				return ErrInvalidLengthErrorpb
			}
			postIndex := iNdEx + msglen
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			if m.RegionNotFound == nil {
				m.RegionNotFound = &RegionNotFound{}
			}
			if err := m.RegionNotFound.Unmarshal(data[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		case 4:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field KeyNotInRegion", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowErrorpb
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
				return ErrInvalidLengthErrorpb
			}
			postIndex := iNdEx + msglen
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			if m.KeyNotInRegion == nil {
				m.KeyNotInRegion = &KeyNotInRegion{}
			}
			if err := m.KeyNotInRegion.Unmarshal(data[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		case 5:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field StaleEpoch", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowErrorpb
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
				return ErrInvalidLengthErrorpb
			}
			postIndex := iNdEx + msglen
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			if m.StaleEpoch == nil {
				m.StaleEpoch = &StaleEpoch{}
			}
			if err := m.StaleEpoch.Unmarshal(data[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		default:
			iNdEx = preIndex
			skippy, err := skipErrorpb(data[iNdEx:])
			if err != nil {
				return err
			}
			if skippy < 0 {
				return ErrInvalidLengthErrorpb
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
func skipErrorpb(data []byte) (n int, err error) {
	l := len(data)
	iNdEx := 0
	for iNdEx < l {
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return 0, ErrIntOverflowErrorpb
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
					return 0, ErrIntOverflowErrorpb
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
					return 0, ErrIntOverflowErrorpb
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
				return 0, ErrInvalidLengthErrorpb
			}
			return iNdEx, nil
		case 3:
			for {
				var innerWire uint64
				var start int = iNdEx
				for shift := uint(0); ; shift += 7 {
					if shift >= 64 {
						return 0, ErrIntOverflowErrorpb
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
				next, err := skipErrorpb(data[start:])
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
	ErrInvalidLengthErrorpb = fmt.Errorf("proto: negative length found during unmarshaling")
	ErrIntOverflowErrorpb   = fmt.Errorf("proto: integer overflow")
)

func init() { proto.RegisterFile("errorpb.proto", fileDescriptorErrorpb) }

var fileDescriptorErrorpb = []byte{
	// 306 bytes of a gzipped FileDescriptorProto
	0x1f, 0x8b, 0x08, 0x00, 0x00, 0x09, 0x6e, 0x88, 0x02, 0xff, 0x6c, 0x90, 0xdf, 0x4a, 0xc3, 0x30,
	0x14, 0xc6, 0xed, 0xfe, 0x38, 0x7a, 0x5a, 0xe7, 0x16, 0x05, 0xcb, 0x90, 0x21, 0x15, 0x64, 0x57,
	0x15, 0x7b, 0xed, 0x95, 0x30, 0x41, 0x14, 0x11, 0xf5, 0xbe, 0x44, 0x7b, 0xac, 0xc5, 0x2d, 0x29,
	0x69, 0xbc, 0xd8, 0x9b, 0xf8, 0x48, 0x5e, 0xea, 0x1b, 0x88, 0xbe, 0x88, 0xc9, 0x69, 0xb7, 0xb9,
	0xe1, 0x45, 0x20, 0xf9, 0xf2, 0x3b, 0x5f, 0xbe, 0x2f, 0xb0, 0x85, 0x4a, 0x49, 0x55, 0x3c, 0x44,
	0x85, 0x92, 0x5a, 0xb2, 0x4e, 0x7d, 0x1c, 0xf8, 0x53, 0xd4, 0x7c, 0x2e, 0x0f, 0x76, 0x33, 0x99,
	0x49, 0xda, 0x1e, 0xdb, 0x5d, 0xa5, 0x86, 0xa7, 0xe0, 0x5e, 0x4b, 0x7d, 0x85, 0x3c, 0x45, 0xc5,
	0xfa, 0xe0, 0x2a, 0xcc, 0x72, 0x29, 0x92, 0x3c, 0x0d, 0x9c, 0x03, 0x67, 0xd4, 0x62, 0xfb, 0xb0,
	0x39, 0xa1, 0xcb, 0xa0, 0x61, 0xce, 0x5e, 0xec, 0x47, 0xb5, 0xe9, 0x0d, 0xa2, 0x0a, 0x0f, 0xa1,
	0x7b, 0x4b, 0x03, 0xc6, 0xe3, 0x5c, 0xbe, 0x8a, 0xf4, 0x1f, 0x8b, 0xf0, 0x1e, 0xba, 0x97, 0x38,
	0x33, 0xc4, 0x85, 0xa8, 0x60, 0xe6, 0x41, 0xf3, 0x05, 0x67, 0x74, 0xed, 0xaf, 0x4e, 0x34, 0xe8,
	0x51, 0x23, 0x95, 0x9a, 0x2b, 0x9d, 0x58, 0xaa, 0x49, 0xd4, 0x36, 0x74, 0x50, 0xa4, 0x24, 0xb4,
	0xac, 0x10, 0xfa, 0x00, 0x77, 0x9a, 0x4f, 0x70, 0x5c, 0xc8, 0xc7, 0xe7, 0xf0, 0xd3, 0x81, 0xf6,
	0xd8, 0xd6, 0xb6, 0xe0, 0x14, 0xcb, 0x92, 0x67, 0x48, 0xfe, 0x2e, 0x3b, 0x02, 0x10, 0x52, 0x27,
	0x2b, 0x2d, 0x58, 0x34, 0xff, 0xb2, 0x65, 0xf9, 0x13, 0xe8, 0xd5, 0x39, 0x2c, 0xfe, 0x64, 0xdb,
	0xd0, 0xdb, 0x5e, 0xbc, 0xb7, 0xa0, 0xd7, 0xca, 0xc6, 0xd0, 0x37, 0x81, 0x88, 0xcf, 0x45, 0x52,
	0x4d, 0x53, 0xbc, 0xbf, 0x33, 0x6b, 0xdd, 0x47, 0xe0, 0x95, 0x36, 0x77, 0x82, 0x36, 0x78, 0xd0,
	0x26, 0x7a, 0x67, 0x41, 0x2f, 0x3b, 0x9d, 0xf5, 0xde, 0xbf, 0x87, 0xce, 0x87, 0x59, 0x5f, 0x66,
	0xbd, 0xfd, 0x0c, 0x37, 0x7e, 0x03, 0x00, 0x00, 0xff, 0xff, 0xcd, 0xb6, 0x89, 0x2b, 0xe9, 0x01,
	0x00, 0x00,
}
