// Code generated by protoc-gen-gogo. DO NOT EDIT.
// source: coprocessor.proto

package coprocessor

import (
	"fmt"
	"io"
	"math"

	proto "github.com/golang/protobuf/proto"

	_ "github.com/gogo/protobuf/gogoproto"

	errorpb "github.com/pingcap/kvproto/pkg/errorpb"

	kvrpcpb "github.com/pingcap/kvproto/pkg/kvrpcpb"

	github_com_pingcap_kvproto_pkg_sharedbytes "github.com/pingcap/kvproto/pkg/sharedbytes"
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

// [start, end)
type KeyRange struct {
	Start                []byte   `protobuf:"bytes,1,opt,name=start,proto3" json:"start,omitempty"`
	End                  []byte   `protobuf:"bytes,2,opt,name=end,proto3" json:"end,omitempty"`
	XXX_NoUnkeyedLiteral struct{} `json:"-"`
	XXX_unrecognized     []byte   `json:"-"`
	XXX_sizecache        int32    `json:"-"`
}

func (m *KeyRange) Reset()         { *m = KeyRange{} }
func (m *KeyRange) String() string { return proto.CompactTextString(m) }
func (*KeyRange) ProtoMessage()    {}
func (*KeyRange) Descriptor() ([]byte, []int) {
	return fileDescriptor_coprocessor_f145b8940f787427, []int{0}
}
func (m *KeyRange) XXX_Unmarshal(b []byte) error {
	return m.Unmarshal(b)
}
func (m *KeyRange) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	if deterministic {
		return xxx_messageInfo_KeyRange.Marshal(b, m, deterministic)
	} else {
		b = b[:cap(b)]
		n, err := m.MarshalTo(b)
		if err != nil {
			return nil, err
		}
		return b[:n], nil
	}
}
func (dst *KeyRange) XXX_Merge(src proto.Message) {
	xxx_messageInfo_KeyRange.Merge(dst, src)
}
func (m *KeyRange) XXX_Size() int {
	return m.Size()
}
func (m *KeyRange) XXX_DiscardUnknown() {
	xxx_messageInfo_KeyRange.DiscardUnknown(m)
}

var xxx_messageInfo_KeyRange proto.InternalMessageInfo

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
	Context *kvrpcpb.Context `protobuf:"bytes,1,opt,name=context" json:"context,omitempty"`
	Tp      int64            `protobuf:"varint,2,opt,name=tp,proto3" json:"tp,omitempty"`
	Data    []byte           `protobuf:"bytes,3,opt,name=data,proto3" json:"data,omitempty"`
	StartTs uint64           `protobuf:"varint,7,opt,name=start_ts,json=startTs,proto3" json:"start_ts,omitempty"`
	Ranges  []*KeyRange      `protobuf:"bytes,4,rep,name=ranges" json:"ranges,omitempty"`
	// If cache is enabled, TiKV returns cache hit instead of data if
	// its last version matches this `cache_if_match_version`.
	IsCacheEnabled      bool   `protobuf:"varint,5,opt,name=is_cache_enabled,json=isCacheEnabled,proto3" json:"is_cache_enabled,omitempty"`
	CacheIfMatchVersion uint64 `protobuf:"varint,6,opt,name=cache_if_match_version,json=cacheIfMatchVersion,proto3" json:"cache_if_match_version,omitempty"`
	// schema_ver is to check whether ddl schema is out of date.
	SchemaVer            int64    `protobuf:"varint,8,opt,name=schema_ver,json=schemaVer,proto3" json:"schema_ver,omitempty"`
	XXX_NoUnkeyedLiteral struct{} `json:"-"`
	XXX_unrecognized     []byte   `json:"-"`
	XXX_sizecache        int32    `json:"-"`
}

