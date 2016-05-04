// Code generated by protoc-gen-go.
// source: pdpb.proto
// DO NOT EDIT!

/*
Package pdpb is a generated protocol buffer package.

It is generated from these files:
	pdpb.proto

It has these top-level messages:
	Leader
	TsoRequest
	Timestamp
	TsoResponse
	BootstrapRequest
	BootstrapResponse
	IsBootstrappedRequest
	IsBootstrappedResponse
	AllocIdRequest
	AllocIdResponse
	GetStoreRequest
	GetStoreResponse
	PutStoreRequest
	PutStoreResponse
	GetRegionRequest
	GetRegionResponse
	GetClusterConfigRequest
	GetClusterConfigResponse
	PutClusterConfigRequest
	PutClusterConfigResponse
	AskChangePeerRequest
	AskChangePeerResponse
	AskSplitRequest
	AskSplitResponse
	RequestHeader
	ResponseHeader
	Request
	Response
	BootstrappedError
	Error
*/
package pdpb

import proto "github.com/golang/protobuf/proto"
import fmt "fmt"
import math "math"
import metapb "github.com/pingcap/kvproto/pkg/metapb"

// Reference imports to suppress errors if they are not otherwise used.
var _ = proto.Marshal
var _ = fmt.Errorf
var _ = math.Inf

// This is a compile-time assertion to ensure that this generated file
// is compatible with the proto package it is being compiled against.
const _ = proto.ProtoPackageIsVersion1

type CommandType int32

const (
	CommandType_Invalid          CommandType = 0
	CommandType_Tso              CommandType = 1
	CommandType_Bootstrap        CommandType = 2
	CommandType_IsBootstrapped   CommandType = 3
	CommandType_AllocId          CommandType = 4
	CommandType_GetStore         CommandType = 5
	CommandType_PutStore         CommandType = 6
	CommandType_DeleteMeta       CommandType = 7
	CommandType_AskChangePeer    CommandType = 8
	CommandType_AskSplit         CommandType = 9
	CommandType_GetRegion        CommandType = 10
	CommandType_GetClusterConfig CommandType = 11
	CommandType_PutClusterConfig CommandType = 12
)

var CommandType_name = map[int32]string{
	0:  "Invalid",
	1:  "Tso",
	2:  "Bootstrap",
	3:  "IsBootstrapped",
	4:  "AllocId",
	5:  "GetStore",
	6:  "PutStore",
	7:  "DeleteMeta",
	8:  "AskChangePeer",
	9:  "AskSplit",
	10: "GetRegion",
	11: "GetClusterConfig",
	12: "PutClusterConfig",
}
var CommandType_value = map[string]int32{
	"Invalid":          0,
	"Tso":              1,
	"Bootstrap":        2,
	"IsBootstrapped":   3,
	"AllocId":          4,
	"GetStore":         5,
	"PutStore":         6,
	"DeleteMeta":       7,
	"AskChangePeer":    8,
	"AskSplit":         9,
	"GetRegion":        10,
	"GetClusterConfig": 11,
	"PutClusterConfig": 12,
}

func (x CommandType) Enum() *CommandType {
	p := new(CommandType)
	*p = x
	return p
}
func (x CommandType) String() string {
	return proto.EnumName(CommandType_name, int32(x))
}
func (x *CommandType) UnmarshalJSON(data []byte) error {
	value, err := proto.UnmarshalJSONEnum(CommandType_value, data, "CommandType")
	if err != nil {
		return err
	}
	*x = CommandType(value)
	return nil
}
func (CommandType) EnumDescriptor() ([]byte, []int) { return fileDescriptor0, []int{0} }

type Leader struct {
	Addr             *string `protobuf:"bytes,1,opt,name=addr" json:"addr,omitempty"`
	Pid              *int64  `protobuf:"varint,2,opt,name=pid" json:"pid,omitempty"`
	XXX_unrecognized []byte  `json:"-"`
}

func (m *Leader) Reset()                    { *m = Leader{} }
func (m *Leader) String() string            { return proto.CompactTextString(m) }
func (*Leader) ProtoMessage()               {}
func (*Leader) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{0} }

func (m *Leader) GetAddr() string {
	if m != nil && m.Addr != nil {
		return *m.Addr
	}
	return ""
}

func (m *Leader) GetPid() int64 {
	if m != nil && m.Pid != nil {
		return *m.Pid
	}
	return 0
}

type TsoRequest struct {
	Number           *uint32 `protobuf:"varint,1,opt,name=number" json:"number,omitempty"`
	XXX_unrecognized []byte  `json:"-"`
}

func (m *TsoRequest) Reset()                    { *m = TsoRequest{} }
func (m *TsoRequest) String() string            { return proto.CompactTextString(m) }
func (*TsoRequest) ProtoMessage()               {}
func (*TsoRequest) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{1} }

func (m *TsoRequest) GetNumber() uint32 {
	if m != nil && m.Number != nil {
		return *m.Number
	}
	return 0
}

type Timestamp struct {
	Physical         *int64 `protobuf:"varint,1,opt,name=physical" json:"physical,omitempty"`
	Logical          *int64 `protobuf:"varint,2,opt,name=logical" json:"logical,omitempty"`
	XXX_unrecognized []byte `json:"-"`
}

func (m *Timestamp) Reset()                    { *m = Timestamp{} }
func (m *Timestamp) String() string            { return proto.CompactTextString(m) }
func (*Timestamp) ProtoMessage()               {}
func (*Timestamp) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{2} }

func (m *Timestamp) GetPhysical() int64 {
	if m != nil && m.Physical != nil {
		return *m.Physical
	}
	return 0
}

func (m *Timestamp) GetLogical() int64 {
	if m != nil && m.Logical != nil {
		return *m.Logical
	}
	return 0
}

type TsoResponse struct {
	Timestamps       []*Timestamp `protobuf:"bytes,1,rep,name=timestamps" json:"timestamps,omitempty"`
	XXX_unrecognized []byte       `json:"-"`
}

func (m *TsoResponse) Reset()                    { *m = TsoResponse{} }
func (m *TsoResponse) String() string            { return proto.CompactTextString(m) }
func (*TsoResponse) ProtoMessage()               {}
func (*TsoResponse) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{3} }

