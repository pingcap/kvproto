// Code generated by protoc-gen-go.
// source: kvrpcpb.proto
// DO NOT EDIT!

/*
Package kvrpcpb is a generated protocol buffer package.

It is generated from these files:
	kvrpcpb.proto

It has these top-level messages:
	LockInfo
	KeyError
	Context
	CmdGetRequest
	CmdGetResponse
	CmdScanRequest
	KvPair
	CmdScanResponse
	Mutation
	CmdPrewriteRequest
	CmdPrewriteResponse
	CmdCommitRequest
	CmdCommitResponse
	CmdCleanupRequest
	CmdCleanupResponse
	CmdRollbackThenGetRequest
	CmdRollbackThenGetResponse
	CmdCommitThenGetRequest
	CmdCommitThenGetResponse
	CmdBatchGetRequest
	CmdBatchGetResponse
	Request
	Response
*/
package kvrpcpb

import proto "github.com/golang/protobuf/proto"
import fmt "fmt"
import math "math"
import metapb "github.com/pingcap/kvproto/pkg/metapb"
import errorpb "github.com/pingcap/kvproto/pkg/errorpb"

// Reference imports to suppress errors if they are not otherwise used.
var _ = proto.Marshal
var _ = fmt.Errorf
var _ = math.Inf

// This is a compile-time assertion to ensure that this generated file
// is compatible with the proto package it is being compiled against.
const _ = proto.ProtoPackageIsVersion1

type MessageType int32

const (
	MessageType_CmdGet      MessageType = 1
	MessageType_CmdScan     MessageType = 2
	MessageType_CmdPrewrite MessageType = 3
	MessageType_CmdCommit   MessageType = 4
	MessageType_CmdCleanup  MessageType = 5
	// Below types both use for Get failed. If Get failed, it may be locked.
	// So it tries to clean primary lock(CmdCleanup), and then server will return
	// either committed or rolled back. Finally, client will commit/rollback
	// primary lock and then Get again.
	MessageType_CmdRollbackThenGet MessageType = 6
	MessageType_CmdCommitThenGet   MessageType = 7
	MessageType_CmdBatchGet        MessageType = 8
)

var MessageType_name = map[int32]string{
	1: "CmdGet",
	2: "CmdScan",
	3: "CmdPrewrite",
	4: "CmdCommit",
	5: "CmdCleanup",
	6: "CmdRollbackThenGet",
	7: "CmdCommitThenGet",
	8: "CmdBatchGet",
}
var MessageType_value = map[string]int32{
	"CmdGet":             1,
	"CmdScan":            2,
	"CmdPrewrite":        3,
	"CmdCommit":          4,
	"CmdCleanup":         5,
	"CmdRollbackThenGet": 6,
	"CmdCommitThenGet":   7,
	"CmdBatchGet":        8,
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

type Op int32

const (
	Op_Put  Op = 1
	Op_Del  Op = 2
	Op_Lock Op = 3
)

var Op_name = map[int32]string{
	1: "Put",
	2: "Del",
	3: "Lock",
}
var Op_value = map[string]int32{
	"Put":  1,
	"Del":  2,
	"Lock": 3,
}

func (x Op) Enum() *Op {
	p := new(Op)
	*p = x
	return p
}
func (x Op) String() string {
	return proto.EnumName(Op_name, int32(x))
}
func (x *Op) UnmarshalJSON(data []byte) error {
	value, err := proto.UnmarshalJSONEnum(Op_value, data, "Op")
	if err != nil {
		return err
	}
	*x = Op(value)
	return nil
}
func (Op) EnumDescriptor() ([]byte, []int) { return fileDescriptor0, []int{1} }

type LockInfo struct {
	PrimaryLock      []byte  `protobuf:"bytes,1,opt,name=primary_lock,json=primaryLock" json:"primary_lock,omitempty"`
	LockVersion      *uint64 `protobuf:"varint,2,opt,name=lock_version,json=lockVersion" json:"lock_version,omitempty"`
	Key              []byte  `protobuf:"bytes,3,opt,name=key" json:"key,omitempty"`
	XXX_unrecognized []byte  `json:"-"`
}

func (m *LockInfo) Reset()                    { *m = LockInfo{} }
func (m *LockInfo) String() string            { return proto.CompactTextString(m) }
func (*LockInfo) ProtoMessage()               {}
func (*LockInfo) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{0} }

func (m *LockInfo) GetPrimaryLock() []byte {
	if m != nil {
		return m.PrimaryLock
	}
	return nil
}

func (m *LockInfo) GetLockVersion() uint64 {
	if m != nil && m.LockVersion != nil {
		return *m.LockVersion
	}
	return 0
}

func (m *LockInfo) GetKey() []byte {
	if m != nil {
		return m.Key
	}
	return nil
}

type KeyError struct {
	Locked           *LockInfo `protobuf:"bytes,1,opt,name=locked" json:"locked,omitempty"`
	Retryable        *string   `protobuf:"bytes,2,opt,name=retryable" json:"retryable,omitempty"`
	Abort            *string   `protobuf:"bytes,3,opt,name=abort" json:"abort,omitempty"`
	XXX_unrecognized []byte    `json:"-"`
}

func (m *KeyError) Reset()                    { *m = KeyError{} }
func (m *KeyError) String() string            { return proto.CompactTextString(m) }
func (*KeyError) ProtoMessage()               {}
func (*KeyError) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{1} }

func (m *KeyError) GetLocked() *LockInfo {
	if m != nil {
		return m.Locked
	}
	return nil
}

func (m *KeyError) GetRetryable() string {
	if m != nil && m.Retryable != nil {
		return *m.Retryable
	}
	return ""
}

func (m *KeyError) GetAbort() string {
	if m != nil && m.Abort != nil {
		return *m.Abort
	}
	return ""
}

type Context struct {
	RegionId         *uint64             `protobuf:"varint,1,opt,name=region_id,json=regionId" json:"region_id,omitempty"`
	RegionEpoch      *metapb.RegionEpoch `protobuf:"bytes,2,opt,name=region_epoch,json=regionEpoch" json:"region_epoch,omitempty"`
	XXX_unrecognized []byte              `json:"-"`
}