func (m *Request) Reset()         { *m = Request{} }
func (m *Request) String() string { return proto.CompactTextString(m) }
func (*Request) ProtoMessage()    {}
func (*Request) Descriptor() ([]byte, []int) {
	return fileDescriptor_coprocessor_f145b8940f787427, []int{1}
}
func (m *Request) XXX_Unmarshal(b []byte) error {
	return m.Unmarshal(b)
}
func (m *Request) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	if deterministic {
		return xxx_messageInfo_Request.Marshal(b, m, deterministic)
	} else {
		b = b[:cap(b)]
		n, err := m.MarshalTo(b)
		if err != nil {
			return nil, err
		}
		return b[:n], nil
	}
}
func (dst *Request) XXX_Merge(src proto.Message) {
	xxx_messageInfo_Request.Merge(dst, src)
}
func (m *Request) XXX_Size() int {
	return m.Size()
}
func (m *Request) XXX_DiscardUnknown() {
	xxx_messageInfo_Request.DiscardUnknown(m)
}

var xxx_messageInfo_Request proto.InternalMessageInfo

func (m *Request) GetContext() *kvrpcpb.Context {
	if m != nil {
		return m.Context
	}
	return nil
}

func (m *Request) GetTp() int64 {
	if m != nil {
		return m.Tp
	}
	return 0
}

func (m *Request) GetData() []byte {
	if m != nil {
		return m.Data
	}
	return nil
}

func (m *Request) GetStartTs() uint64 {
	if m != nil {
		return m.StartTs
	}
	return 0
}

func (m *Request) GetRanges() []*KeyRange {
	if m != nil {
		return m.Ranges
	}
	return nil
}

func (m *Request) GetIsCacheEnabled() bool {
	if m != nil {
		return m.IsCacheEnabled
	}
	return false
}

func (m *Request) GetCacheIfMatchVersion() uint64 {
	if m != nil {
		return m.CacheIfMatchVersion
	}
	return 0
}

func (m *Request) GetSchemaVer() int64 {
	if m != nil {
		return m.SchemaVer
	}
	return 0
}

type Response struct {
	Data                 github_com_pingcap_kvproto_pkg_sharedbytes.SharedBytes `protobuf:"bytes,1,opt,name=data,proto3,customtype=github.com/pingcap/kvproto/pkg/sharedbytes.SharedBytes" json:"data"`
	RegionError          *errorpb.Error                                         `protobuf:"bytes,2,opt,name=region_error,json=regionError" json:"region_error,omitempty"`
	Locked               *kvrpcpb.LockInfo                                      `protobuf:"bytes,3,opt,name=locked" json:"locked,omitempty"`
	OtherError           string                                                 `protobuf:"bytes,4,opt,name=other_error,json=otherError,proto3" json:"other_error,omitempty"`
	Range                *KeyRange                                              `protobuf:"bytes,5,opt,name=range" json:"range,omitempty"`
	ExecDetails          *kvrpcpb.ExecDetails                                   `protobuf:"bytes,6,opt,name=exec_details,json=execDetails" json:"exec_details,omitempty"`
	IsCacheHit           bool                                                   `protobuf:"varint,7,opt,name=is_cache_hit,json=isCacheHit,proto3" json:"is_cache_hit,omitempty"`
	CacheLastVersion     uint64                                                 `protobuf:"varint,8,opt,name=cache_last_version,json=cacheLastVersion,proto3" json:"cache_last_version,omitempty"`
	XXX_NoUnkeyedLiteral struct{}                                               `json:"-"`
	XXX_unrecognized     []byte                                                 `json:"-"`
	XXX_sizecache        int32                                                  `json:"-"`
}

func (m *Response) Reset()         { *m = Response{} }
func (m *Response) String() string { return proto.CompactTextString(m) }
func (*Response) ProtoMessage()    {}
func (*Response) Descriptor() ([]byte, []int) {
	return fileDescriptor_coprocessor_f145b8940f787427, []int{2}
}
func (m *Response) XXX_Unmarshal(b []byte) error {
	return m.Unmarshal(b)
}
func (m *Response) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	if deterministic {
		return xxx_messageInfo_Response.Marshal(b, m, deterministic)
	} else {
		b = b[:cap(b)]
		n, err := m.MarshalTo(b)
		if err != nil {
			return nil, err
		}
		return b[:n], nil
	}
}
func (dst *Response) XXX_Merge(src proto.Message) {
	xxx_messageInfo_Response.Merge(dst, src)
}
func (m *Response) XXX_Size() int {
	return m.Size()
}
func (m *Response) XXX_DiscardUnknown() {
	xxx_messageInfo_Response.DiscardUnknown(m)
}

