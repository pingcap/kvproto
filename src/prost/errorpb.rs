#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotLeader {
    #[prost(uint64, tag = "1")]
    pub region_id: u64,
    #[prost(message, optional, tag = "2")]
    pub leader: ::std::option::Option<super::metapb::Peer>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoreNotMatch {
    #[prost(uint64, tag = "1")]
    pub request_store_id: u64,
    #[prost(uint64, tag = "2")]
    pub actual_store_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegionNotFound {
    #[prost(uint64, tag = "1")]
    pub region_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyNotInRegion {
    #[prost(bytes, tag = "1")]
    pub key: std::vec::Vec<u8>,
    #[prost(uint64, tag = "2")]
    pub region_id: u64,
    #[prost(bytes, tag = "3")]
    pub start_key: std::vec::Vec<u8>,
    #[prost(bytes, tag = "4")]
    pub end_key: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EpochNotMatch {
    #[prost(message, repeated, tag = "1")]
    pub current_regions: ::std::vec::Vec<super::metapb::Region>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerIsBusy {
    #[prost(string, tag = "1")]
    pub reason: std::string::String,
    #[prost(uint64, tag = "2")]
    pub backoff_ms: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StaleCommand {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RaftEntryTooLarge {
    #[prost(uint64, tag = "1")]
    pub region_id: u64,
    #[prost(uint64, tag = "2")]
    pub entry_size: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Error {
    #[prost(string, tag = "1")]
    pub message: std::string::String,
    #[prost(message, optional, tag = "2")]
    pub not_leader: ::std::option::Option<NotLeader>,
    #[prost(message, optional, tag = "3")]
    pub region_not_found: ::std::option::Option<RegionNotFound>,
    #[prost(message, optional, tag = "4")]
    pub key_not_in_region: ::std::option::Option<KeyNotInRegion>,
    #[prost(message, optional, tag = "5")]
    pub epoch_not_match: ::std::option::Option<EpochNotMatch>,
    #[prost(message, optional, tag = "6")]
    pub server_is_busy: ::std::option::Option<ServerIsBusy>,
    #[prost(message, optional, tag = "7")]
    pub stale_command: ::std::option::Option<StaleCommand>,
    #[prost(message, optional, tag = "8")]
    pub store_not_match: ::std::option::Option<StoreNotMatch>,
    #[prost(message, optional, tag = "9")]
    pub raft_entry_too_large: ::std::option::Option<RaftEntryTooLarge>,
}