func (m *Context) Reset()                    { *m = Context{} }
func (m *Context) String() string            { return proto.CompactTextString(m) }
func (*Context) ProtoMessage()               {}
func (*Context) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{2} }

func (m *Context) GetRegionId() uint64 {
	if m != nil && m.RegionId != nil {
		return *m.RegionId
	}
	return 0
}

func (m *Context) GetRegionEpoch() *metapb.RegionEpoch {
	if m != nil {
		return m.RegionEpoch
	}
	return nil
}

type CmdGetRequest struct {
	Key              []byte  `protobuf:"bytes,1,opt,name=key" json:"key,omitempty"`
	Version          *uint64 `protobuf:"varint,2,opt,name=version" json:"version,omitempty"`
	XXX_unrecognized []byte  `json:"-"`
}

func (m *CmdGetRequest) Reset()                    { *m = CmdGetRequest{} }
func (m *CmdGetRequest) String() string            { return proto.CompactTextString(m) }
func (*CmdGetRequest) ProtoMessage()               {}
func (*CmdGetRequest) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{3} }

func (m *CmdGetRequest) GetKey() []byte {
	if m != nil {
		return m.Key
	}
	return nil
}

func (m *CmdGetRequest) GetVersion() uint64 {
	if m != nil && m.Version != nil {
		return *m.Version
	}
	return 0
}

type CmdGetResponse struct {
	Error            *KeyError `protobuf:"bytes,1,opt,name=error" json:"error,omitempty"`
	Value            []byte    `protobuf:"bytes,2,opt,name=value" json:"value,omitempty"`
	XXX_unrecognized []byte    `json:"-"`
}

func (m *CmdGetResponse) Reset()                    { *m = CmdGetResponse{} }
func (m *CmdGetResponse) String() string            { return proto.CompactTextString(m) }
func (*CmdGetResponse) ProtoMessage()               {}
func (*CmdGetResponse) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{4} }

func (m *CmdGetResponse) GetError() *KeyError {
	if m != nil {
		return m.Error
	}
	return nil
}

func (m *CmdGetResponse) GetValue() []byte {
	if m != nil {
		return m.Value
	}
	return nil
}

type CmdScanRequest struct {
	StartKey         []byte  `protobuf:"bytes,1,opt,name=start_key,json=startKey" json:"start_key,omitempty"`
	Limit            *uint32 `protobuf:"varint,2,opt,name=limit" json:"limit,omitempty"`
	Version          *uint64 `protobuf:"varint,3,opt,name=version" json:"version,omitempty"`
	XXX_unrecognized []byte  `json:"-"`
}

func (m *CmdScanRequest) Reset()                    { *m = CmdScanRequest{} }
func (m *CmdScanRequest) String() string            { return proto.CompactTextString(m) }
func (*CmdScanRequest) ProtoMessage()               {}
func (*CmdScanRequest) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{5} }

func (m *CmdScanRequest) GetStartKey() []byte {
	if m != nil {
		return m.StartKey
	}
	return nil
}

func (m *CmdScanRequest) GetLimit() uint32 {
	if m != nil && m.Limit != nil {
		return *m.Limit
	}
	return 0
}

func (m *CmdScanRequest) GetVersion() uint64 {
	if m != nil && m.Version != nil {
		return *m.Version
	}
	return 0
}

type KvPair struct {
	Error            *KeyError `protobuf:"bytes,1,opt,name=error" json:"error,omitempty"`
	Key              []byte    `protobuf:"bytes,2,opt,name=key" json:"key,omitempty"`
	Value            []byte    `protobuf:"bytes,3,opt,name=value" json:"value,omitempty"`
	XXX_unrecognized []byte    `json:"-"`
}

func (m *KvPair) Reset()                    { *m = KvPair{} }
func (m *KvPair) String() string            { return proto.CompactTextString(m) }
func (*KvPair) ProtoMessage()               {}
func (*KvPair) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{6} }

func (m *KvPair) GetError() *KeyError {
	if m != nil {
		return m.Error
	}
	return nil
}

func (m *KvPair) GetKey() []byte {
	if m != nil {
		return m.Key
	}
	return nil
}

func (m *KvPair) GetValue() []byte {
	if m != nil {
		return m.Value
	}
	return nil
}

type CmdScanResponse struct {
	Pairs            []*KvPair `protobuf:"bytes,1,rep,name=pairs" json:"pairs,omitempty"`
	XXX_unrecognized []byte    `json:"-"`
}

func (m *CmdScanResponse) Reset()                    { *m = CmdScanResponse{} }
func (m *CmdScanResponse) String() string            { return proto.CompactTextString(m) }
func (*CmdScanResponse) ProtoMessage()               {}
func (*CmdScanResponse) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{7} }

func (m *CmdScanResponse) GetPairs() []*KvPair {
	if m != nil {
		return m.Pairs
	}
	return nil
}

type Mutation struct {
	Op               *Op    `protobuf:"varint,1,opt,name=op,enum=kvrpcpb.Op" json:"op,omitempty"`
	Key              []byte `protobuf:"bytes,2,opt,name=key" json:"key,omitempty"`
	Value            []byte `protobuf:"bytes,3,opt,name=value" json:"value,omitempty"`
	XXX_unrecognized []byte `json:"-"`
}

func (m *Mutation) Reset()                    { *m = Mutation{} }
func (m *Mutation) String() string            { return proto.CompactTextString(m) }
func (*Mutation) ProtoMessage()               {}
func (*Mutation) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{8} }

func (m *Mutation) GetOp() Op {
	if m != nil && m.Op != nil {
		return *m.Op
	}
	return Op_Put
}

func (m *Mutation) GetKey() []byte {
	if m != nil {
		return m.Key
	}
	return nil
}

func (m *Mutation) GetValue() []byte {
	if m != nil {
		return m.Value
	}
	return nil
}

type CmdPrewriteRequest struct {
	Mutations []*Mutation `protobuf:"bytes,1,rep,name=mutations" json:"mutations,omitempty"`
	// primary_lock_key
	PrimaryLock      []byte  `protobuf:"bytes,2,opt,name=primary_lock,json=primaryLock" json:"primary_lock,omitempty"`
	StartVersion     *uint64 `protobuf:"varint,3,opt,name=start_version,json=startVersion" json:"start_version,omitempty"`
	XXX_unrecognized []byte  `json:"-"`
}