var xxx_messageInfo_Response proto.InternalMessageInfo

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
	if m != nil {
		return m.OtherError
	}
	return ""
}

func (m *Response) GetRange() *KeyRange {
	if m != nil {
		return m.Range
	}
	return nil
}

func (m *Response) GetExecDetails() *kvrpcpb.ExecDetails {
	if m != nil {
		return m.ExecDetails
	}
	return nil
}

func (m *Response) GetIsCacheHit() bool {
	if m != nil {
		return m.IsCacheHit
	}
	return false
}

func (m *Response) GetCacheLastVersion() uint64 {
	if m != nil {
		return m.CacheLastVersion
	}
	return 0
}

func init() {
	proto.RegisterType((*KeyRange)(nil), "coprocessor.KeyRange")
	proto.RegisterType((*Request)(nil), "coprocessor.Request")
	proto.RegisterType((*Response)(nil), "coprocessor.Response")
}
func (m *KeyRange) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalTo(dAtA)
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *KeyRange) MarshalTo(dAtA []byte) (int, error) {
	var i int
	_ = i
	var l int
	_ = l
	if len(m.Start) > 0 {
		dAtA[i] = 0xa
		i++
		i = encodeVarintCoprocessor(dAtA, i, uint64(len(m.Start)))
		i += copy(dAtA[i:], m.Start)
	}
	if len(m.End) > 0 {
		dAtA[i] = 0x12
		i++
		i = encodeVarintCoprocessor(dAtA, i, uint64(len(m.End)))
		i += copy(dAtA[i:], m.End)
	}
	if m.XXX_unrecognized != nil {
		i += copy(dAtA[i:], m.XXX_unrecognized)
	}
	return i, nil
}

func (m *Request) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalTo(dAtA)
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *Request) MarshalTo(dAtA []byte) (int, error) {
	var i int
	_ = i
	var l int
	_ = l
	if m.Context != nil {
		dAtA[i] = 0xa
		i++
		i = encodeVarintCoprocessor(dAtA, i, uint64(m.Context.Size()))
		n1, err := m.Context.MarshalTo(dAtA[i:])
		if err != nil {
			return 0, err
		}
		i += n1
	}
	if m.Tp != 0 {
		dAtA[i] = 0x10
		i++
		i = encodeVarintCoprocessor(dAtA, i, uint64(m.Tp))
	}
	if len(m.Data) > 0 {
		dAtA[i] = 0x1a
		i++
		i = encodeVarintCoprocessor(dAtA, i, uint64(len(m.Data)))
		i += copy(dAtA[i:], m.Data)
	}
	if len(m.Ranges) > 0 {
		for _, msg := range m.Ranges {
			dAtA[i] = 0x22
			i++
			i = encodeVarintCoprocessor(dAtA, i, uint64(msg.Size()))
			n, err := msg.MarshalTo(dAtA[i:])
			if err != nil {
				return 0, err
			}
			i += n
		}
	}
	if m.IsCacheEnabled {
		dAtA[i] = 0x28
		i++
		if m.IsCacheEnabled {
			dAtA[i] = 1
		} else {
			dAtA[i] = 0
		}
		i++
	}
	if m.CacheIfMatchVersion != 0 {
		dAtA[i] = 0x30
		i++
		i = encodeVarintCoprocessor(dAtA, i, uint64(m.CacheIfMatchVersion))
	}
	if m.StartTs != 0 {
		dAtA[i] = 0x38
		i++
		i = encodeVarintCoprocessor(dAtA, i, uint64(m.StartTs))
	}
	if m.SchemaVer != 0 {
		dAtA[i] = 0x40
		i++
		i = encodeVarintCoprocessor(dAtA, i, uint64(m.SchemaVer))
	}
	if m.XXX_unrecognized != nil {
		i += copy(dAtA[i:], m.XXX_unrecognized)
	}
	return i, nil
}

