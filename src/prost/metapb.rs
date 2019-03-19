#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cluster {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// max peer count for a region.
    /// pd will do the auto-balance if region peer count mismatches.
    ///
    /// more attributes......
    #[prost(uint32, tag = "2")]
    pub max_peer_count: u32,
}
/// Case insensitive key/value for replica constraints.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoreLabel {
    #[prost(string, tag = "1")]
    pub key: std::string::String,
    #[prost(string, tag = "2")]
    pub value: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Store {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(string, tag = "2")]
    pub address: std::string::String,
    #[prost(enumeration = "StoreState", tag = "3")]
    pub state: i32,
    #[prost(message, repeated, tag = "4")]
    pub labels: ::std::vec::Vec<StoreLabel>,
    /// more attributes......
    #[prost(string, tag = "5")]
    pub version: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegionEpoch {
    /// Conf change version, auto increment when add or remove peer
    #[prost(uint64, tag = "1")]
    pub conf_ver: u64,
    /// Region version, auto increment when split or merge
    #[prost(uint64, tag = "2")]
    pub version: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Region {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// Region key range [start_key, end_key).
    #[prost(bytes, tag = "2")]
    pub start_key: std::vec::Vec<u8>,
    #[prost(bytes, tag = "3")]
    pub end_key: std::vec::Vec<u8>,
    #[prost(message, optional, tag = "4")]
    pub region_epoch: ::std::option::Option<RegionEpoch>,
    #[prost(message, repeated, tag = "5")]
    pub peers: ::std::vec::Vec<Peer>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Peer {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(uint64, tag = "2")]
    pub store_id: u64,
    #[prost(bool, tag = "3")]
    pub is_learner: bool,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StoreState {
    Up = 0,
    Offline = 1,
    Tombstone = 2,
}