func (m *CmdPrewriteRequest) Reset()                    { *m = CmdPrewriteRequest{} }
func (m *CmdPrewriteRequest) String() string            { return proto.CompactTextString(m) }
func (*CmdPrewriteRequest) ProtoMessage()               {}
func (*CmdPrewriteRequest) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{9} }

func (m *CmdPrewriteRequest) GetMutations() []*Mutation {
	if m != nil {
		return m.Mutations
	}
	return nil
}

func (m *CmdPrewriteRequest) GetPrimaryLock() []byte {
	if m != nil {
		return m.PrimaryLock
	}
	return nil
}

func (m *CmdPrewriteRequest) GetStartVersion() uint64 {
	if m != nil && m.StartVersion != nil {
		return *m.StartVersion
	}
	return 0
}

type CmdPrewriteResponse struct {
	Errors           []*KeyError `protobuf:"bytes,1,rep,name=errors" json:"errors,omitempty"`
	XXX_unrecognized []byte      `json:"-"`
}

func (m *CmdPrewriteResponse) Reset()                    { *m = CmdPrewriteResponse{} }
func (m *CmdPrewriteResponse) String() string            { return proto.CompactTextString(m) }
func (*CmdPrewriteResponse) ProtoMessage()               {}
func (*CmdPrewriteResponse) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{10} }

func (m *CmdPrewriteResponse) GetErrors() []*KeyError {
	if m != nil {
		return m.Errors
	}
	return nil
}

type CmdCommitRequest struct {
	StartVersion     *uint64  `protobuf:"varint,1,opt,name=start_version,json=startVersion" json:"start_version,omitempty"`
	Keys             [][]byte `protobuf:"bytes,2,rep,name=keys" json:"keys,omitempty"`
	CommitVersion    *uint64  `protobuf:"varint,3,opt,name=commit_version,json=commitVersion" json:"commit_version,omitempty"`
	XXX_unrecognized []byte   `json:"-"`
}

func (m *CmdCommitRequest) Reset()                    { *m = CmdCommitRequest{} }
func (m *CmdCommitRequest) String() string            { return proto.CompactTextString(m) }
func (*CmdCommitRequest) ProtoMessage()               {}
func (*CmdCommitRequest) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{11} }

func (m *CmdCommitRequest) GetStartVersion() uint64 {
	if m != nil && m.StartVersion != nil {
		return *m.StartVersion
	}
	return 0
}

func (m *CmdCommitRequest) GetKeys() [][]byte {
	if m != nil {
		return m.Keys
	}
	return nil
}

func (m *CmdCommitRequest) GetCommitVersion() uint64 {
	if m != nil && m.CommitVersion != nil {
		return *m.CommitVersion
	}
	return 0
}

type CmdCommitResponse struct {
	Errors           []*KeyError `protobuf:"bytes,1,rep,name=errors" json:"errors,omitempty"`
	XXX_unrecognized []byte      `json:"-"`
}

func (m *CmdCommitResponse) Reset()                    { *m = CmdCommitResponse{} }
func (m *CmdCommitResponse) String() string            { return proto.CompactTextString(m) }
func (*CmdCommitResponse) ProtoMessage()               {}
func (*CmdCommitResponse) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{12} }

func (m *CmdCommitResponse) GetErrors() []*KeyError {
	if m != nil {
		return m.Errors
	}
	return nil
}

type CmdCleanupRequest struct {
	Key              []byte  `protobuf:"bytes,1,opt,name=key" json:"key,omitempty"`
	StartVersion     *uint64 `protobuf:"varint,2,opt,name=start_version,json=startVersion" json:"start_version,omitempty"`
	XXX_unrecognized []byte  `json:"-"`
}

func (m *CmdCleanupRequest) Reset()                    { *m = CmdCleanupRequest{} }
func (m *CmdCleanupRequest) String() string            { return proto.CompactTextString(m) }
func (*CmdCleanupRequest) ProtoMessage()               {}
func (*CmdCleanupRequest) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{13} }

func (m *CmdCleanupRequest) GetKey() []byte {
	if m != nil {
		return m.Key
	}
	return nil
}

func (m *CmdCleanupRequest) GetStartVersion() uint64 {
	if m != nil && m.StartVersion != nil {
		return *m.StartVersion
	}
	return 0
}

type CmdCleanupResponse struct {
	Error            *KeyError `protobuf:"bytes,1,opt,name=error" json:"error,omitempty"`
	CommitVersion    *uint64   `protobuf:"varint,2,opt,name=commit_version,json=commitVersion" json:"commit_version,omitempty"`
	XXX_unrecognized []byte    `json:"-"`
}

func (m *CmdCleanupResponse) Reset()                    { *m = CmdCleanupResponse{} }
func (m *CmdCleanupResponse) String() string            { return proto.CompactTextString(m) }
func (*CmdCleanupResponse) ProtoMessage()               {}
func (*CmdCleanupResponse) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{14} }

func (m *CmdCleanupResponse) GetError() *KeyError {
	if m != nil {
		return m.Error
	}
	return nil
}

func (m *CmdCleanupResponse) GetCommitVersion() uint64 {
	if m != nil && m.CommitVersion != nil {
		return *m.CommitVersion
	}
	return 0
}

type CmdRollbackThenGetRequest struct {
	Key              []byte  `protobuf:"bytes,1,opt,name=key" json:"key,omitempty"`
	LockVersion      *uint64 `protobuf:"varint,2,opt,name=lock_version,json=lockVersion" json:"lock_version,omitempty"`
	XXX_unrecognized []byte  `json:"-"`
}

func (m *CmdRollbackThenGetRequest) Reset()                    { *m = CmdRollbackThenGetRequest{} }
func (m *CmdRollbackThenGetRequest) String() string            { return proto.CompactTextString(m) }
func (*CmdRollbackThenGetRequest) ProtoMessage()               {}
func (*CmdRollbackThenGetRequest) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{15} }

func (m *CmdRollbackThenGetRequest) GetKey() []byte {
	if m != nil {
		return m.Key
	}
	return nil
}

func (m *CmdRollbackThenGetRequest) GetLockVersion() uint64 {
	if m != nil && m.LockVersion != nil {
		return *m.LockVersion
	}
	return 0
}

