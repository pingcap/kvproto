// Code generated by protoc-gen-gogo. DO NOT EDIT.
// source: coprocessor_v2.proto

package coprocessor_v2

import (
	"fmt"
	"io"
	"math"

	proto "github.com/golang/protobuf/proto"

	_ "github.com/gogo/protobuf/gogoproto"

	errorpb "github.com/pingcap/kvproto/pkg/errorpb"

	kvrpcpb "github.com/pingcap/kvproto/pkg/kvrpcpb"
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

type RawCoprocessorRequest struct {
	Context               *kvrpcpb.Context `protobuf:"bytes,1,opt,name=context" json:"context,omitempty"`
	CoprName              string           `protobuf:"bytes,2,opt,name=copr_name,json=coprName,proto3" json:"copr_name,omitempty"`
	CoprVersionConstraint string           `protobuf:"bytes,3,opt,name=copr_version_constraint,json=coprVersionConstraint,proto3" json:"copr_version_constraint,omitempty"`
	Data                  []byte           `protobuf:"bytes,4,opt,name=data,proto3" json:"data,omitempty"`
	XXX_NoUnkeyedLiteral  struct{}         `json:"-"`
	XXX_unrecognized      []byte           `json:"-"`
	XXX_sizecache         int32            `json:"-"`
}

func (m *RawCoprocessorRequest) Reset()         { *m = RawCoprocessorRequest{} }
func (m *RawCoprocessorRequest) String() string { return proto.CompactTextString(m) }
func (*RawCoprocessorRequest) ProtoMessage()    {}
func (*RawCoprocessorRequest) Descriptor() ([]byte, []int) {
	return fileDescriptor_coprocessor_v2_413f6c45cdad6f3b, []int{0}
}
func (m *RawCoprocessorRequest) XXX_Unmarshal(b []byte) error {
	return m.Unmarshal(b)
}
func (m *RawCoprocessorRequest) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	if deterministic {
		return xxx_messageInfo_RawCoprocessorRequest.Marshal(b, m, deterministic)
	} else {
		b = b[:cap(b)]
		n, err := m.MarshalTo(b)
		if err != nil {
			return nil, err
		}
		return b[:n], nil
	}
}
func (dst *RawCoprocessorRequest) XXX_Merge(src proto.Message) {
	xxx_messageInfo_RawCoprocessorRequest.Merge(dst, src)
}
func (m *RawCoprocessorRequest) XXX_Size() int {
	return m.Size()
}
func (m *RawCoprocessorRequest) XXX_DiscardUnknown() {
	xxx_messageInfo_RawCoprocessorRequest.DiscardUnknown(m)
}

var xxx_messageInfo_RawCoprocessorRequest proto.InternalMessageInfo

func (m *RawCoprocessorRequest) GetContext() *kvrpcpb.Context {
	if m != nil {
		return m.Context
	}
	return nil
}

func (m *RawCoprocessorRequest) GetCoprName() string {
	if m != nil {
		return m.CoprName
	}
	return ""
}

func (m *RawCoprocessorRequest) GetCoprVersionConstraint() string {
	if m != nil {
		return m.CoprVersionConstraint
	}
	return ""
}

func (m *RawCoprocessorRequest) GetData() []byte {
	if m != nil {
		return m.Data
	}
	return nil
}

type RawCoprocessorResponse struct {
	Data        []byte         `protobuf:"bytes,1,opt,name=data,proto3" json:"data,omitempty"`
	RegionError *errorpb.Error `protobuf:"bytes,2,opt,name=region_error,json=regionError" json:"region_error,omitempty"`
	// Error message for cases like if no coprocessor with a matching name is found
	// or on a version mismatch between plugin_api and the coprocessor.
	OtherError           string   `protobuf:"bytes,4,opt,name=other_error,json=otherError,proto3" json:"other_error,omitempty"`
	XXX_NoUnkeyedLiteral struct{} `json:"-"`
	XXX_unrecognized     []byte   `json:"-"`
	XXX_sizecache        int32    `json:"-"`
}