func (m *Response) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalTo(dAtA)
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *Response) MarshalTo(dAtA []byte) (int, error) {
	var i int
	_ = i
	var l int
	_ = l
	dAtA[i] = 0xa
	i++
	i = encodeVarintCoprocessor(dAtA, i, uint64(m.Data.Size()))
	n2, err := m.Data.MarshalTo(dAtA[i:])
	if err != nil {
		return 0, err
	}
	i += n2
	if m.RegionError != nil {
		dAtA[i] = 0x12
		i++
		i = encodeVarintCoprocessor(dAtA, i, uint64(m.RegionError.Size()))
		n3, err := m.RegionError.MarshalTo(dAtA[i:])
		if err != nil {
			return 0, err
		}
		i += n3
	}
	if m.Locked != nil {
		dAtA[i] = 0x1a
		i++
		i = encodeVarintCoprocessor(dAtA, i, uint64(m.Locked.Size()))
		n4, err := m.Locked.MarshalTo(dAtA[i:])
		if err != nil {
			return 0, err
		}
		i += n4
	}
	if len(m.OtherError) > 0 {
		dAtA[i] = 0x22
		i++
		i = encodeVarintCoprocessor(dAtA, i, uint64(len(m.OtherError)))
		i += copy(dAtA[i:], m.OtherError)
	}
	if m.Range != nil {
		dAtA[i] = 0x2a
		i++
		i = encodeVarintCoprocessor(dAtA, i, uint64(m.Range.Size()))
		n5, err := m.Range.MarshalTo(dAtA[i:])
		if err != nil {
			return 0, err
		}
		i += n5
	}
	if m.ExecDetails != nil {
		dAtA[i] = 0x32
		i++
		i = encodeVarintCoprocessor(dAtA, i, uint64(m.ExecDetails.Size()))
		n6, err := m.ExecDetails.MarshalTo(dAtA[i:])
		if err != nil {
			return 0, err
		}
		i += n6
	}
	if m.IsCacheHit {
		dAtA[i] = 0x38
		i++
		if m.IsCacheHit {
			dAtA[i] = 1
		} else {
			dAtA[i] = 0
		}
		i++
	}
	if m.CacheLastVersion != 0 {
		dAtA[i] = 0x40
		i++
		i = encodeVarintCoprocessor(dAtA, i, uint64(m.CacheLastVersion))
	}
	if m.XXX_unrecognized != nil {
		i += copy(dAtA[i:], m.XXX_unrecognized)
	}
	return i, nil
}

func encodeVarintCoprocessor(dAtA []byte, offset int, v uint64) int {
	for v >= 1<<7 {
		dAtA[offset] = uint8(v&0x7f | 0x80)
		v >>= 7
		offset++
	}
	dAtA[offset] = uint8(v)
	return offset + 1
}
func (m *KeyRange) Size() (n int) {
	var l int
	_ = l
	l = len(m.Start)
	if l > 0 {
		n += 1 + l + sovCoprocessor(uint64(l))
	}
	l = len(m.End)
	if l > 0 {
		n += 1 + l + sovCoprocessor(uint64(l))
	}
	if m.XXX_unrecognized != nil {
		n += len(m.XXX_unrecognized)
	}
	return n
}