type CmdRollbackThenGetResponse struct {
	Error            *KeyError `protobuf:"bytes,1,opt,name=error" json:"error,omitempty"`
	Value            []byte    `protobuf:"bytes,2,opt,name=value" json:"value,omitempty"`
	XXX_unrecognized []byte    `json:"-"`
}

func (m *CmdRollbackThenGetResponse) Reset()                    { *m = CmdRollbackThenGetResponse{} }
func (m *CmdRollbackThenGetResponse) String() string            { return proto.CompactTextString(m) }
func (*CmdRollbackThenGetResponse) ProtoMessage()               {}
func (*CmdRollbackThenGetResponse) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{16} }

func (m *CmdRollbackThenGetResponse) GetError() *KeyError {
	if m != nil {
		return m.Error
	}
	return nil
}

func (m *CmdRollbackThenGetResponse) GetValue() []byte {
	if m != nil {
		return m.Value
	}
	return nil
}

type CmdCommitThenGetRequest struct {
	Key              []byte  `protobuf:"bytes,1,opt,name=key" json:"key,omitempty"`
	LockVersion      *uint64 `protobuf:"varint,2,opt,name=lock_version,json=lockVersion" json:"lock_version,omitempty"`
	CommitVersion    *uint64 `protobuf:"varint,3,opt,name=commit_version,json=commitVersion" json:"commit_version,omitempty"`
	GetVersion       *uint64 `protobuf:"varint,4,opt,name=get_version,json=getVersion" json:"get_version,omitempty"`
	XXX_unrecognized []byte  `json:"-"`
}

func (m *CmdCommitThenGetRequest) Reset()                    { *m = CmdCommitThenGetRequest{} }
func (m *CmdCommitThenGetRequest) String() string            { return proto.CompactTextString(m) }
func (*CmdCommitThenGetRequest) ProtoMessage()               {}
func (*CmdCommitThenGetRequest) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{17} }

func (m *CmdCommitThenGetRequest) GetKey() []byte {
	if m != nil {
		return m.Key
	}
	return nil
}

func (m *CmdCommitThenGetRequest) GetLockVersion() uint64 {
	if m != nil && m.LockVersion != nil {
		return *m.LockVersion
	}
	return 0
}

func (m *CmdCommitThenGetRequest) GetCommitVersion() uint64 {
	if m != nil && m.CommitVersion != nil {
		return *m.CommitVersion
	}
	return 0
}

func (m *CmdCommitThenGetRequest) GetGetVersion() uint64 {
	if m != nil && m.GetVersion != nil {
		return *m.GetVersion
	}
	return 0
}

type CmdCommitThenGetResponse struct {
	Error            *KeyError `protobuf:"bytes,1,opt,name=error" json:"error,omitempty"`
	Value            []byte    `protobuf:"bytes,2,opt,name=value" json:"value,omitempty"`
	XXX_unrecognized []byte    `json:"-"`
}

func (m *CmdCommitThenGetResponse) Reset()                    { *m = CmdCommitThenGetResponse{} }
func (m *CmdCommitThenGetResponse) String() string            { return proto.CompactTextString(m) }
func (*CmdCommitThenGetResponse) ProtoMessage()               {}
func (*CmdCommitThenGetResponse) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{18} }

func (m *CmdCommitThenGetResponse) GetError() *KeyError {
	if m != nil {
		return m.Error
	}
	return nil
}

func (m *CmdCommitThenGetResponse) GetValue() []byte {
	if m != nil {
		return m.Value
	}
	return nil
}

type CmdBatchGetRequest struct {
	Keys             [][]byte `protobuf:"bytes,1,rep,name=keys" json:"keys,omitempty"`
	Version          *uint64  `protobuf:"varint,2,opt,name=version" json:"version,omitempty"`
	XXX_unrecognized []byte   `json:"-"`
}

func (m *CmdBatchGetRequest) Reset()                    { *m = CmdBatchGetRequest{} }
func (m *CmdBatchGetRequest) String() string            { return proto.CompactTextString(m) }
func (*CmdBatchGetRequest) ProtoMessage()               {}
func (*CmdBatchGetRequest) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{19} }

func (m *CmdBatchGetRequest) GetKeys() [][]byte {
	if m != nil {
		return m.Keys
	}
	return nil
}

func (m *CmdBatchGetRequest) GetVersion() uint64 {
	if m != nil && m.Version != nil {
		return *m.Version
	}
	return 0
}

type CmdBatchGetResponse struct {
	Pairs            []*KvPair `protobuf:"bytes,1,rep,name=pairs" json:"pairs,omitempty"`
	XXX_unrecognized []byte    `json:"-"`
}

func (m *CmdBatchGetResponse) Reset()                    { *m = CmdBatchGetResponse{} }
func (m *CmdBatchGetResponse) String() string            { return proto.CompactTextString(m) }
func (*CmdBatchGetResponse) ProtoMessage()               {}
func (*CmdBatchGetResponse) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{20} }

func (m *CmdBatchGetResponse) GetPairs() []*KvPair {
	if m != nil {
		return m.Pairs
	}
	return nil
}

type Request struct {
	Type             *MessageType               `protobuf:"varint,1,opt,name=type,enum=kvrpcpb.MessageType" json:"type,omitempty"`
	Context          *Context                   `protobuf:"bytes,2,opt,name=context" json:"context,omitempty"`
	CmdGetReq        *CmdGetRequest             `protobuf:"bytes,3,opt,name=cmd_get_req,json=cmdGetReq" json:"cmd_get_req,omitempty"`
	CmdScanReq       *CmdScanRequest            `protobuf:"bytes,4,opt,name=cmd_scan_req,json=cmdScanReq" json:"cmd_scan_req,omitempty"`
	CmdPrewriteReq   *CmdPrewriteRequest        `protobuf:"bytes,5,opt,name=cmd_prewrite_req,json=cmdPrewriteReq" json:"cmd_prewrite_req,omitempty"`
	CmdCommitReq     *CmdCommitRequest          `protobuf:"bytes,6,opt,name=cmd_commit_req,json=cmdCommitReq" json:"cmd_commit_req,omitempty"`
	CmdCleanupReq    *CmdCleanupRequest         `protobuf:"bytes,7,opt,name=cmd_cleanup_req,json=cmdCleanupReq" json:"cmd_cleanup_req,omitempty"`
	CmdRbGetReq      *CmdRollbackThenGetRequest `protobuf:"bytes,8,opt,name=cmd_rb_get_req,json=cmdRbGetReq" json:"cmd_rb_get_req,omitempty"`
	CmdCommitGetReq  *CmdCommitThenGetRequest   `protobuf:"bytes,9,opt,name=cmd_commit_get_req,json=cmdCommitGetReq" json:"cmd_commit_get_req,omitempty"`
	CmdBatchGetReq   *CmdBatchGetRequest        `protobuf:"bytes,10,opt,name=cmd_batch_get_req,json=cmdBatchGetReq" json:"cmd_batch_get_req,omitempty"`
	XXX_unrecognized []byte                     `json:"-"`
}