func (m *RawCoprocessorResponse) Reset()         { *m = RawCoprocessorResponse{} }
func (m *RawCoprocessorResponse) String() string { return proto.CompactTextString(m) }
func (*RawCoprocessorResponse) ProtoMessage()    {}
func (*RawCoprocessorResponse) Descriptor() ([]byte, []int) {
	return fileDescriptor_coprocessor_v2_413f6c45cdad6f3b, []int{1}
}
func (m *RawCoprocessorResponse) XXX_Unmarshal(b []byte) error {
	return m.Unmarshal(b)
}
func (m *RawCoprocessorResponse) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	if deterministic {
		return xxx_messageInfo_RawCoprocessorResponse.Marshal(b, m, deterministic)
	} else {
		b = b[:cap(b)]
		n, err := m.MarshalTo(b)
		if err != nil {
			return nil, err
		}
		return b[:n], nil
	}
}
func (dst *RawCoprocessorResponse) XXX_Merge(src proto.Message) {
	xxx_messageInfo_RawCoprocessorResponse.Merge(dst, src)
}
func (m *RawCoprocessorResponse) XXX_Size() int {
	return m.Size()
}
func (m *RawCoprocessorResponse) XXX_DiscardUnknown() {
	xxx_messageInfo_RawCoprocessorResponse.DiscardUnknown(m)
}

var xxx_messageInfo_RawCoprocessorResponse proto.InternalMessageInfo

func (m *RawCoprocessorResponse) GetData() []byte {
	if m != nil {
		return m.Data
	}
	return nil
}

func (m *RawCoprocessorResponse) GetRegionError() *errorpb.Error {
	if m != nil {
		return m.RegionError
	}
	return nil
}

func (m *RawCoprocessorResponse) GetOtherError() string {
	if m != nil {
		return m.OtherError
	}
	return ""
}

func init() {
	proto.RegisterType((*RawCoprocessorRequest)(nil), "coprocessor_v2.RawCoprocessorRequest")
	proto.RegisterType((*RawCoprocessorResponse)(nil), "coprocessor_v2.RawCoprocessorResponse")
}
func (m *RawCoprocessorRequest) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalTo(dAtA)
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *RawCoprocessorRequest) MarshalTo(dAtA []byte) (int, error) {
	var i int
	_ = i
	var l int
	_ = l
	if m.Context != nil {
		dAtA[i] = 0xa
		i++
		i = encodeVarintCoprocessorV2(dAtA, i, uint64(m.Context.Size()))
		n1, err := m.Context.MarshalTo(dAtA[i:])
		if err != nil {
			return 0, err
		}
		i += n1
	}
	if len(m.CoprName) > 0 {
		dAtA[i] = 0x12
		i++
		i = encodeVarintCoprocessorV2(dAtA, i, uint64(len(m.CoprName)))
		i += copy(dAtA[i:], m.CoprName)
	}
	if len(m.CoprVersionConstraint) > 0 {
		dAtA[i] = 0x1a
		i++
		i = encodeVarintCoprocessorV2(dAtA, i, uint64(len(m.CoprVersionConstraint)))
		i += copy(dAtA[i:], m.CoprVersionConstraint)
	}
	if len(m.Data) > 0 {
		dAtA[i] = 0x22
		i++
		i = encodeVarintCoprocessorV2(dAtA, i, uint64(len(m.Data)))
		i += copy(dAtA[i:], m.Data)
	}
	if m.XXX_unrecognized != nil {
		i += copy(dAtA[i:], m.XXX_unrecognized)
	}
	return i, nil
}