func (m *TsoResponse) GetTimestamps() []*Timestamp {
	if m != nil {
		return m.Timestamps
	}
	return nil
}

type BootstrapRequest struct {
	Store            *metapb.Store  `protobuf:"bytes,1,opt,name=store" json:"store,omitempty"`
	Region           *metapb.Region `protobuf:"bytes,2,opt,name=region" json:"region,omitempty"`
	XXX_unrecognized []byte         `json:"-"`
}

func (m *BootstrapRequest) Reset()                    { *m = BootstrapRequest{} }
func (m *BootstrapRequest) String() string            { return proto.CompactTextString(m) }
func (*BootstrapRequest) ProtoMessage()               {}
func (*BootstrapRequest) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{4} }

func (m *BootstrapRequest) GetStore() *metapb.Store {
	if m != nil {
		return m.Store
	}
	return nil
}

func (m *BootstrapRequest) GetRegion() *metapb.Region {
	if m != nil {
		return m.Region
	}
	return nil
}

type BootstrapResponse struct {
	XXX_unrecognized []byte `json:"-"`
}

func (m *BootstrapResponse) Reset()                    { *m = BootstrapResponse{} }
func (m *BootstrapResponse) String() string            { return proto.CompactTextString(m) }
func (*BootstrapResponse) ProtoMessage()               {}
func (*BootstrapResponse) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{5} }

type IsBootstrappedRequest struct {
	XXX_unrecognized []byte `json:"-"`
}

func (m *IsBootstrappedRequest) Reset()                    { *m = IsBootstrappedRequest{} }
func (m *IsBootstrappedRequest) String() string            { return proto.CompactTextString(m) }
func (*IsBootstrappedRequest) ProtoMessage()               {}
func (*IsBootstrappedRequest) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{6} }

type IsBootstrappedResponse struct {
	Bootstrapped     *bool  `protobuf:"varint,1,opt,name=bootstrapped" json:"bootstrapped,omitempty"`
	XXX_unrecognized []byte `json:"-"`
}

func (m *IsBootstrappedResponse) Reset()                    { *m = IsBootstrappedResponse{} }
func (m *IsBootstrappedResponse) String() string            { return proto.CompactTextString(m) }
func (*IsBootstrappedResponse) ProtoMessage()               {}
func (*IsBootstrappedResponse) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{7} }

func (m *IsBootstrappedResponse) GetBootstrapped() bool {
	if m != nil && m.Bootstrapped != nil {
		return *m.Bootstrapped
	}
	return false
}

type AllocIdRequest struct {
	XXX_unrecognized []byte `json:"-"`
}

func (m *AllocIdRequest) Reset()                    { *m = AllocIdRequest{} }
func (m *AllocIdRequest) String() string            { return proto.CompactTextString(m) }
func (*AllocIdRequest) ProtoMessage()               {}
func (*AllocIdRequest) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{8} }

type AllocIdResponse struct {
	Id               *uint64 `protobuf:"varint,1,opt,name=id" json:"id,omitempty"`
	XXX_unrecognized []byte  `json:"-"`
}

func (m *AllocIdResponse) Reset()                    { *m = AllocIdResponse{} }
func (m *AllocIdResponse) String() string            { return proto.CompactTextString(m) }
func (*AllocIdResponse) ProtoMessage()               {}
func (*AllocIdResponse) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{9} }

func (m *AllocIdResponse) GetId() uint64 {
	if m != nil && m.Id != nil {
		return *m.Id
	}
	return 0
}

type GetStoreRequest struct {
	StoreId          *uint64 `protobuf:"varint,1,opt,name=store_id,json=storeId" json:"store_id,omitempty"`
	XXX_unrecognized []byte  `json:"-"`
}

func (m *GetStoreRequest) Reset()                    { *m = GetStoreRequest{} }
func (m *GetStoreRequest) String() string            { return proto.CompactTextString(m) }
func (*GetStoreRequest) ProtoMessage()               {}
func (*GetStoreRequest) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{10} }

func (m *GetStoreRequest) GetStoreId() uint64 {
	if m != nil && m.StoreId != nil {
		return *m.StoreId
	}
	return 0
}

type GetStoreResponse struct {
	Store            *metapb.Store `protobuf:"bytes,1,opt,name=store" json:"store,omitempty"`
	XXX_unrecognized []byte        `json:"-"`
}

func (m *GetStoreResponse) Reset()                    { *m = GetStoreResponse{} }
func (m *GetStoreResponse) String() string            { return proto.CompactTextString(m) }
func (*GetStoreResponse) ProtoMessage()               {}
func (*GetStoreResponse) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{11} }

func (m *GetStoreResponse) GetStore() *metapb.Store {
	if m != nil {
		return m.Store
	}
	return nil
}

type PutStoreRequest struct {
	Store            *metapb.Store `protobuf:"bytes,1,opt,name=store" json:"store,omitempty"`
	XXX_unrecognized []byte        `json:"-"`
}

func (m *PutStoreRequest) Reset()                    { *m = PutStoreRequest{} }
func (m *PutStoreRequest) String() string            { return proto.CompactTextString(m) }
func (*PutStoreRequest) ProtoMessage()               {}
func (*PutStoreRequest) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{12} }

func (m *PutStoreRequest) GetStore() *metapb.Store {
	if m != nil {
		return m.Store
	}
	return nil
}

type PutStoreResponse struct {
	XXX_unrecognized []byte `json:"-"`
}

func (m *PutStoreResponse) Reset()                    { *m = PutStoreResponse{} }
func (m *PutStoreResponse) String() string            { return proto.CompactTextString(m) }
func (*PutStoreResponse) ProtoMessage()               {}
func (*PutStoreResponse) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{13} }

type GetRegionRequest struct {
	RegionKey        []byte `protobuf:"bytes,1,opt,name=region_key,json=regionKey" json:"region_key,omitempty"`
	XXX_unrecognized []byte `json:"-"`
}

func (m *GetRegionRequest) Reset()                    { *m = GetRegionRequest{} }
func (m *GetRegionRequest) String() string            { return proto.CompactTextString(m) }
func (*GetRegionRequest) ProtoMessage()               {}
func (*GetRegionRequest) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{14} }