func (m *Request) Reset()                    { *m = Request{} }
func (m *Request) String() string            { return proto.CompactTextString(m) }
func (*Request) ProtoMessage()               {}
func (*Request) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{21} }

func (m *Request) GetType() MessageType {
	if m != nil && m.Type != nil {
		return *m.Type
	}
	return MessageType_CmdGet
}

func (m *Request) GetContext() *Context {
	if m != nil {
		return m.Context
	}
	return nil
}

func (m *Request) GetCmdGetReq() *CmdGetRequest {
	if m != nil {
		return m.CmdGetReq
	}
	return nil
}

func (m *Request) GetCmdScanReq() *CmdScanRequest {
	if m != nil {
		return m.CmdScanReq
	}
	return nil
}

func (m *Request) GetCmdPrewriteReq() *CmdPrewriteRequest {
	if m != nil {
		return m.CmdPrewriteReq
	}
	return nil
}

func (m *Request) GetCmdCommitReq() *CmdCommitRequest {
	if m != nil {
		return m.CmdCommitReq
	}
	return nil
}

func (m *Request) GetCmdCleanupReq() *CmdCleanupRequest {
	if m != nil {
		return m.CmdCleanupReq
	}
	return nil
}

func (m *Request) GetCmdRbGetReq() *CmdRollbackThenGetRequest {
	if m != nil {
		return m.CmdRbGetReq
	}
	return nil
}

func (m *Request) GetCmdCommitGetReq() *CmdCommitThenGetRequest {
	if m != nil {
		return m.CmdCommitGetReq
	}
	return nil
}

func (m *Request) GetCmdBatchGetReq() *CmdBatchGetRequest {
	if m != nil {
		return m.CmdBatchGetReq
	}
	return nil
}

type Response struct {
	Type             *MessageType                `protobuf:"varint,1,opt,name=type,enum=kvrpcpb.MessageType" json:"type,omitempty"`
	RegionError      *errorpb.Error              `protobuf:"bytes,2,opt,name=region_error,json=regionError" json:"region_error,omitempty"`
	CmdGetResp       *CmdGetResponse             `protobuf:"bytes,3,opt,name=cmd_get_resp,json=cmdGetResp" json:"cmd_get_resp,omitempty"`
	CmdScanResp      *CmdScanResponse            `protobuf:"bytes,4,opt,name=cmd_scan_resp,json=cmdScanResp" json:"cmd_scan_resp,omitempty"`
	CmdPrewriteResp  *CmdPrewriteResponse        `protobuf:"bytes,5,opt,name=cmd_prewrite_resp,json=cmdPrewriteResp" json:"cmd_prewrite_resp,omitempty"`
	CmdCommitResp    *CmdCommitResponse          `protobuf:"bytes,6,opt,name=cmd_commit_resp,json=cmdCommitResp" json:"cmd_commit_resp,omitempty"`
	CmdCleanupResp   *CmdCleanupResponse         `protobuf:"bytes,7,opt,name=cmd_cleanup_resp,json=cmdCleanupResp" json:"cmd_cleanup_resp,omitempty"`
	CmdRbGetResp     *CmdRollbackThenGetResponse `protobuf:"bytes,8,opt,name=cmd_rb_get_resp,json=cmdRbGetResp" json:"cmd_rb_get_resp,omitempty"`
	CmdCommitGetResp *CmdCommitThenGetResponse   `protobuf:"bytes,9,opt,name=cmd_commit_get_resp,json=cmdCommitGetResp" json:"cmd_commit_get_resp,omitempty"`
	CmdBatchGetResp  *CmdBatchGetResponse        `protobuf:"bytes,10,opt,name=cmd_batch_get_resp,json=cmdBatchGetResp" json:"cmd_batch_get_resp,omitempty"`
	XXX_unrecognized []byte                      `json:"-"`
}

func (m *Response) Reset()                    { *m = Response{} }
func (m *Response) String() string            { return proto.CompactTextString(m) }
func (*Response) ProtoMessage()               {}
func (*Response) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{22} }

func (m *Response) GetType() MessageType {
	if m != nil && m.Type != nil {
		return *m.Type
	}
	return MessageType_CmdGet
}

func (m *Response) GetRegionError() *errorpb.Error {
	if m != nil {
		return m.RegionError
	}
	return nil
}

func (m *Response) GetCmdGetResp() *CmdGetResponse {
	if m != nil {
		return m.CmdGetResp
	}
	return nil
}

func (m *Response) GetCmdScanResp() *CmdScanResponse {
	if m != nil {
		return m.CmdScanResp
	}
	return nil
}

func (m *Response) GetCmdPrewriteResp() *CmdPrewriteResponse {
	if m != nil {
		return m.CmdPrewriteResp
	}
	return nil
}

func (m *Response) GetCmdCommitResp() *CmdCommitResponse {
	if m != nil {
		return m.CmdCommitResp
	}
	return nil
}

func (m *Response) GetCmdCleanupResp() *CmdCleanupResponse {
	if m != nil {
		return m.CmdCleanupResp
	}
	return nil
}

func (m *Response) GetCmdRbGetResp() *CmdRollbackThenGetResponse {
	if m != nil {
		return m.CmdRbGetResp
	}
	return nil
}

func (m *Response) GetCmdCommitGetResp() *CmdCommitThenGetResponse {
	if m != nil {
		return m.CmdCommitGetResp
	}
	return nil
}