func (m *RawCoprocessorResponse) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalTo(dAtA)
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *RawCoprocessorResponse) MarshalTo(dAtA []byte) (int, error) {
	var i int
	_ = i
	var l int
	_ = l
	if len(m.Data) > 0 {
		dAtA[i] = 0xa
		i++
		i = encodeVarintCoprocessorV2(dAtA, i, uint64(len(m.Data)))
		i += copy(dAtA[i:], m.Data)
	}
	if m.RegionError != nil {
		dAtA[i] = 0x12
		i++
		i = encodeVarintCoprocessorV2(dAtA, i, uint64(m.RegionError.Size()))
		n2, err := m.RegionError.MarshalTo(dAtA[i:])
		if err != nil {
			return 0, err
		}
		i += n2
	}
	if len(m.OtherError) > 0 {
		dAtA[i] = 0x22
		i++
		i = encodeVarintCoprocessorV2(dAtA, i, uint64(len(m.OtherError)))
		i += copy(dAtA[i:], m.OtherError)
	}
	if m.XXX_unrecognized != nil {
		i += copy(dAtA[i:], m.XXX_unrecognized)
	}
	return i, nil
}

func encodeVarintCoprocessorV2(dAtA []byte, offset int, v uint64) int {
	for v >= 1<<7 {
		dAtA[offset] = uint8(v&0x7f | 0x80)
		v >>= 7
		offset++
	}
	dAtA[offset] = uint8(v)
	return offset + 1
}
func (m *RawCoprocessorRequest) Size() (n int) {
	var l int
	_ = l
	if m.Context != nil {
		l = m.Context.Size()
		n += 1 + l + sovCoprocessorV2(uint64(l))
	}
	l = len(m.CoprName)
	if l > 0 {
		n += 1 + l + sovCoprocessorV2(uint64(l))
	}
	l = len(m.CoprVersionConstraint)
	if l > 0 {
		n += 1 + l + sovCoprocessorV2(uint64(l))
	}
	l = len(m.Data)
	if l > 0 {
		n += 1 + l + sovCoprocessorV2(uint64(l))
	}
	if m.XXX_unrecognized != nil {
		n += len(m.XXX_unrecognized)
	}
	return n
}

func (m *RawCoprocessorResponse) Size() (n int) {
	var l int
	_ = l
	l = len(m.Data)
	if l > 0 {
		n += 1 + l + sovCoprocessorV2(uint64(l))
	}
	if m.RegionError != nil {
		l = m.RegionError.Size()
		n += 1 + l + sovCoprocessorV2(uint64(l))
	}
	l = len(m.OtherError)
	if l > 0 {
		n += 1 + l + sovCoprocessorV2(uint64(l))
	}
	if m.XXX_unrecognized != nil {
		n += len(m.XXX_unrecognized)
	}
	return n
}