func (m *GetRegionRequest) GetRegionKey() []byte {
	if m != nil {
		return m.RegionKey
	}
	return nil
}

type GetRegionResponse struct {
	Region           *metapb.Region `protobuf:"bytes,1,opt,name=region" json:"region,omitempty"`
	XXX_unrecognized []byte         `json:"-"`
}

func (m *GetRegionResponse) Reset()                    { *m = GetRegionResponse{} }
func (m *GetRegionResponse) String() string            { return proto.CompactTextString(m) }
func (*GetRegionResponse) ProtoMessage()               {}
func (*GetRegionResponse) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{15} }

func (m *GetRegionResponse) GetRegion() *metapb.Region {
	if m != nil {
		return m.Region
	}
	return nil
}

type GetClusterConfigRequest struct {
	ClusterName      *string `protobuf:"bytes,1,opt,name=cluster_name,json=clusterName" json:"cluster_name,omitempty"`
	XXX_unrecognized []byte  `json:"-"`
}

func (m *GetClusterConfigRequest) Reset()                    { *m = GetClusterConfigRequest{} }
func (m *GetClusterConfigRequest) String() string            { return proto.CompactTextString(m) }
func (*GetClusterConfigRequest) ProtoMessage()               {}
func (*GetClusterConfigRequest) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{16} }

func (m *GetClusterConfigRequest) GetClusterName() string {
	if m != nil && m.ClusterName != nil {
		return *m.ClusterName
	}
	return ""
}

type GetClusterConfigResponse struct {
	Cluster          *metapb.Cluster `protobuf:"bytes,1,opt,name=cluster" json:"cluster,omitempty"`
	XXX_unrecognized []byte          `json:"-"`
}

func (m *GetClusterConfigResponse) Reset()                    { *m = GetClusterConfigResponse{} }
func (m *GetClusterConfigResponse) String() string            { return proto.CompactTextString(m) }
func (*GetClusterConfigResponse) ProtoMessage()               {}
func (*GetClusterConfigResponse) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{17} }

func (m *GetClusterConfigResponse) GetCluster() *metapb.Cluster {
	if m != nil {
		return m.Cluster
	}
	return nil
}

type PutClusterConfigRequest struct {
	Cluster          *metapb.Cluster `protobuf:"bytes,1,opt,name=cluster" json:"cluster,omitempty"`
	XXX_unrecognized []byte          `json:"-"`
}

func (m *PutClusterConfigRequest) Reset()                    { *m = PutClusterConfigRequest{} }
func (m *PutClusterConfigRequest) String() string            { return proto.CompactTextString(m) }
func (*PutClusterConfigRequest) ProtoMessage()               {}
func (*PutClusterConfigRequest) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{18} }

func (m *PutClusterConfigRequest) GetCluster() *metapb.Cluster {
	if m != nil {
		return m.Cluster
	}
	return nil
}

type PutClusterConfigResponse struct {
	XXX_unrecognized []byte `json:"-"`
}

func (m *PutClusterConfigResponse) Reset()                    { *m = PutClusterConfigResponse{} }
func (m *PutClusterConfigResponse) String() string            { return proto.CompactTextString(m) }
func (*PutClusterConfigResponse) ProtoMessage()               {}
func (*PutClusterConfigResponse) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{19} }

type AskChangePeerRequest struct {
	Region *metapb.Region `protobuf:"bytes,1,opt,name=region" json:"region,omitempty"`
	// The current leader store id of the region.
	// Pd can first try to send command to this store,
	// if the peer is not leader now, pd will try to
	// find the new leader of the region and then send
	// command again.
	LeaderStoreId    *uint64 `protobuf:"varint,2,opt,name=leader_store_id,json=leaderStoreId" json:"leader_store_id,omitempty"`
	XXX_unrecognized []byte  `json:"-"`
}

func (m *AskChangePeerRequest) Reset()                    { *m = AskChangePeerRequest{} }
func (m *AskChangePeerRequest) String() string            { return proto.CompactTextString(m) }
func (*AskChangePeerRequest) ProtoMessage()               {}
func (*AskChangePeerRequest) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{20} }

func (m *AskChangePeerRequest) GetRegion() *metapb.Region {
	if m != nil {
		return m.Region
	}
	return nil
}

func (m *AskChangePeerRequest) GetLeaderStoreId() uint64 {
	if m != nil && m.LeaderStoreId != nil {
		return *m.LeaderStoreId
	}
	return 0
}

type AskChangePeerResponse struct {
	XXX_unrecognized []byte `json:"-"`
}

func (m *AskChangePeerResponse) Reset()                    { *m = AskChangePeerResponse{} }
func (m *AskChangePeerResponse) String() string            { return proto.CompactTextString(m) }
func (*AskChangePeerResponse) ProtoMessage()               {}
func (*AskChangePeerResponse) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{21} }

type AskSplitRequest struct {
	Region           *metapb.Region `protobuf:"bytes,1,opt,name=region" json:"region,omitempty"`
	SplitKey         []byte         `protobuf:"bytes,2,opt,name=split_key,json=splitKey" json:"split_key,omitempty"`
	LeaderStoreId    *uint64        `protobuf:"varint,3,opt,name=leader_store_id,json=leaderStoreId" json:"leader_store_id,omitempty"`
	XXX_unrecognized []byte         `json:"-"`
}

func (m *AskSplitRequest) Reset()                    { *m = AskSplitRequest{} }
func (m *AskSplitRequest) String() string            { return proto.CompactTextString(m) }
func (*AskSplitRequest) ProtoMessage()               {}
func (*AskSplitRequest) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{22} }

func (m *AskSplitRequest) GetRegion() *metapb.Region {
	if m != nil {
		return m.Region
	}
	return nil
}

func (m *AskSplitRequest) GetSplitKey() []byte {
	if m != nil {
		return m.SplitKey
	}
	return nil
}

func (m *AskSplitRequest) GetLeaderStoreId() uint64 {
	if m != nil && m.LeaderStoreId != nil {
		return *m.LeaderStoreId
	}
	return 0
}

