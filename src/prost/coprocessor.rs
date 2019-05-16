/// [start, end)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyRange {
    #[prost(bytes, tag = "1")]
    pub start: std::vec::Vec<u8>,
    #[prost(bytes, tag = "2")]
    pub end: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Request {
    #[prost(message, optional, tag = "1")]
    pub context: ::std::option::Option<super::kvrpcpb::Context>,
    #[prost(int64, tag = "2")]
    pub tp: i64,
    #[prost(bytes, tag = "3")]
    pub data: std::vec::Vec<u8>,
    #[prost(message, repeated, tag = "4")]
    pub ranges: ::std::vec::Vec<KeyRange>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    #[prost(bytes, tag = "1")]
    pub data: std::vec::Vec<u8>,
    #[prost(message, optional, tag = "2")]
    pub region_error: ::std::option::Option<super::errorpb::Error>,
    #[prost(message, optional, tag = "3")]
    pub locked: ::std::option::Option<super::kvrpcpb::LockInfo>,
    #[prost(string, tag = "4")]
    pub other_error: std::string::String,
    #[prost(message, optional, tag = "5")]
    pub range: ::std::option::Option<KeyRange>,
    #[prost(message, optional, tag = "6")]
    pub exec_details: ::std::option::Option<super::kvrpcpb::ExecDetails>,
}