func (m *Response) GetCmdBatchGetResp() *CmdBatchGetResponse {
	if m != nil {
		return m.CmdBatchGetResp
	}
	return nil
}

func init() {
	proto.RegisterType((*LockInfo)(nil), "kvrpcpb.LockInfo")
	proto.RegisterType((*KeyError)(nil), "kvrpcpb.KeyError")
	proto.RegisterType((*Context)(nil), "kvrpcpb.Context")
	proto.RegisterType((*CmdGetRequest)(nil), "kvrpcpb.CmdGetRequest")
	proto.RegisterType((*CmdGetResponse)(nil), "kvrpcpb.CmdGetResponse")
	proto.RegisterType((*CmdScanRequest)(nil), "kvrpcpb.CmdScanRequest")
	proto.RegisterType((*KvPair)(nil), "kvrpcpb.KvPair")
	proto.RegisterType((*CmdScanResponse)(nil), "kvrpcpb.CmdScanResponse")
	proto.RegisterType((*Mutation)(nil), "kvrpcpb.Mutation")
	proto.RegisterType((*CmdPrewriteRequest)(nil), "kvrpcpb.CmdPrewriteRequest")
	proto.RegisterType((*CmdPrewriteResponse)(nil), "kvrpcpb.CmdPrewriteResponse")
	proto.RegisterType((*CmdCommitRequest)(nil), "kvrpcpb.CmdCommitRequest")
	proto.RegisterType((*CmdCommitResponse)(nil), "kvrpcpb.CmdCommitResponse")
	proto.RegisterType((*CmdCleanupRequest)(nil), "kvrpcpb.CmdCleanupRequest")
	proto.RegisterType((*CmdCleanupResponse)(nil), "kvrpcpb.CmdCleanupResponse")
	proto.RegisterType((*CmdRollbackThenGetRequest)(nil), "kvrpcpb.CmdRollbackThenGetRequest")
	proto.RegisterType((*CmdRollbackThenGetResponse)(nil), "kvrpcpb.CmdRollbackThenGetResponse")
	proto.RegisterType((*CmdCommitThenGetRequest)(nil), "kvrpcpb.CmdCommitThenGetRequest")
	proto.RegisterType((*CmdCommitThenGetResponse)(nil), "kvrpcpb.CmdCommitThenGetResponse")
	proto.RegisterType((*CmdBatchGetRequest)(nil), "kvrpcpb.CmdBatchGetRequest")
	proto.RegisterType((*CmdBatchGetResponse)(nil), "kvrpcpb.CmdBatchGetResponse")
	proto.RegisterType((*Request)(nil), "kvrpcpb.Request")
	proto.RegisterType((*Response)(nil), "kvrpcpb.Response")
	proto.RegisterEnum("kvrpcpb.MessageType", MessageType_name, MessageType_value)
	proto.RegisterEnum("kvrpcpb.Op", Op_name, Op_value)
}