type AskSplitResponse struct {
	XXX_unrecognized []byte `json:"-"`
}

func (m *AskSplitResponse) Reset()                    { *m = AskSplitResponse{} }
func (m *AskSplitResponse) String() string            { return proto.CompactTextString(m) }
func (*AskSplitResponse) ProtoMessage()               {}
func (*AskSplitResponse) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{23} }

type RequestHeader struct {
	// 16 bytes, to distinguish request.
	Uuid             []byte  `protobuf:"bytes,1,opt,name=uuid" json:"uuid,omitempty"`
	ClusterName      *string `protobuf:"bytes,2,opt,name=cluster_name,json=clusterName" json:"cluster_name,omitempty"`
	XXX_unrecognized []byte  `json:"-"`
}

func (m *RequestHeader) Reset()                    { *m = RequestHeader{} }
func (m *RequestHeader) String() string            { return proto.CompactTextString(m) }
func (*RequestHeader) ProtoMessage()               {}
func (*RequestHeader) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{24} }

func (m *RequestHeader) GetUuid() []byte {
	if m != nil {
		return m.Uuid
	}
	return nil
}

func (m *RequestHeader) GetClusterName() string {
	if m != nil && m.ClusterName != nil {
		return *m.ClusterName
	}
	return ""
}

type ResponseHeader struct {
	// 16 bytes, to distinguish request.
	Uuid             []byte  `protobuf:"bytes,1,opt,name=uuid" json:"uuid,omitempty"`
	ClusterName      *string `protobuf:"bytes,2,opt,name=cluster_name,json=clusterName" json:"cluster_name,omitempty"`
	Error            *Error  `protobuf:"bytes,3,opt,name=error" json:"error,omitempty"`
	XXX_unrecognized []byte  `json:"-"`
}

func (m *ResponseHeader) Reset()                    { *m = ResponseHeader{} }
func (m *ResponseHeader) String() string            { return proto.CompactTextString(m) }
func (*ResponseHeader) ProtoMessage()               {}
func (*ResponseHeader) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{25} }

func (m *ResponseHeader) GetUuid() []byte {
	if m != nil {
		return m.Uuid
	}
	return nil
}

func (m *ResponseHeader) GetClusterName() string {
	if m != nil && m.ClusterName != nil {
		return *m.ClusterName
	}
	return ""
}

func (m *ResponseHeader) GetError() *Error {
	if m != nil {
		return m.Error
	}
	return nil
}

type Request struct {
	Header           *RequestHeader           `protobuf:"bytes,1,opt,name=header" json:"header,omitempty"`
	CmdType          *CommandType             `protobuf:"varint,2,opt,name=cmd_type,json=cmdType,enum=pdpb.CommandType" json:"cmd_type,omitempty"`
	Tso              *TsoRequest              `protobuf:"bytes,3,opt,name=tso" json:"tso,omitempty"`
	Bootstrap        *BootstrapRequest        `protobuf:"bytes,4,opt,name=bootstrap" json:"bootstrap,omitempty"`
	IsBootstrapped   *IsBootstrappedRequest   `protobuf:"bytes,5,opt,name=is_bootstrapped,json=isBootstrapped" json:"is_bootstrapped,omitempty"`
	AllocId          *AllocIdRequest          `protobuf:"bytes,6,opt,name=alloc_id,json=allocId" json:"alloc_id,omitempty"`
	AskChangePeer    *AskChangePeerRequest    `protobuf:"bytes,7,opt,name=ask_change_peer,json=askChangePeer" json:"ask_change_peer,omitempty"`
	AskSplit         *AskSplitRequest         `protobuf:"bytes,8,opt,name=ask_split,json=askSplit" json:"ask_split,omitempty"`
	GetStore         *GetStoreRequest         `protobuf:"bytes,9,opt,name=get_store,json=getStore" json:"get_store,omitempty"`
	GetRegion        *GetRegionRequest        `protobuf:"bytes,10,opt,name=get_region,json=getRegion" json:"get_region,omitempty"`
	GetClusterConfig *GetClusterConfigRequest `protobuf:"bytes,11,opt,name=get_cluster_config,json=getClusterConfig" json:"get_cluster_config,omitempty"`
	PutStore         *PutStoreRequest         `protobuf:"bytes,12,opt,name=put_store,json=putStore" json:"put_store,omitempty"`
	PutClusterConfig *PutClusterConfigRequest `protobuf:"bytes,13,opt,name=put_cluster_config,json=putClusterConfig" json:"put_cluster_config,omitempty"`
	XXX_unrecognized []byte                   `json:"-"`
}

func (m *Request) Reset()                    { *m = Request{} }
func (m *Request) String() string            { return proto.CompactTextString(m) }
func (*Request) ProtoMessage()               {}
func (*Request) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{26} }

func (m *Request) GetHeader() *RequestHeader {
	if m != nil {
		return m.Header
	}
	return nil
}

func (m *Request) GetCmdType() CommandType {
	if m != nil && m.CmdType != nil {
		return *m.CmdType
	}
	return CommandType_Invalid
}

func (m *Request) GetTso() *TsoRequest {
	if m != nil {
		return m.Tso
	}
	return nil
}

func (m *Request) GetBootstrap() *BootstrapRequest {
	if m != nil {
		return m.Bootstrap
	}
	return nil
}

func (m *Request) GetIsBootstrapped() *IsBootstrappedRequest {
	if m != nil {
		return m.IsBootstrapped
	}
	return nil
}

func (m *Request) GetAllocId() *AllocIdRequest {
	if m != nil {
		return m.AllocId
	}
	return nil
}

func (m *Request) GetAskChangePeer() *AskChangePeerRequest {
	if m != nil {
		return m.AskChangePeer
	}
	return nil
}

func (m *Request) GetAskSplit() *AskSplitRequest {
	if m != nil {
		return m.AskSplit
	}
	return nil
}

func (m *Request) GetGetStore() *GetStoreRequest {
	if m != nil {
		return m.GetStore
	}
	return nil
}

func (m *Request) GetGetRegion() *GetRegionRequest {
	if m != nil {
		return m.GetRegion
	}
	return nil
}