func (m *Request) Size() (n int) {
	var l int
	_ = l
	if m.Context != nil {
		l = m.Context.Size()
		n += 1 + l + sovCoprocessor(uint64(l))
	}
	if m.Tp != 0 {
		n += 1 + sovCoprocessor(uint64(m.Tp))
	}
	l = len(m.Data)
	if l > 0 {
		n += 1 + l + sovCoprocessor(uint64(l))
	}
	if len(m.Ranges) > 0 {
		for _, e := range m.Ranges {
			l = e.Size()
			n += 1 + l + sovCoprocessor(uint64(l))
		}
	}
	if m.IsCacheEnabled {
		n += 2
	}
	if m.CacheIfMatchVersion != 0 {
		n += 1 + sovCoprocessor(uint64(m.CacheIfMatchVersion))
	}
	if m.StartTs != 0 {
		n += 1 + sovCoprocessor(uint64(m.StartTs))
	}
	if m.SchemaVer != 0 {
		n += 1 + sovCoprocessor(uint64(m.SchemaVer))
	}
	if m.XXX_unrecognized != nil {
		n += len(m.XXX_unrecognized)
	}
	return n
}

func (m *Response) Size() (n int) {
	var l int
	_ = l
	l = m.Data.Size()
	n += 1 + l + sovCoprocessor(uint64(l))
	if m.RegionError != nil {
		l = m.RegionError.Size()
		n += 1 + l + sovCoprocessor(uint64(l))
	}
	if m.Locked != nil {
		l = m.Locked.Size()
		n += 1 + l + sovCoprocessor(uint64(l))
	}
	l = len(m.OtherError)
	if l > 0 {
		n += 1 + l + sovCoprocessor(uint64(l))
	}
	if m.Range != nil {
		l = m.Range.Size()
		n += 1 + l + sovCoprocessor(uint64(l))
	}
	if m.ExecDetails != nil {
		l = m.ExecDetails.Size()
		n += 1 + l + sovCoprocessor(uint64(l))
	}
	if m.IsCacheHit {
		n += 2
	}
	if m.CacheLastVersion != 0 {
		n += 1 + sovCoprocessor(uint64(m.CacheLastVersion))
	}
	if m.XXX_unrecognized != nil {
		n += len(m.XXX_unrecognized)
	}
	return n
}