var fileDescriptor0 = []byte{
	// 1123 bytes of a gzipped FileDescriptorProto
	0x1f, 0x8b, 0x08, 0x00, 0x00, 0x09, 0x6e, 0x88, 0x02, 0xff, 0xac, 0x56, 0xdd, 0x6f, 0x1b, 0x45,
	0x10, 0x97, 0xbf, 0xed, 0x39, 0x7f, 0x5c, 0x36, 0x55, 0xe3, 0x26, 0x95, 0x48, 0xaf, 0x42, 0x94,
	0x3c, 0x04, 0x91, 0x07, 0x04, 0xa2, 0x02, 0x94, 0x10, 0x4a, 0x5a, 0xaa, 0x44, 0x4b, 0x55, 0x09,
	0x21, 0xb0, 0xce, 0x97, 0x25, 0xb1, 0x62, 0xfb, 0xae, 0x7b, 0xe7, 0x80, 0xff, 0x08, 0x1e, 0xe1,
	0x91, 0x3f, 0x15, 0xb1, 0x3b, 0xfb, 0x71, 0x7b, 0x3e, 0x37, 0x24, 0x52, 0xde, 0x6e, 0x67, 0x67,
	0x7e, 0xf3, 0x9b, 0xd9, 0xdf, 0xce, 0x1e, 0xf4, 0xae, 0xae, 0x79, 0x12, 0x25, 0xe3, 0xfd, 0x84,
	0xc7, 0x59, 0x4c, 0x5a, 0x7a, 0xb9, 0xdd, 0x9d, 0xb1, 0x2c, 0x34, 0xe6, 0xed, 0x1e, 0xe3, 0x3c,
	0xe6, 0x66, 0x19, 0x8c, 0xa1, 0xfd, 0x43, 0x1c, 0x5d, 0x9d, 0xcc, 0x7f, 0x8b, 0xc9, 0x13, 0xe8,
	0x26, 0x7c, 0x32, 0x0b, 0xf9, 0x72, 0x34, 0x15, 0xb6, 0x61, 0x65, 0xb7, 0xf2, 0xac, 0x4b, 0x3d,
	0x6d, 0x93, 0x6e, 0xd2, 0x45, 0x6e, 0x8d, 0xae, 0x19, 0x4f, 0x27, 0xf1, 0x7c, 0x58, 0x15, 0x2e,
	0x75, 0xea, 0x49, 0xdb, 0x5b, 0x65, 0x22, 0x3e, 0xd4, 0xae, 0xd8, 0x72, 0x58, 0xc3, 0x60, 0xf9,
	0x19, 0x4c, 0xa0, 0xfd, 0x8a, 0x2d, 0x8f, 0x65, 0x5e, 0xf2, 0x31, 0x34, 0xa5, 0x33, 0x3b, 0x47,
	0x74, 0xef, 0x60, 0x63, 0xdf, 0xb0, 0x36, 0x34, 0xa8, 0x76, 0x20, 0x8f, 0xa1, 0xc3, 0x59, 0xc6,
	0x97, 0xe1, 0x78, 0xca, 0x30, 0x51, 0x87, 0xe6, 0x06, 0xf2, 0x00, 0x1a, 0xe1, 0x38, 0xe6, 0x19,
	0x26, 0xea, 0x50, 0xb5, 0x08, 0x7e, 0x85, 0xd6, 0x51, 0x3c, 0xcf, 0xd8, 0x1f, 0x19, 0xd9, 0x91,
	0xe1, 0x17, 0x82, 0xd1, 0x68, 0xa2, 0x92, 0xd5, 0x69, 0x5b, 0x19, 0x4e, 0xce, 0xc9, 0x67, 0xd0,
	0xd5, 0x9b, 0x2c, 0x89, 0xa3, 0x4b, 0x84, 0xf7, 0x0e, 0x36, 0xf7, 0x75, 0xab, 0x28, 0xee, 0x1d,
	0xcb, 0x2d, 0xea, 0xf1, 0x7c, 0x11, 0x7c, 0x09, 0xbd, 0xa3, 0xd9, 0xf9, 0x0b, 0x96, 0x51, 0xf6,
	0x6e, 0xc1, 0xd2, 0xcc, 0x54, 0x5b, 0xb1, 0xd5, 0x92, 0x21, 0xb4, 0x8a, 0xdd, 0x31, 0xcb, 0xe0,
	0x14, 0xfa, 0x26, 0x38, 0x4d, 0xe2, 0x79, 0xca, 0xc8, 0x47, 0xd0, 0xc0, 0xe3, 0x28, 0x35, 0xc3,
	0xf4, 0x8b, 0xaa, 0x7d, 0x59, 0xed, 0x75, 0x38, 0x5d, 0xa8, 0x3e, 0x74, 0xa9, 0x5a, 0x04, 0xbf,
	0x20, 0xe0, 0x8f, 0x51, 0x38, 0x37, 0x74, 0x44, 0xd1, 0x69, 0x16, 0xf2, 0x6c, 0x94, 0x93, 0x6a,
	0xa3, 0x41, 0x00, 0x4a, 0x90, 0xe9, 0x64, 0x36, 0xc9, 0x10, 0xa4, 0x47, 0xd5, 0xc2, 0xe5, 0x5b,
	0x2b, 0xf2, 0xfd, 0x09, 0x9a, 0xaf, 0xae, 0xcf, 0xc2, 0x09, 0xbf, 0x3d, 0x4f, 0xdd, 0x8e, 0x6a,
	0xde, 0x0e, 0xcb, 0xbc, 0xe6, 0x32, 0xff, 0x1c, 0x06, 0x96, 0xb9, 0xee, 0xc5, 0x87, 0xd0, 0x48,
	0x44, 0xae, 0x54, 0xe4, 0xa8, 0x89, 0x1c, 0x83, 0x3c, 0x07, 0x72, 0xa0, 0x6a, 0x57, 0x34, 0xb1,
	0xfd, 0x7a, 0x91, 0x85, 0x99, 0x94, 0xda, 0x0e, 0x54, 0xe3, 0x04, 0x39, 0xf5, 0x0f, 0x3c, 0xeb,
	0x7f, 0x9a, 0x50, 0x61, 0xbe, 0x35, 0x95, 0x3f, 0x2b, 0x40, 0x04, 0x97, 0x33, 0xce, 0x7e, 0xe7,
	0x93, 0x8c, 0x99, 0x4e, 0x7e, 0x02, 0x9d, 0x99, 0xce, 0x63, 0x28, 0xe5, 0x65, 0x1b, 0x06, 0x34,
	0xf7, 0x29, 0xdd, 0x9e, 0x6a, 0xf9, 0xf6, 0x3c, 0x85, 0x9e, 0x3a, 0x9d, 0x62, 0xc3, 0xbb, 0x68,
	0xd4, 0xf7, 0x27, 0xf8, 0x06, 0x36, 0x0b, 0x74, 0x74, 0x7b, 0xc4, 0xc5, 0xc1, 0x16, 0x97, 0xc9,
	0xd8, 0x33, 0xd0, 0x0e, 0x01, 0x07, 0x5f, 0x20, 0x1c, 0xc5, 0x33, 0x71, 0xbc, 0xa6, 0x9c, 0x52,
	0xea, 0x4a, 0x39, 0x35, 0x21, 0x50, 0x17, 0x7d, 0x4a, 0x05, 0xf5, 0x9a, 0xa0, 0x8e, 0xdf, 0xe2,
	0x58, 0xfa, 0x11, 0x22, 0xad, 0x90, 0xee, 0x29, 0xab, 0x61, 0xfd, 0x15, 0x6c, 0x38, 0x39, 0xef,
	0xce, 0xf9, 0xa5, 0x8a, 0x9f, 0xb2, 0x70, 0xbe, 0x48, 0xde, 0x7f, 0xb9, 0x4a, 0x65, 0x54, 0xd7,
	0x74, 0xf0, 0x1c, 0x0f, 0xd4, 0x62, 0xdd, 0xf5, 0xae, 0x95, 0x2b, 0xae, 0xae, 0xab, 0xf8, 0x0c,
	0x1e, 0x89, 0x2c, 0x34, 0x9e, 0x4e, 0xc7, 0x61, 0x74, 0xf5, 0xe6, 0x92, 0xcd, 0x6f, 0x1c, 0x0b,
	0xff, 0x3f, 0x39, 0x83, 0x9f, 0x61, 0x7b, 0x1d, 0xe2, 0xfd, 0xcc, 0x8a, 0xbf, 0x2b, 0xb0, 0x65,
	0x4f, 0xe8, 0x1e, 0xd8, 0xde, 0x52, 0x18, 0xe4, 0x03, 0xf0, 0x2e, 0x58, 0xee, 0x53, 0x47, 0x1f,
	0x10, 0xa6, 0xb7, 0x76, 0xca, 0x0c, 0xcb, 0xbc, 0xee, 0xa7, 0xe6, 0x43, 0x14, 0xc2, 0x61, 0x98,
	0x45, 0x97, 0x4e, 0xb5, 0x46, 0xe5, 0x15, 0x47, 0xe5, 0xef, 0x1f, 0xda, 0xcf, 0xf1, 0x3a, 0xe6,
	0x18, 0x77, 0x9b, 0x56, 0xff, 0xd6, 0xa1, 0x65, 0xf2, 0x3e, 0x83, 0x7a, 0xb6, 0x4c, 0x98, 0x9e,
	0x57, 0x0f, 0xf2, 0x61, 0xc2, 0xd2, 0x34, 0xbc, 0x60, 0x6f, 0xc4, 0x1e, 0x45, 0x0f, 0xb2, 0x07,
	0xad, 0x48, 0xbd, 0x62, 0xfa, 0x61, 0xf2, 0xad, 0xb3, 0x7e, 0xdd, 0xa8, 0x71, 0x10, 0x2f, 0x99,
	0x17, 0xcd, 0xce, 0x47, 0xb2, 0xc7, 0x9c, 0xbd, 0xc3, 0x33, 0xf0, 0x0e, 0x1e, 0xe6, 0xfe, 0xee,
	0x6b, 0x45, 0x3b, 0x91, 0x59, 0x92, 0x2f, 0xa0, 0x2b, 0xe3, 0x52, 0x31, 0x82, 0x31, 0xb0, 0x8e,
	0x81, 0x5b, 0x6e, 0xa0, 0xf3, 0xb0, 0x50, 0x88, 0xec, 0x9a, 0x1c, 0x83, 0x2f, 0x43, 0x13, 0x3d,
	0xa2, 0x30, 0xbc, 0x81, 0xe1, 0x3b, 0x6e, 0xf8, 0xca, 0x44, 0xa5, 0xfd, 0xa8, 0x60, 0x23, 0x5f,
	0x83, 0xb4, 0x8c, 0xb4, 0x88, 0x24, 0x48, 0x13, 0x41, 0x1e, 0xb9, 0x20, 0x85, 0x29, 0x46, 0x25,
	0x65, 0x6b, 0x21, 0x87, 0x30, 0x40, 0x00, 0x75, 0xd1, 0x11, 0xa1, 0x85, 0x08, 0xdb, 0x05, 0x84,
	0xc2, 0x4c, 0x11, 0xf2, 0x74, 0x4d, 0xe4, 0x85, 0x22, 0xc1, 0xc7, 0xb6, 0x83, 0x6d, 0x84, 0x08,
	0x5c, 0x88, 0xf5, 0x97, 0x9c, 0xca, 0xc6, 0xd3, 0xb1, 0xee, 0xe7, 0x6b, 0x20, 0x4e, 0x35, 0x06,
	0xac, 0x83, 0x60, 0xbb, 0xe5, 0x8a, 0x56, 0xa0, 0x06, 0xb6, 0x30, 0x0d, 0xf7, 0x1d, 0x6c, 0x48,
	0xb8, 0xb1, 0xd4, 0x9d, 0x45, 0x83, 0x72, 0x93, 0x57, 0xc4, 0x8d, 0x4d, 0x76, 0x6c, 0xc1, 0x3f,
	0x0d, 0x68, 0x5b, 0xd1, 0xde, 0x5e, 0x81, 0x9f, 0xe6, 0xff, 0x47, 0x78, 0xff, 0x94, 0x0c, 0xfb,
	0xfb, 0xe6, 0xe7, 0x51, 0x5d, 0x3e, 0xf3, 0x6b, 0x84, 0x57, 0x50, 0x0b, 0x4a, 0x71, 0x4d, 0x13,
	0xad, 0xc4, 0xad, 0x92, 0x12, 0x15, 0x17, 0x14, 0x94, 0x5e, 0x93, 0xe7, 0xd0, 0x73, 0xb4, 0x28,
	0x62, 0x95, 0x18, 0x87, 0x65, 0x31, 0xea, 0x60, 0x2f, 0xca, 0x0d, 0xe4, 0x7b, 0xd5, 0x2a, 0x47,
	0x8e, 0x02, 0x41, 0xe9, 0xf1, 0xf1, 0x7a, 0x3d, 0x6a, 0x94, 0x41, 0x54, 0x34, 0x5a, 0x41, 0x19,
	0x45, 0x0a, 0x9c, 0xe6, 0x1a, 0x41, 0x15, 0x1e, 0x39, 0x25, 0x28, 0x6b, 0x32, 0x97, 0x23, 0x17,
	0xa5, 0x00, 0x69, 0x95, 0xcf, 0x6d, 0xe5, 0x75, 0xc2, 0x73, 0x73, 0x6c, 0xe4, 0xa5, 0xa2, 0x62,
	0x75, 0x29, 0x50, 0x94, 0x30, 0x9f, 0xde, 0x28, 0x4c, 0x8d, 0xd6, 0xcd, 0x95, 0x29, 0xb0, 0xce,
	0x60, 0xb3, 0x24, 0x4d, 0x81, 0xa7, 0xb4, 0xf9, 0xe4, 0x06, 0x6d, 0x6a, 0x34, 0xbf, 0x28, 0x4e,
	0x81, 0x78, 0xa2, 0xc4, 0xee, 0xaa, 0x53, 0x00, 0x42, 0xb9, 0xe7, 0xab, 0x73, 0x13, 0x7b, 0xee,
	0x1a, 0xf7, 0xfe, 0xaa, 0x80, 0xe7, 0xe8, 0x8f, 0x00, 0x34, 0x95, 0x52, 0xfc, 0x0a, 0xf1, 0xc4,
	0xdf, 0xbc, 0x3a, 0x68, 0xbf, 0x4a, 0x06, 0xe0, 0x39, 0x87, 0xe8, 0xd7, 0x48, 0x0f, 0x3a, 0x96,
	0xb2, 0x5f, 0x27, 0x7d, 0x80, 0xbc, 0xaf, 0x7e, 0x83, 0x3c, 0xc4, 0xe1, 0xbf, 0xd2, 0x21, 0xbf,
	0x29, 0x9e, 0x0a, 0x7f, 0xb5, 0x52, 0xbf, 0xa5, 0xd1, 0x0d, 0x33, 0xbf, 0xbd, 0xb7, 0x0b, 0xd5,
	0xd3, 0x84, 0xb4, 0xa0, 0x76, 0xb6, 0x90, 0x54, 0xc4, 0xc7, 0xb7, 0x6c, 0x2a, 0x68, 0xb4, 0xa1,
	0x2e, 0xff, 0xe5, 0xfc, 0xda, 0x7f, 0x01, 0x00, 0x00, 0xff, 0xff, 0x72, 0xb0, 0x78, 0x21, 0x70,
	0x0d, 0x00, 0x00,
}
