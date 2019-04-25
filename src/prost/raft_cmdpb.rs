#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRequest {
    #[prost(string, tag = "1")]
    pub cf: std::string::String,
    #[prost(bytes, tag = "2")]
    pub key: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetResponse {
    #[prost(bytes, tag = "1")]
    pub value: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutRequest {
    #[prost(string, tag = "1")]
    pub cf: std::string::String,
    #[prost(bytes, tag = "2")]
    pub key: std::vec::Vec<u8>,
    #[prost(bytes, tag = "3")]
    pub value: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRequest {
    #[prost(string, tag = "1")]
    pub cf: std::string::String,
    #[prost(bytes, tag = "2")]
    pub key: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRangeRequest {
    #[prost(string, tag = "1")]
    pub cf: std::string::String,
    #[prost(bytes, tag = "2")]
    pub start_key: std::vec::Vec<u8>,
    #[prost(bytes, tag = "3")]
    pub end_key: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRangeResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnapRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnapResponse {
    #[prost(message, optional, tag = "1")]
    pub region: ::std::option::Option<super::metapb::Region>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrewriteRequest {
    #[prost(bytes, tag = "1")]
    pub key: std::vec::Vec<u8>,
    #[prost(bytes, tag = "2")]
    pub value: std::vec::Vec<u8>,
    #[prost(bytes, tag = "3")]
    pub lock: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrewriteResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IngestSstRequest {
    #[prost(message, optional, tag = "1")]
    pub sst: ::std::option::Option<super::import_sstpb::SstMeta>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IngestSstResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadIndexRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadIndexResponse {
    #[prost(uint64, tag = "1")]
    pub read_index: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Request {
    #[prost(enumeration = "CmdType", tag = "1")]
    pub cmd_type: i32,
    #[prost(message, optional, tag = "2")]
    pub get: ::std::option::Option<GetRequest>,
    #[prost(message, optional, tag = "4")]
    pub put: ::std::option::Option<PutRequest>,
    #[prost(message, optional, tag = "5")]
    pub delete: ::std::option::Option<DeleteRequest>,
    #[prost(message, optional, tag = "6")]
    pub snap: ::std::option::Option<SnapRequest>,
    #[prost(message, optional, tag = "7")]
    pub prewrite: ::std::option::Option<PrewriteRequest>,
    #[prost(message, optional, tag = "8")]
    pub delete_range: ::std::option::Option<DeleteRangeRequest>,
    #[prost(message, optional, tag = "9")]
    pub ingest_sst: ::std::option::Option<IngestSstRequest>,
    #[prost(message, optional, tag = "10")]
    pub read_index: ::std::option::Option<ReadIndexRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    #[prost(enumeration = "CmdType", tag = "1")]
    pub cmd_type: i32,
    #[prost(message, optional, tag = "2")]
    pub get: ::std::option::Option<GetResponse>,
    #[prost(message, optional, tag = "4")]
    pub put: ::std::option::Option<PutResponse>,
    #[prost(message, optional, tag = "5")]
    pub delete: ::std::option::Option<DeleteResponse>,
    #[prost(message, optional, tag = "6")]
    pub snap: ::std::option::Option<SnapResponse>,
    #[prost(message, optional, tag = "7")]
    pub prewrite: ::std::option::Option<PrewriteResponse>,
    #[prost(message, optional, tag = "8")]
    pub delte_range: ::std::option::Option<DeleteRangeResponse>,
    #[prost(message, optional, tag = "9")]
    pub ingest_sst: ::std::option::Option<IngestSstResponse>,
    #[prost(message, optional, tag = "10")]
    pub read_index: ::std::option::Option<ReadIndexResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangePeerRequest {
    /// This can be only called in internal RaftStore now.
    #[prost(enumeration = "super::eraftpb::ConfChangeType", tag = "1")]
    pub change_type: i32,
    #[prost(message, optional, tag = "2")]
    pub peer: ::std::option::Option<super::metapb::Peer>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangePeerResponse {
    #[prost(message, optional, tag = "1")]
    pub region: ::std::option::Option<super::metapb::Region>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SplitRequest {
    /// This can be only called in internal RaftStore now.
    /// The split_key must be in the been splitting region.
    #[prost(bytes, tag = "1")]
    pub split_key: std::vec::Vec<u8>,
    /// We split the region into two, first uses the origin
    /// parent region id, and the second uses the new_region_id.
    /// We must guarantee that the new_region_id is global unique.
    #[prost(uint64, tag = "2")]
    pub new_region_id: u64,
    /// The peer ids for the new split region.
    #[prost(uint64, repeated, tag = "3")]
    pub new_peer_ids: ::std::vec::Vec<u64>,
    /// If true, right region derive the origin region_id,
    /// left region use new_region_id.
    /// Will be ignored in batch split, use `BatchSplitRequest::right_derive` instead.
    #[prost(bool, tag = "4")]
    pub right_derive: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SplitResponse {
    #[prost(message, optional, tag = "1")]
    pub left: ::std::option::Option<super::metapb::Region>,
    #[prost(message, optional, tag = "2")]
    pub right: ::std::option::Option<super::metapb::Region>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchSplitRequest {
    #[prost(message, repeated, tag = "1")]
    pub requests: ::std::vec::Vec<SplitRequest>,
    /// If true, the last region derive the origin region_id,
    /// other regions use new ids.
    #[prost(bool, tag = "2")]
    pub right_derive: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchSplitResponse {
    #[prost(message, repeated, tag = "1")]
    pub regions: ::std::vec::Vec<super::metapb::Region>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompactLogRequest {
    #[prost(uint64, tag = "1")]
    pub compact_index: u64,
    #[prost(uint64, tag = "2")]
    pub compact_term: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompactLogResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferLeaderRequest {
    #[prost(message, optional, tag = "1")]
    pub peer: ::std::option::Option<super::metapb::Peer>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferLeaderResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyHashRequest {
    #[prost(uint64, tag = "1")]
    pub index: u64,
    #[prost(bytes, tag = "2")]
    pub hash: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyHashResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrepareMergeRequest {
    #[prost(uint64, tag = "1")]
    pub min_index: u64,
    #[prost(message, optional, tag = "2")]
    pub target: ::std::option::Option<super::metapb::Region>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrepareMergeResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitMergeRequest {
    #[prost(message, optional, tag = "1")]
    pub source: ::std::option::Option<super::metapb::Region>,
    #[prost(uint64, tag = "2")]
    pub commit: u64,
    #[prost(message, repeated, tag = "3")]
    pub entries: ::std::vec::Vec<super::eraftpb::Entry>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitMergeResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RollbackMergeRequest {
    #[prost(uint64, tag = "1")]
    pub commit: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RollbackMergeResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdminRequest {
    #[prost(enumeration = "AdminCmdType", tag = "1")]
    pub cmd_type: i32,
    #[prost(message, optional, tag = "2")]
    pub change_peer: ::std::option::Option<ChangePeerRequest>,
    #[prost(message, optional, tag = "3")]
    pub split: ::std::option::Option<SplitRequest>,
    #[prost(message, optional, tag = "4")]
    pub compact_log: ::std::option::Option<CompactLogRequest>,
    #[prost(message, optional, tag = "5")]
    pub transfer_leader: ::std::option::Option<TransferLeaderRequest>,
    #[prost(message, optional, tag = "6")]
    pub verify_hash: ::std::option::Option<VerifyHashRequest>,
    #[prost(message, optional, tag = "7")]
    pub prepare_merge: ::std::option::Option<PrepareMergeRequest>,
    #[prost(message, optional, tag = "8")]
    pub commit_merge: ::std::option::Option<CommitMergeRequest>,
    #[prost(message, optional, tag = "9")]
    pub rollback_merge: ::std::option::Option<RollbackMergeRequest>,
    #[prost(message, optional, tag = "10")]
    pub splits: ::std::option::Option<BatchSplitRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdminResponse {
    #[prost(enumeration = "AdminCmdType", tag = "1")]
    pub cmd_type: i32,
    #[prost(message, optional, tag = "2")]
    pub change_peer: ::std::option::Option<ChangePeerResponse>,
    #[prost(message, optional, tag = "3")]
    pub split: ::std::option::Option<SplitResponse>,
    #[prost(message, optional, tag = "4")]
    pub compact_log: ::std::option::Option<CompactLogResponse>,
    #[prost(message, optional, tag = "5")]
    pub transfer_leader: ::std::option::Option<TransferLeaderResponse>,
    #[prost(message, optional, tag = "6")]
    pub verify_hash: ::std::option::Option<VerifyHashResponse>,
    #[prost(message, optional, tag = "7")]
    pub prepare_merge: ::std::option::Option<PrepareMergeResponse>,
    #[prost(message, optional, tag = "8")]
    pub commit_merge: ::std::option::Option<CommitMergeResponse>,
    #[prost(message, optional, tag = "9")]
    pub rollback_merge: ::std::option::Option<RollbackMergeResponse>,
    #[prost(message, optional, tag = "10")]
    pub splits: ::std::option::Option<BatchSplitResponse>,
}
/// For get the leader of the region.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegionLeaderRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegionLeaderResponse {
    #[prost(message, optional, tag = "1")]
    pub leader: ::std::option::Option<super::metapb::Peer>,
}
/// For getting more information of the region.
/// We add some admin operations (ChangePeer, Split...) into the pb job list,
/// then pd server will peek the first one, handle it and then pop it from the job lib. 
/// But sometimes, the pd server may crash before popping. When another pd server
/// starts and finds the job is running but not finished, it will first check whether
/// the raft server already has handled this job.
/// E,g, for ChangePeer, if we add Peer10 into region1 and find region1 has already had
/// Peer10, we can think this ChangePeer is finished, and can pop this job from job list
/// directly.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegionDetailRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegionDetailResponse {
    #[prost(message, optional, tag = "1")]
    pub region: ::std::option::Option<super::metapb::Region>,
    #[prost(message, optional, tag = "2")]
    pub leader: ::std::option::Option<super::metapb::Peer>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusRequest {
    #[prost(enumeration = "StatusCmdType", tag = "1")]
    pub cmd_type: i32,
    #[prost(message, optional, tag = "2")]
    pub region_leader: ::std::option::Option<RegionLeaderRequest>,
    #[prost(message, optional, tag = "3")]
    pub region_detail: ::std::option::Option<RegionDetailRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusResponse {
    #[prost(enumeration = "StatusCmdType", tag = "1")]
    pub cmd_type: i32,
    #[prost(message, optional, tag = "2")]
    pub region_leader: ::std::option::Option<RegionLeaderResponse>,
    #[prost(message, optional, tag = "3")]
    pub region_detail: ::std::option::Option<RegionDetailResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RaftRequestHeader {
    #[prost(uint64, tag = "1")]
    pub region_id: u64,
    #[prost(message, optional, tag = "2")]
    pub peer: ::std::option::Option<super::metapb::Peer>,
    /// true for read linearization
    #[prost(bool, tag = "3")]
    pub read_quorum: bool,
    /// 16 bytes, to distinguish request.  
    #[prost(bytes, tag = "4")]
    pub uuid: std::vec::Vec<u8>,
    #[prost(message, optional, tag = "5")]
    pub region_epoch: ::std::option::Option<super::metapb::RegionEpoch>,
    #[prost(uint64, tag = "6")]
    pub term: u64,
    #[prost(bool, tag = "7")]
    pub sync_log: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RaftResponseHeader {
    #[prost(message, optional, tag = "1")]
    pub error: ::std::option::Option<super::errorpb::Error>,
    #[prost(bytes, tag = "2")]
    pub uuid: std::vec::Vec<u8>,
    #[prost(uint64, tag = "3")]
    pub current_term: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RaftCmdRequest {
    #[prost(message, optional, tag = "1")]
    pub header: ::std::option::Option<RaftRequestHeader>,
    /// We can't enclose normal requests and administrator request
    /// at same time.
    #[prost(message, repeated, tag = "2")]
    pub requests: ::std::vec::Vec<Request>,
    #[prost(message, optional, tag = "3")]
    pub admin_request: ::std::option::Option<AdminRequest>,
    #[prost(message, optional, tag = "4")]
    pub status_request: ::std::option::Option<StatusRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RaftCmdResponse {
    #[prost(message, optional, tag = "1")]
    pub header: ::std::option::Option<RaftResponseHeader>,
    #[prost(message, repeated, tag = "2")]
    pub responses: ::std::vec::Vec<Response>,
    #[prost(message, optional, tag = "3")]
    pub admin_response: ::std::option::Option<AdminResponse>,
    #[prost(message, optional, tag = "4")]
    pub status_response: ::std::option::Option<StatusResponse>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CmdType {
    Invalid = 0,
    Get = 1,
    Put = 3,
    Delete = 4,
    Snap = 5,
    Prewrite = 6,
    DeleteRange = 7,
    IngestSst = 8,
    ReadIndex = 9,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AdminCmdType {
    InvalidAdmin = 0,
    ChangePeer = 1,
    /// Use `BatchSplit` instead.
    Split = 2,
    CompactLog = 3,
    TransferLeader = 4,
    ComputeHash = 5,
    VerifyHash = 6,
    PrepareMerge = 7,
    CommitMerge = 8,
    RollbackMerge = 9,
    BatchSplit = 10,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StatusCmdType {
    InvalidStatus = 0,
    RegionLeader = 1,
    RegionDetail = 2,
}