func sovCoprocessor(x uint64) (n int) {
	for {
		n++
		x >>= 7
		if x == 0 {
			break
		}
	}
	return n
}
func sozCoprocessor(x uint64) (n int) {
	return sovCoprocessor(uint64((x << 1) ^ uint64((int64(x) >> 63))))
}
func (m *KeyRange) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowCoprocessor
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
			return fmt.Errorf("proto: KeyRange: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: KeyRange: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field Start", wireType)
			}
			var byteLen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowCoprocessor
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
				return ErrInvalidLengthCoprocessor
			}
			postIndex := iNdEx + byteLen
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.Start = append(m.Start[:0], dAtA[iNdEx:postIndex]...)
			if m.Start == nil {
				m.Start = []byte{}
			}
			iNdEx = postIndex
		case 2:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field End", wireType)
			}
			var byteLen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowCoprocessor
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
				return ErrInvalidLengthCoprocessor
			}
			postIndex := iNdEx + byteLen
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.End = append(m.End[:0], dAtA[iNdEx:postIndex]...)
			if m.End == nil {
				m.End = []byte{}
			}
			iNdEx = postIndex
		default:
			iNdEx = preIndex
			skippy, err := skipCoprocessor(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if skippy < 0 {
				return ErrInvalidLengthCoprocessor
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
func (m *Request) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowCoprocessor
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
			return fmt.Errorf("proto: Request: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: Request: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field Context", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowCoprocessor
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
				return ErrInvalidLengthCoprocessor
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
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field Tp", wireType)
			}
			m.Tp = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowCoprocessor
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				m.Tp |= (int64(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		case 3:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field Data", wireType)
			}
			var byteLen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowCoprocessor
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
				return ErrInvalidLengthCoprocessor
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
		case 4:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field Ranges", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowCoprocessor
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
				return ErrInvalidLengthCoprocessor
			}
			postIndex := iNdEx + msglen
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.Ranges = append(m.Ranges, &KeyRange{})
			if err := m.Ranges[len(m.Ranges)-1].Unmarshal(dAtA[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		case 5:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field IsCacheEnabled", wireType)
			}
			var v int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowCoprocessor
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				v |= (int(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			m.IsCacheEnabled = bool(v != 0)
		case 6:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field CacheIfMatchVersion", wireType)
			}
			m.CacheIfMatchVersion = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowCoprocessor
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				m.CacheIfMatchVersion |= (uint64(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		case 7:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field StartTs", wireType)
			}
			m.StartTs = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowCoprocessor
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				m.StartTs |= (uint64(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		case 8:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field SchemaVer", wireType)
			}
			m.SchemaVer = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowCoprocessor
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				m.SchemaVer |= (int64(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		default:
			iNdEx = preIndex
			skippy, err := skipCoprocessor(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if skippy < 0 {
				return ErrInvalidLengthCoprocessor
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
func (m *Response) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowCoprocessor
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
			return fmt.Errorf("proto: Response: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: Response: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field Data", wireType)
			}
			var byteLen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowCoprocessor
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
				return ErrInvalidLengthCoprocessor
			}
			postIndex := iNdEx + byteLen
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			if err := m.Data.Unmarshal(dAtA[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		case 2:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field RegionError", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowCoprocessor
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
				return ErrInvalidLengthCoprocessor
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
		case 3:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field Locked", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowCoprocessor
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
				return ErrInvalidLengthCoprocessor
			}
			postIndex := iNdEx + msglen
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			if m.Locked == nil {
				m.Locked = &kvrpcpb.LockInfo{}
			}
			if err := m.Locked.Unmarshal(dAtA[iNdEx:postIndex]); err != nil {
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
					return ErrIntOverflowCoprocessor
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
				return ErrInvalidLengthCoprocessor
			}
			postIndex := iNdEx + intStringLen
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.OtherError = string(dAtA[iNdEx:postIndex])
			iNdEx = postIndex
		case 5:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field Range", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowCoprocessor
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
				return ErrInvalidLengthCoprocessor
			}
			postIndex := iNdEx + msglen
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			if m.Range == nil {
				m.Range = &KeyRange{}
			}
			if err := m.Range.Unmarshal(dAtA[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		case 6:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field ExecDetails", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowCoprocessor
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
				return ErrInvalidLengthCoprocessor
			}
			postIndex := iNdEx + msglen
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			if m.ExecDetails == nil {
				m.ExecDetails = &kvrpcpb.ExecDetails{}
			}
			if err := m.ExecDetails.Unmarshal(dAtA[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		case 7:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field IsCacheHit", wireType)
			}
			var v int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowCoprocessor
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				v |= (int(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			m.IsCacheHit = bool(v != 0)
		case 8:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field CacheLastVersion", wireType)
			}
			m.CacheLastVersion = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowCoprocessor
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				m.CacheLastVersion |= (uint64(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		default:
			iNdEx = preIndex
			skippy, err := skipCoprocessor(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if skippy < 0 {
				return ErrInvalidLengthCoprocessor
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
func skipCoprocessor(dAtA []byte) (n int, err error) {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return 0, ErrIntOverflowCoprocessor
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
					return 0, ErrIntOverflowCoprocessor
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
					return 0, ErrIntOverflowCoprocessor
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
				return 0, ErrInvalidLengthCoprocessor
			}
			return iNdEx, nil
		case 3:
			for {
				var innerWire uint64
				var start int = iNdEx
				for shift := uint(0); ; shift += 7 {
					if shift >= 64 {
						return 0, ErrIntOverflowCoprocessor
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
				next, err := skipCoprocessor(dAtA[start:])
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
	ErrInvalidLengthCoprocessor = fmt.Errorf("proto: negative length found during unmarshaling")
	ErrIntOverflowCoprocessor   = fmt.Errorf("proto: integer overflow")
)

func init() { proto.RegisterFile("coprocessor.proto", fileDescriptor_coprocessor_f145b8940f787427) }

var fileDescriptor_coprocessor_f145b8940f787427 = []byte{
	// 565 bytes of a gzipped FileDescriptorProto
	0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0xff, 0x74, 0x52, 0x4f, 0x6f, 0xd3, 0x4e,
	0x14, 0xac, 0x93, 0x34, 0x71, 0x9f, 0xd3, 0xfe, 0xd2, 0xfd, 0x15, 0x64, 0x2a, 0x91, 0x46, 0x3d,
	0xa0, 0xf0, 0xcf, 0x11, 0xa9, 0x04, 0x37, 0x0e, 0x2d, 0x95, 0xa8, 0x28, 0x97, 0x05, 0xf5, 0x6a,
	0x6d, 0x36, 0xaf, 0xb6, 0xe5, 0xd4, 0x6b, 0x76, 0xb7, 0x51, 0xfb, 0x39, 0xb8, 0x70, 0xe5, 0xc6,
	0x47, 0xe9, 0x91, 0x23, 0xe2, 0x50, 0xa1, 0xf0, 0x45, 0x90, 0xdf, 0x3a, 0xa1, 0x17, 0x4e, 0x9e,
	0x37, 0x33, 0x79, 0xd9, 0x9d, 0x59, 0xd8, 0x96, 0xaa, 0xd4, 0x4a, 0xa2, 0x31, 0x4a, 0x47, 0xa5,
	0x56, 0x56, 0xb1, 0xe0, 0x0e, 0xb5, 0xbb, 0x89, 0x5a, 0x2b, 0x5d, 0x4e, 0x9c, 0xb6, 0xbb, 0x99,
	0xcf, 0x75, 0x29, 0x57, 0xe3, 0x4e, 0xa2, 0x12, 0x45, 0x70, 0x54, 0xa1, 0x9a, 0xfd, 0x4f, 0x5f,
	0x1a, 0x4b, 0xd0, 0x11, 0xfb, 0x63, 0xf0, 0xdf, 0xe1, 0x35, 0x17, 0x45, 0x82, 0x6c, 0x07, 0xd6,
	0x8d, 0x15, 0xda, 0x86, 0xde, 0xc0, 0x1b, 0x76, 0xb9, 0x1b, 0x58, 0x0f, 0x9a, 0x58, 0x4c, 0xc3,
	0x06, 0x71, 0x15, 0xdc, 0xff, 0xda, 0x80, 0x0e, 0xc7, 0x4f, 0x97, 0x68, 0x2c, 0x7b, 0x02, 0x1d,
	0xa9, 0x0a, 0x8b, 0x57, 0xee, 0x57, 0xc1, 0xb8, 0x17, 0x2d, 0xcf, 0x71, 0xe4, 0x78, 0xbe, 0x34,
	0xb0, 0x2d, 0x68, 0xd8, 0x92, 0x16, 0x35, 0x79, 0xc3, 0x96, 0x8c, 0x41, 0x6b, 0x2a, 0xac, 0x08,
	0x9b, 0xb4, 0x9a, 0x30, 0x7b, 0x0e, 0x6d, 0x5d, 0x1d, 0xc6, 0x84, 0xad, 0x41, 0x73, 0x18, 0x8c,
	0xef, 0x45, 0x77, 0x53, 0x58, 0x1e, 0x95, 0xd7, 0x26, 0x36, 0x84, 0x5e, 0x66, 0x62, 0x29, 0x64,
	0x8a, 0x31, 0x16, 0x62, 0x32, 0xc3, 0x69, 0xb8, 0x3e, 0xf0, 0x86, 0x3e, 0xdf, 0xca, 0xcc, 0x51,
	0x45, 0x1f, 0x3b, 0x96, 0x1d, 0xc0, 0x7d, 0x67, 0xcb, 0xce, 0xe3, 0x0b, 0x61, 0x65, 0x1a, 0xcf,
	0x51, 0x9b, 0x4c, 0x15, 0x61, 0x7b, 0xe0, 0x0d, 0x5b, 0xfc, 0x7f, 0x52, 0x4f, 0xce, 0xdf, 0x57,
	0xda, 0x99, 0x93, 0xd8, 0x03, 0xf0, 0x29, 0x84, 0xd8, 0x9a, 0xb0, 0x43, 0xb6, 0x0e, 0xcd, 0x1f,
	0x0d, 0x7b, 0x08, 0x60, 0x64, 0x8a, 0x17, 0xa2, 0xda, 0x13, 0xfa, 0x74, 0xa9, 0x0d, 0xc7, 0x9c,
	0xa1, 0xde, 0xff, 0xdc, 0x04, 0x9f, 0xa3, 0x29, 0x55, 0x61, 0x90, 0xf1, 0xfa, 0xa2, 0x94, 0xeb,
	0xe1, 0xeb, 0x9b, 0xdb, 0xbd, 0xb5, 0x9f, 0xb7, 0x7b, 0x2f, 0x93, 0xcc, 0xa6, 0x97, 0x93, 0x48,
	0xaa, 0x8b, 0x51, 0x99, 0x15, 0x89, 0x14, 0xe5, 0x28, 0x9f, 0xbb, 0xca, 0xca, 0x3c, 0x19, 0x99,
	0x54, 0x68, 0x9c, 0x4e, 0xae, 0x2d, 0x9a, 0xe8, 0x03, 0xe1, 0xc3, 0x0a, 0xd7, 0x41, 0xbd, 0x80,
	0xae, 0xc6, 0x24, 0x53, 0x45, 0x4c, 0xcf, 0x80, 0x62, 0x0d, 0xc6, 0x5b, 0xd1, 0xf2, 0x51, 0x1c,
	0x57, 0x5f, 0x1e, 0x38, 0x0f, 0x0d, 0xec, 0x31, 0xb4, 0x67, 0x4a, 0xe6, 0x38, 0xa5, 0xc4, 0x83,
	0xf1, 0xf6, 0xaa, 0xaa, 0x53, 0x25, 0xf3, 0x93, 0xe2, 0x5c, 0xf1, 0xda, 0xc0, 0xf6, 0x20, 0x50,
	0x36, 0x45, 0x5d, 0x2f, 0x6f, 0x0d, 0xbc, 0xe1, 0x06, 0x07, 0xa2, 0xdc, 0xae, 0xa7, 0xb0, 0x4e,
	0x15, 0x50, 0xda, 0xff, 0xac, 0xc9, 0x79, 0xd8, 0x2b, 0xe8, 0xe2, 0x15, 0xca, 0x78, 0x8a, 0x56,
	0x64, 0x33, 0x43, 0x89, 0x07, 0xe3, 0x9d, 0xd5, 0xdf, 0x1f, 0x5f, 0xa1, 0x7c, 0xe3, 0x34, 0x1e,
	0xe0, 0xdf, 0x81, 0x0d, 0xa0, 0xbb, 0xaa, 0x37, 0xcd, 0x2c, 0x75, 0xe0, 0x73, 0xa8, 0xab, 0x7d,
	0x9b, 0x59, 0xf6, 0x0c, 0x98, 0x93, 0x67, 0xc2, 0xd8, 0x55, 0xa5, 0x3e, 0x75, 0xd5, 0x23, 0xe5,
	0x54, 0x18, 0x5b, 0xf7, 0x79, 0xf8, 0xe8, 0xc7, 0x37, 0xdf, 0xbb, 0x59, 0xf4, 0xbd, 0xef, 0x8b,
	0xbe, 0xf7, 0x6b, 0xd1, 0xf7, 0xbe, 0xfc, 0xee, 0xaf, 0x41, 0x4f, 0xe9, 0x24, 0xb2, 0x59, 0x3e,
	0x8f, 0xea, 0xf8, 0x27, 0x6d, 0xfa, 0x1c, 0xfc, 0x09, 0x00, 0x00, 0xff, 0xff, 0x04, 0x8c, 0xaf,
	0xb6, 0x83, 0x03, 0x00, 0x00,
}