func sovCoprocessorV2(x uint64) (n int) {
	for {
		n++
		x >>= 7
		if x == 0 {
			break
		}
	}
	return n
}
func sozCoprocessorV2(x uint64) (n int) {
	return sovCoprocessorV2(uint64((x << 1) ^ uint64((int64(x) >> 63))))
}
func (m *RawCoprocessorRequest) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowCoprocessorV2
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
			return fmt.Errorf("proto: RawCoprocessorRequest: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: RawCoprocessorRequest: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field Context", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowCoprocessorV2
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
				return ErrInvalidLengthCoprocessorV2
			}
			postIndex := iNdEx + msglen
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			if m.Context == nil {
				m.Context = &kvrpcpb.Context{}
			}
			if err := m.Context.Unmarshal(dAtA[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		case 2:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field CoprName", wireType)
			}
			var stringLen uint64
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowCoprocessorV2
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				stringLen |= (uint64(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			intStringLen := int(stringLen)
			if intStringLen < 0 {
				return ErrInvalidLengthCoprocessorV2
			}
			postIndex := iNdEx + intStringLen
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.CoprName = string(dAtA[iNdEx:postIndex])
			iNdEx = postIndex
		case 3:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field CoprVersionConstraint", wireType)
			}
			var stringLen uint64
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowCoprocessorV2
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				stringLen |= (uint64(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			intStringLen := int(stringLen)
			if intStringLen < 0 {
				return ErrInvalidLengthCoprocessorV2
			}
			postIndex := iNdEx + intStringLen
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.CoprVersionConstraint = string(dAtA[iNdEx:postIndex])
			iNdEx = postIndex
		case 4:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field Data", wireType)
			}
			var byteLen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowCoprocessorV2
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
				return ErrInvalidLengthCoprocessorV2
			}
			postIndex := iNdEx + byteLen
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.Data = append(m.Data[:0], dAtA[iNdEx:postIndex]...)
			if m.Data == nil {
				m.Data = []byte{}
			}
			iNdEx = postIndex
		default:
			iNdEx = preIndex
			skippy, err := skipCoprocessorV2(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if skippy < 0 {
				return ErrInvalidLengthCoprocessorV2
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
func (m *RawCoprocessorResponse) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowCoprocessorV2
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
			return fmt.Errorf("proto: RawCoprocessorResponse: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: RawCoprocessorResponse: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field Data", wireType)
			}
			var byteLen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowCoprocessorV2
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
				return ErrInvalidLengthCoprocessorV2
			}
			postIndex := iNdEx + byteLen
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.Data = append(m.Data[:0], dAtA[iNdEx:postIndex]...)
			if m.Data == nil {
				m.Data = []byte{}
			}
			iNdEx = postIndex
		case 2:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field RegionError", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowCoprocessorV2
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
				return ErrInvalidLengthCoprocessorV2
			}
			postIndex := iNdEx + msglen
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			if m.RegionError == nil {
				m.RegionError = &errorpb.Error{}
			}
			if err := m.RegionError.Unmarshal(dAtA[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		case 4:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field OtherError", wireType)
			}
			var stringLen uint64
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowCoprocessorV2
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				stringLen |= (uint64(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			intStringLen := int(stringLen)
			if intStringLen < 0 {
				return ErrInvalidLengthCoprocessorV2
			}
			postIndex := iNdEx + intStringLen
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.OtherError = string(dAtA[iNdEx:postIndex])
			iNdEx = postIndex
		default:
			iNdEx = preIndex
			skippy, err := skipCoprocessorV2(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if skippy < 0 {
				return ErrInvalidLengthCoprocessorV2
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
func skipCoprocessorV2(dAtA []byte) (n int, err error) {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return 0, ErrIntOverflowCoprocessorV2
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
					return 0, ErrIntOverflowCoprocessorV2
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
					return 0, ErrIntOverflowCoprocessorV2
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
				return 0, ErrInvalidLengthCoprocessorV2
			}
			return iNdEx, nil
		case 3:
			for {
				var innerWire uint64
				var start int = iNdEx
				for shift := uint(0); ; shift += 7 {
					if shift >= 64 {
						return 0, ErrIntOverflowCoprocessorV2
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
				next, err := skipCoprocessorV2(dAtA[start:])
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
	ErrInvalidLengthCoprocessorV2 = fmt.Errorf("proto: negative length found during unmarshaling")
	ErrIntOverflowCoprocessorV2   = fmt.Errorf("proto: integer overflow")
)

func init() {
	proto.RegisterFile("coprocessor_v2.proto", fileDescriptor_coprocessor_v2_413f6c45cdad6f3b)
}

var fileDescriptor_coprocessor_v2_413f6c45cdad6f3b = []byte{
	// 316 bytes of a gzipped FileDescriptorProto
	0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0xff, 0x5c, 0x50, 0xb1, 0x4e, 0x2a, 0x41,
	0x14, 0x7d, 0xf3, 0x24, 0x0a, 0xb3, 0x80, 0x64, 0x02, 0xba, 0xc1, 0x64, 0x25, 0x14, 0x86, 0x58,
	0xac, 0x11, 0x13, 0x3f, 0x40, 0x62, 0x6b, 0x31, 0x85, 0x2d, 0x19, 0xd6, 0xc9, 0x4a, 0x08, 0x73,
	0xd7, 0x3b, 0xc3, 0x6a, 0xe9, 0x67, 0xf8, 0x09, 0x16, 0x7e, 0x88, 0xa5, 0xa5, 0xa5, 0xc1, 0x1f,
	0x31, 0x73, 0x97, 0x5d, 0xa2, 0xd5, 0x3d, 0xf7, 0x9c, 0x33, 0x73, 0x4f, 0x0e, 0xef, 0x26, 0x90,
	0x21, 0x24, 0xda, 0x5a, 0xc0, 0x69, 0x3e, 0x8e, 0x33, 0x04, 0x07, 0xa2, 0xfd, 0x9b, 0xed, 0xb7,
	0x34, 0x22, 0x60, 0x36, 0x2b, 0xe4, 0x7e, 0x6b, 0x91, 0x63, 0x96, 0x54, 0x6b, 0x37, 0x85, 0x14,
	0x08, 0x9e, 0x79, 0xb4, 0x61, 0xf7, 0x71, 0x65, 0x1d, 0xc1, 0x82, 0x18, 0xbe, 0x31, 0xde, 0x93,
	0xea, 0x71, 0xb2, 0xfd, 0x5a, 0xea, 0x87, 0x95, 0xb6, 0x4e, 0x9c, 0xf2, 0xbd, 0x04, 0x8c, 0xd3,
	0x4f, 0x2e, 0x64, 0x03, 0x36, 0x0a, 0xc6, 0x9d, 0xb8, 0xbc, 0x30, 0x29, 0x78, 0x59, 0x1a, 0xc4,
	0x11, 0x6f, 0xf8, 0x70, 0x53, 0xa3, 0x96, 0x3a, 0xfc, 0x3f, 0x60, 0xa3, 0x86, 0xac, 0x7b, 0xe2,
	0x46, 0x2d, 0xb5, 0xb8, 0xe4, 0x87, 0x24, 0xe6, 0x1a, 0xed, 0x1c, 0xcc, 0x34, 0x01, 0x63, 0x1d,
	0xaa, 0xb9, 0x71, 0xe1, 0x0e, 0x59, 0x7b, 0x5e, 0xbe, 0x2d, 0xd4, 0x49, 0x25, 0x0a, 0xc1, 0x6b,
	0x77, 0xca, 0xa9, 0xb0, 0x36, 0x60, 0xa3, 0xa6, 0x24, 0x3c, 0x7c, 0x66, 0xfc, 0xe0, 0x6f, 0x5c,
	0x9b, 0x81, 0xb1, 0xba, 0xb2, 0xb3, 0xad, 0x5d, 0x9c, 0xf3, 0x26, 0xea, 0xd4, 0x1f, 0xa5, 0xae,
	0x28, 0x5a, 0x30, 0x6e, 0xc7, 0x65, 0x73, 0xd7, 0x7e, 0xca, 0xa0, 0xf0, 0xd0, 0x22, 0x8e, 0x79,
	0x00, 0xee, 0x5e, 0xe3, 0xe6, 0x45, 0x8d, 0x12, 0x72, 0xa2, 0xc8, 0x70, 0x75, 0xf2, 0xf9, 0x5a,
	0x67, 0xef, 0xeb, 0x88, 0x7d, 0xac, 0x23, 0xf6, 0xb5, 0x8e, 0xd8, 0xcb, 0x77, 0xf4, 0x8f, 0x77,
	0x00, 0xd3, 0xd8, 0xcd, 0x17, 0x79, 0xbc, 0xc8, 0xa9, 0xd9, 0xd9, 0x2e, 0x8d, 0x8b, 0x9f, 0x00,
	0x00, 0x00, 0xff, 0xff, 0xa9, 0x68, 0x86, 0xb6, 0xcd, 0x01, 0x00, 0x00,
}