func (m *Request) GetGetClusterConfig() *GetClusterConfigRequest {
	if m != nil {
		return m.GetClusterConfig
	}
	return nil
}

func (m *Request) GetPutStore() *PutStoreRequest {
	if m != nil {
		return m.PutStore
	}
	return nil
}

func (m *Request) GetPutClusterConfig() *PutClusterConfigRequest {
	if m != nil {
		return m.PutClusterConfig
	}
	return nil
}

type Response struct {
	Header           *ResponseHeader           `protobuf:"bytes,1,opt,name=header" json:"header,omitempty"`
	CmdType          *CommandType              `protobuf:"varint,2,opt,name=cmd_type,json=cmdType,enum=pdpb.CommandType" json:"cmd_type,omitempty"`
	Tso              *TsoResponse              `protobuf:"bytes,3,opt,name=tso" json:"tso,omitempty"`
	Bootstrap        *BootstrapResponse        `protobuf:"bytes,4,opt,name=bootstrap" json:"bootstrap,omitempty"`
	IsBootstrapped   *IsBootstrappedResponse   `protobuf:"bytes,5,opt,name=is_bootstrapped,json=isBootstrapped" json:"is_bootstrapped,omitempty"`
	AllocId          *AllocIdResponse          `protobuf:"bytes,6,opt,name=alloc_id,json=allocId" json:"alloc_id,omitempty"`
	AskChangePeer    *AskChangePeerResponse    `protobuf:"bytes,7,opt,name=ask_change_peer,json=askChangePeer" json:"ask_change_peer,omitempty"`
	AskSplit         *AskSplitResponse         `protobuf:"bytes,8,opt,name=ask_split,json=askSplit" json:"ask_split,omitempty"`
	GetStore         *GetStoreResponse         `protobuf:"bytes,9,opt,name=get_store,json=getStore" json:"get_store,omitempty"`
	GetRegion        *GetRegionResponse        `protobuf:"bytes,10,opt,name=get_region,json=getRegion" json:"get_region,omitempty"`
	GetClusterConfig *GetClusterConfigResponse `protobuf:"bytes,11,opt,name=get_cluster_config,json=getClusterConfig" json:"get_cluster_config,omitempty"`
	PutStore         *PutStoreResponse         `protobuf:"bytes,12,opt,name=put_store,json=putStore" json:"put_store,omitempty"`
	PutClusterConfig *PutClusterConfigResponse `protobuf:"bytes,13,opt,name=put_cluster_config,json=putClusterConfig" json:"put_cluster_config,omitempty"`
	XXX_unrecognized []byte                    `json:"-"`
}

func (m *Response) Reset()                    { *m = Response{} }
func (m *Response) String() string            { return proto.CompactTextString(m) }
func (*Response) ProtoMessage()               {}
func (*Response) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{27} }

func (m *Response) GetHeader() *ResponseHeader {
	if m != nil {
		return m.Header
	}
	return nil
}

func (m *Response) GetCmdType() CommandType {
	if m != nil && m.CmdType != nil {
		return *m.CmdType
	}
	return CommandType_Invalid
}

func (m *Response) GetTso() *TsoResponse {
	if m != nil {
		return m.Tso
	}
	return nil
}

func (m *Response) GetBootstrap() *BootstrapResponse {
	if m != nil {
		return m.Bootstrap
	}
	return nil
}

func (m *Response) GetIsBootstrapped() *IsBootstrappedResponse {
	if m != nil {
		return m.IsBootstrapped
	}
	return nil
}

func (m *Response) GetAllocId() *AllocIdResponse {
	if m != nil {
		return m.AllocId
	}
	return nil
}

func (m *Response) GetAskChangePeer() *AskChangePeerResponse {
	if m != nil {
		return m.AskChangePeer
	}
	return nil
}

func (m *Response) GetAskSplit() *AskSplitResponse {
	if m != nil {
		return m.AskSplit
	}
	return nil
}

func (m *Response) GetGetStore() *GetStoreResponse {
	if m != nil {
		return m.GetStore
	}
	return nil
}

func (m *Response) GetGetRegion() *GetRegionResponse {
	if m != nil {
		return m.GetRegion
	}
	return nil
}

func (m *Response) GetGetClusterConfig() *GetClusterConfigResponse {
	if m != nil {
		return m.GetClusterConfig
	}
	return nil
}

func (m *Response) GetPutStore() *PutStoreResponse {
	if m != nil {
		return m.PutStore
	}
	return nil
}

func (m *Response) GetPutClusterConfig() *PutClusterConfigResponse {
	if m != nil {
		return m.PutClusterConfig
	}
	return nil
}

type BootstrappedError struct {
	XXX_unrecognized []byte `json:"-"`
}

func (m *BootstrappedError) Reset()                    { *m = BootstrappedError{} }
func (m *BootstrappedError) String() string            { return proto.CompactTextString(m) }
func (*BootstrappedError) ProtoMessage()               {}
func (*BootstrappedError) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{28} }

type Error struct {
	Message          *string            `protobuf:"bytes,1,opt,name=message" json:"message,omitempty"`
	Bootstrapped     *BootstrappedError `protobuf:"bytes,2,opt,name=bootstrapped" json:"bootstrapped,omitempty"`
	XXX_unrecognized []byte             `json:"-"`
}

func (m *Error) Reset()                    { *m = Error{} }
func (m *Error) String() string            { return proto.CompactTextString(m) }
func (*Error) ProtoMessage()               {}
func (*Error) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{29} }

func (m *Error) GetMessage() string {
	if m != nil && m.Message != nil {
		return *m.Message
	}
	return ""
}

func (m *Error) GetBootstrapped() *BootstrappedError {
	if m != nil {
		return m.Bootstrapped
	}
	return nil
}

