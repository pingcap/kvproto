#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RaftMessage {
    #[prost(uint64, tag = "1")]
    pub region_id: u64,
    #[prost(message, optional, tag = "2")]
    pub from_peer: ::std::option::Option<super::metapb::Peer>,
    #[prost(message, optional, tag = "3")]
    pub to_peer: ::std::option::Option<super::metapb::Peer>,
    #[prost(message, optional, tag = "4")]
    pub message: ::std::option::Option<super::eraftpb::Message>,
    #[prost(message, optional, tag = "5")]
    pub region_epoch: ::std::option::Option<super::metapb::RegionEpoch>,
    /// true means to_peer is a tombstone peer and it should remove itself.
    #[prost(bool, tag = "6")]
    pub is_tombstone: bool,
    /// Region key range [start_key, end_key).
    #[prost(bytes, tag = "7")]
    pub start_key: std::vec::Vec<u8>,
    #[prost(bytes, tag = "8")]
    pub end_key: std::vec::Vec<u8>,
    /// If it has value, to_peer should be removed if merge is never going to complete.
    #[prost(message, optional, tag = "9")]
    pub merge_target: ::std::option::Option<super::metapb::Region>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RaftTruncatedState {
    #[prost(uint64, tag = "1")]
    pub index: u64,
    #[prost(uint64, tag = "2")]
    pub term: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnapshotCfFile {
    #[prost(string, tag = "1")]
    pub cf: std::string::String,
    #[prost(uint64, tag = "2")]
    pub size: u64,
    #[prost(uint32, tag = "3")]
    pub checksum: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnapshotMeta {
    #[prost(message, repeated, tag = "1")]
    pub cf_files: ::std::vec::Vec<SnapshotCfFile>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnapshotChunk {
    #[prost(message, optional, tag = "1")]
    pub message: ::std::option::Option<RaftMessage>,
    #[prost(bytes, tag = "2")]
    pub data: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Done {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyValue {
    #[prost(bytes, tag = "1")]
    pub key: std::vec::Vec<u8>,
    #[prost(bytes, tag = "2")]
    pub value: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RaftSnapshotData {
    #[prost(message, optional, tag = "1")]
    pub region: ::std::option::Option<super::metapb::Region>,
    #[prost(uint64, tag = "2")]
    pub file_size: u64,
    #[prost(message, repeated, tag = "3")]
    pub data: ::std::vec::Vec<KeyValue>,
    #[prost(uint64, tag = "4")]
    pub version: u64,
    #[prost(message, optional, tag = "5")]
    pub meta: ::std::option::Option<SnapshotMeta>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoreIdent {
    #[prost(uint64, tag = "1")]
    pub cluster_id: u64,
    #[prost(uint64, tag = "2")]
    pub store_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RaftLocalState {
    #[prost(message, optional, tag = "1")]
    pub hard_state: ::std::option::Option<super::eraftpb::HardState>,
    #[prost(uint64, tag = "2")]
    pub last_index: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RaftApplyState {
    #[prost(uint64, tag = "1")]
    pub applied_index: u64,
    #[prost(message, optional, tag = "2")]
    pub truncated_state: ::std::option::Option<RaftTruncatedState>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MergeState {
    #[prost(uint64, tag = "1")]
    pub min_index: u64,
    #[prost(message, optional, tag = "2")]
    pub target: ::std::option::Option<super::metapb::Region>,
    #[prost(uint64, tag = "3")]
    pub commit: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegionLocalState {
    #[prost(enumeration = "PeerState", tag = "1")]
    pub state: i32,
    #[prost(message, optional, tag = "2")]
    pub region: ::std::option::Option<super::metapb::Region>,
    #[prost(message, optional, tag = "3")]
    pub merge_state: ::std::option::Option<MergeState>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PeerState {
    Normal = 0,
    Applying = 1,
    Tombstone = 2,
    Merging = 3,
}