func init() {
	proto.RegisterType((*Leader)(nil), "pdpb.Leader")
	proto.RegisterType((*TsoRequest)(nil), "pdpb.TsoRequest")
	proto.RegisterType((*Timestamp)(nil), "pdpb.Timestamp")
	proto.RegisterType((*TsoResponse)(nil), "pdpb.TsoResponse")
	proto.RegisterType((*BootstrapRequest)(nil), "pdpb.BootstrapRequest")
	proto.RegisterType((*BootstrapResponse)(nil), "pdpb.BootstrapResponse")
	proto.RegisterType((*IsBootstrappedRequest)(nil), "pdpb.IsBootstrappedRequest")
	proto.RegisterType((*IsBootstrappedResponse)(nil), "pdpb.IsBootstrappedResponse")
	proto.RegisterType((*AllocIdRequest)(nil), "pdpb.AllocIdRequest")
	proto.RegisterType((*AllocIdResponse)(nil), "pdpb.AllocIdResponse")
	proto.RegisterType((*GetStoreRequest)(nil), "pdpb.GetStoreRequest")
	proto.RegisterType((*GetStoreResponse)(nil), "pdpb.GetStoreResponse")
	proto.RegisterType((*PutStoreRequest)(nil), "pdpb.PutStoreRequest")
	proto.RegisterType((*PutStoreResponse)(nil), "pdpb.PutStoreResponse")
	proto.RegisterType((*GetRegionRequest)(nil), "pdpb.GetRegionRequest")
	proto.RegisterType((*GetRegionResponse)(nil), "pdpb.GetRegionResponse")
	proto.RegisterType((*GetClusterConfigRequest)(nil), "pdpb.GetClusterConfigRequest")
	proto.RegisterType((*GetClusterConfigResponse)(nil), "pdpb.GetClusterConfigResponse")
	proto.RegisterType((*PutClusterConfigRequest)(nil), "pdpb.PutClusterConfigRequest")
	proto.RegisterType((*PutClusterConfigResponse)(nil), "pdpb.PutClusterConfigResponse")
	proto.RegisterType((*AskChangePeerRequest)(nil), "pdpb.AskChangePeerRequest")
	proto.RegisterType((*AskChangePeerResponse)(nil), "pdpb.AskChangePeerResponse")
	proto.RegisterType((*AskSplitRequest)(nil), "pdpb.AskSplitRequest")
	proto.RegisterType((*AskSplitResponse)(nil), "pdpb.AskSplitResponse")
	proto.RegisterType((*RequestHeader)(nil), "pdpb.RequestHeader")
	proto.RegisterType((*ResponseHeader)(nil), "pdpb.ResponseHeader")
	proto.RegisterType((*Request)(nil), "pdpb.Request")
	proto.RegisterType((*Response)(nil), "pdpb.Response")
	proto.RegisterType((*BootstrappedError)(nil), "pdpb.BootstrappedError")
	proto.RegisterType((*Error)(nil), "pdpb.Error")
	proto.RegisterEnum("pdpb.CommandType", CommandType_name, CommandType_value)
}

var fileDescriptor0 = []byte{
	// 1124 bytes of a gzipped FileDescriptorProto
	0x1f, 0x8b, 0x08, 0x00, 0x00, 0x09, 0x6e, 0x88, 0x02, 0xff, 0xa4, 0x96, 0x5d, 0x73, 0xdb, 0x44,
	0x17, 0xc7, 0x9f, 0xd8, 0xb1, 0x2d, 0x1f, 0xf9, 0x65, 0xb3, 0x4d, 0x1c, 0x3f, 0x09, 0xed, 0x50,
	0x95, 0x61, 0x0a, 0x64, 0x02, 0x38, 0xb4, 0x5c, 0xb4, 0xc3, 0x4c, 0x9a, 0x04, 0xc8, 0x34, 0x30,
	0x9d, 0x4d, 0xae, 0xf1, 0x28, 0xd6, 0xd6, 0x11, 0xb1, 0x2d, 0x21, 0xc9, 0xcc, 0xe4, 0x86, 0x2f,
	0xca, 0x3d, 0x5f, 0x80, 0x2f, 0xc0, 0xbe, 0x9c, 0x95, 0xf5, 0xe6, 0x4c, 0x0a, 0x77, 0xbb, 0x67,
	0xcf, 0xff, 0xec, 0xd1, 0xd9, 0xdf, 0x1e, 0x2d, 0x40, 0xe8, 0x85, 0xd7, 0x87, 0x61, 0x14, 0x24,
	0x01, 0xdd, 0x94, 0xe3, 0xbd, 0xce, 0x9c, 0x27, 0xae, 0xb1, 0x39, 0x87, 0xd0, 0xbc, 0xe0, 0xae,
	0xc7, 0x23, 0x4a, 0x61, 0xd3, 0xf5, 0xbc, 0x68, 0xb8, 0xf1, 0xf1, 0xc6, 0xf3, 0x36, 0x53, 0x63,
	0x4a, 0xa0, 0x1e, 0xfa, 0xde, 0xb0, 0x26, 0x4c, 0x75, 0x26, 0x87, 0xce, 0x27, 0x00, 0x57, 0x71,
	0xc0, 0xf8, 0x6f, 0x4b, 0x1e, 0x27, 0x74, 0x00, 0xcd, 0xc5, 0x72, 0x7e, 0xcd, 0xb5, 0xaa, 0xcb,
	0x70, 0xe6, 0x1c, 0x43, 0xfb, 0xca, 0x9f, 0x0b, 0x0f, 0x77, 0x1e, 0xd2, 0x3d, 0xb0, 0xc2, 0x9b,
	0xbb, 0xd8, 0x9f, 0xb8, 0x33, 0xe5, 0x56, 0x67, 0xe9, 0x9c, 0x0e, 0xa1, 0x35, 0x0b, 0xa6, 0x6a,
	0x49, 0x6f, 0x62, 0xa6, 0xce, 0x77, 0x60, 0xab, 0x8d, 0xe2, 0x30, 0x58, 0xc4, 0x9c, 0x7e, 0x09,
	0x90, 0x98, 0x88, 0xb1, 0x08, 0x53, 0x7f, 0x6e, 0x8f, 0xfa, 0x87, 0xea, 0xe3, 0xd2, 0x9d, 0x58,
	0xc6, 0xc5, 0x19, 0x03, 0x79, 0x13, 0x04, 0x49, 0x9c, 0x44, 0x6e, 0x68, 0xd2, 0x7d, 0x06, 0x8d,
	0x38, 0x09, 0x22, 0xae, 0xd2, 0xb0, 0x47, 0xdd, 0x43, 0x2c, 0xc5, 0xa5, 0x34, 0x32, 0xbd, 0x46,
	0x3f, 0x85, 0x66, 0xc4, 0xa7, 0x7e, 0xb0, 0x50, 0x19, 0xd9, 0xa3, 0x9e, 0xf1, 0x62, 0xca, 0xca,
	0x70, 0xd5, 0x79, 0x04, 0x5b, 0x99, 0x0d, 0x74, 0x9a, 0xce, 0x2e, 0xec, 0x9c, 0xc7, 0xa9, 0x39,
	0xe4, 0x1e, 0x6e, 0xed, 0xbc, 0x86, 0x41, 0x71, 0x01, 0xbf, 0xcc, 0x81, 0xce, 0x75, 0xc6, 0xae,
	0x72, 0xb3, 0x58, 0xce, 0xe6, 0x10, 0xe8, 0x1d, 0xcf, 0x66, 0xc1, 0xe4, 0x3c, 0x8d, 0xf7, 0x14,
	0xfa, 0xa9, 0x05, 0x03, 0xf5, 0xa0, 0xe6, 0x6b, 0xf9, 0x26, 0x13, 0x23, 0xe7, 0x00, 0xfa, 0x3f,
	0xf0, 0x44, 0x7f, 0x1b, 0x16, 0xe0, 0xff, 0x60, 0xa9, 0x8f, 0x1c, 0xa7, 0x8e, 0x2d, 0x35, 0x3f,
	0xf7, 0x9c, 0x6f, 0x81, 0xac, 0xbc, 0x31, 0xe2, 0x43, 0xea, 0xe5, 0xbc, 0x84, 0xfe, 0xbb, 0x65,
	0x7e, 0x9b, 0x07, 0xe9, 0x28, 0x90, 0x95, 0x0e, 0xcb, 0xf7, 0xb5, 0x4a, 0x02, 0x0b, 0x8d, 0xc1,
	0x1e, 0x03, 0xe8, 0x8a, 0x8f, 0x6f, 0xf9, 0x9d, 0x8a, 0xd8, 0x61, 0x6d, 0x6d, 0x79, 0xcb, 0xef,
	0x9c, 0x57, 0xb0, 0x95, 0x91, 0x60, 0xe2, 0xab, 0x33, 0xdc, 0xb8, 0xf7, 0x0c, 0x5f, 0xc3, 0xae,
	0x10, 0x9f, 0xcc, 0x96, 0x71, 0xc2, 0xa3, 0x93, 0x60, 0xf1, 0xde, 0x9f, 0x9a, 0x6d, 0x9f, 0x42,
	0x67, 0xa2, 0xed, 0xe3, 0x85, 0x3b, 0xe7, 0x78, 0x2d, 0x6c, 0xb4, 0xfd, 0x2c, 0x4c, 0xce, 0x19,
	0x0c, 0xcb, 0x6a, 0xcc, 0xe0, 0x33, 0x68, 0xa1, 0x2b, 0xa6, 0xd0, 0x37, 0x29, 0xa0, 0x3f, 0x33,
	0xeb, 0xce, 0x29, 0xec, 0x8a, 0x42, 0x54, 0x26, 0xf1, 0x01, 0x51, 0xf6, 0x60, 0x58, 0x8e, 0x82,
	0x65, 0x7d, 0x0f, 0xdb, 0xc7, 0xf1, 0xed, 0xc9, 0x8d, 0xbb, 0x98, 0xf2, 0x77, 0x5c, 0xa8, 0x30,
	0xfc, 0x03, 0xcb, 0x24, 0xfc, 0xfa, 0x33, 0xd5, 0x24, 0xc6, 0x29, 0x3d, 0x35, 0x45, 0x4f, 0x57,
	0x9b, 0x2f, 0x91, 0x21, 0x41, 0x7f, 0x61, 0x1f, 0x4c, 0xe0, 0x0f, 0x41, 0x6b, 0x7c, 0x7b, 0x19,
	0xce, 0xfc, 0xe4, 0x43, 0xf7, 0xde, 0x87, 0x76, 0x2c, 0x75, 0xea, 0xf4, 0x6b, 0xea, 0xf4, 0x2d,
	0x65, 0x10, 0x87, 0x5f, 0x95, 0x58, 0xbd, 0x2a, 0x31, 0xc1, 0xda, 0x6a, 0x7f, 0xcc, 0xe9, 0x7b,
	0xe8, 0x62, 0x2e, 0x3f, 0xa6, 0x0d, 0x70, 0xb9, 0xc4, 0x8b, 0xd1, 0x61, 0x6a, 0x5c, 0xa2, 0xa0,
	0x56, 0xa6, 0xe0, 0x57, 0xe8, 0x99, 0x98, 0xff, 0x29, 0x90, 0x70, 0x69, 0xf0, 0x28, 0x0a, 0x22,
	0xf5, 0x09, 0xf6, 0xc8, 0xd6, 0xdd, 0xed, 0x4c, 0x9a, 0x98, 0x5e, 0x71, 0xfe, 0x6a, 0x40, 0xcb,
	0x14, 0xf0, 0x0b, 0x68, 0xde, 0xa8, 0xfd, 0xb0, 0x80, 0x8f, 0xb4, 0x7f, 0xee, 0x9b, 0x18, 0xba,
	0xd0, 0x03, 0xb0, 0x26, 0x73, 0x6f, 0x9c, 0xdc, 0x85, 0x7a, 0xeb, 0xde, 0x68, 0x4b, 0xbb, 0x9f,
	0x04, 0xf3, 0xb9, 0xbb, 0xf0, 0xae, 0xc4, 0x82, 0x60, 0x69, 0xae, 0x06, 0xa2, 0x25, 0xd5, 0x93,
	0x38, 0xc0, 0x3c, 0x08, 0x76, 0xd9, 0xb4, 0xeb, 0x33, 0xb9, 0x48, 0xbf, 0x81, 0x76, 0xda, 0xa2,
	0x86, 0x9b, 0xca, 0x73, 0xa0, 0x3d, 0x8b, 0x6d, 0x97, 0xad, 0x1c, 0xe9, 0x29, 0xf4, 0xfd, 0x78,
	0x9c, 0xeb, 0x77, 0x0d, 0xa5, 0xdd, 0xd7, 0xda, 0xca, 0xe6, 0xc9, 0x7a, 0x7e, 0xce, 0x2c, 0x7e,
	0x06, 0x96, 0x2b, 0x9b, 0x9f, 0x3c, 0xef, 0xa6, 0x92, 0x6f, 0x6b, 0x79, 0xbe, 0x49, 0xb2, 0x96,
	0xab, 0xe7, 0xf4, 0x0d, 0xf4, 0xdd, 0xf8, 0x76, 0x3c, 0x51, 0x64, 0x8e, 0x43, 0x81, 0xe6, 0xb0,
	0xa5, 0x74, 0x7b, 0xa8, 0xab, 0xb8, 0x1d, 0xac, 0xeb, 0x66, 0xad, 0x74, 0x04, 0x6d, 0x19, 0x43,
	0xb1, 0x37, 0xb4, 0x94, 0x7a, 0x27, 0x55, 0x67, 0xd1, 0x66, 0x96, 0x8b, 0x06, 0xa9, 0x99, 0xf2,
	0x44, 0xc3, 0x39, 0x6c, 0x67, 0x35, 0x85, 0xce, 0xcc, 0xac, 0x29, 0x1a, 0xe8, 0x0b, 0x00, 0xa9,
	0xc1, 0xcb, 0x01, 0xd9, 0xca, 0x16, 0x7b, 0x23, 0x93, 0xd1, 0xb5, 0x85, 0xbe, 0x05, 0x2a, 0x65,
	0x06, 0xb2, 0x89, 0xea, 0x00, 0x43, 0x5b, 0xc9, 0x1f, 0xa7, 0xf2, 0xaa, 0x2e, 0xc3, 0xc8, 0xb4,
	0xb0, 0x20, 0xf3, 0x0e, 0x97, 0x26, 0xef, 0x4e, 0x36, 0xef, 0x42, 0xab, 0x17, 0xbf, 0x72, 0x34,
	0xc8, 0x04, 0xa4, 0xa6, 0x90, 0x40, 0x37, 0x9b, 0xc0, 0x9a, 0x36, 0xc7, 0x48, 0x58, 0x58, 0x70,
	0xfe, 0x6e, 0x80, 0x95, 0xf6, 0xd2, 0x83, 0x02, 0xe9, 0xdb, 0x86, 0xf4, 0xec, 0xad, 0xfb, 0x97,
	0xa8, 0x3f, 0xcb, 0xa2, 0xbe, 0x95, 0x41, 0x5d, 0xc7, 0xd6, 0xac, 0xbf, 0x28, 0xb3, 0xbe, 0x5b,
	0x62, 0x1d, 0x05, 0x19, 0xd8, 0xcf, 0xd6, 0xc1, 0xfe, 0x51, 0x35, 0xec, 0x18, 0xa1, 0x48, 0xfb,
	0x57, 0x25, 0xda, 0x77, 0x0a, 0xb4, 0xa3, 0x30, 0xc5, 0xfd, 0x64, 0x1d, 0xee, 0xfb, 0x95, 0xb8,
	0xa3, 0xbc, 0xc0, 0xfb, 0x51, 0x99, 0xf7, 0x41, 0x91, 0x77, 0x54, 0xae, 0x80, 0x3f, 0x2a, 0x03,
	0x3f, 0x28, 0x02, 0x6f, 0x44, 0x29, 0xf1, 0x2f, 0x2b, 0x88, 0xdf, 0x2d, 0x11, 0x6f, 0xea, 0xbb,
	0x42, 0xfe, 0xe2, 0x1e, 0xe4, 0x9f, 0xac, 0x43, 0x1e, 0xc3, 0x94, 0x99, 0x3f, 0x2a, 0x33, 0x3f,
	0x28, 0x32, 0x6f, 0x52, 0x4f, 0xa1, 0xbf, 0xb8, 0x07, 0xfa, 0x27, 0xeb, 0xa0, 0x37, 0x29, 0x94,
	0xa8, 0xcf, 0x3e, 0x29, 0xc5, 0xc9, 0xab, 0xd6, 0xef, 0xfc, 0x02, 0x0d, 0x35, 0x90, 0x6f, 0x65,
	0xf1, 0xba, 0x8d, 0xdd, 0xa9, 0x79, 0x8c, 0x98, 0x29, 0x7d, 0x55, 0x78, 0x42, 0xd6, 0x2a, 0x11,
	0x35, 0x11, 0xf3, 0x6f, 0xcb, 0xcf, 0xff, 0xdc, 0x00, 0x3b, 0x73, 0x35, 0xa8, 0x0d, 0xad, 0xf3,
	0xc5, 0xef, 0xee, 0xcc, 0xf7, 0xc8, 0xff, 0x68, 0x0b, 0xea, 0xe2, 0x36, 0x90, 0x0d, 0xda, 0x85,
	0x76, 0x1a, 0x88, 0xd4, 0xc4, 0x2f, 0xae, 0x97, 0xa7, 0x97, 0xd4, 0xa5, 0x10, 0x89, 0x24, 0x9b,
	0xb4, 0x03, 0x96, 0x39, 0x71, 0xd2, 0x90, 0x33, 0x53, 0x44, 0xd2, 0x14, 0x0f, 0x55, 0x38, 0xe5,
	0x33, 0x9e, 0xf0, 0x9f, 0xc4, 0x1f, 0x9f, 0xb4, 0xe8, 0x16, 0x74, 0x73, 0x44, 0x12, 0x4b, 0x0a,
	0x0c, 0x65, 0xa4, 0x2d, 0x37, 0x4f, 0x41, 0x20, 0x40, 0xb7, 0xd5, 0x2b, 0x31, 0x57, 0x3a, 0x62,
	0x4b, 0x6b, 0xb1, 0xd4, 0xa4, 0xf3, 0x4f, 0x00, 0x00, 0x00, 0xff, 0xff, 0x74, 0xf0, 0x49, 0x5f,
	0x00, 0x0d, 0x00, 0x00,
}
