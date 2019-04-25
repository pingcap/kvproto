#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRequest {
    #[prost(enumeration = "Db", tag = "1")]
    pub db: i32,
    #[prost(string, tag = "2")]
    pub cf: std::string::String,
    #[prost(bytes, tag = "3")]
    pub key: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetResponse {
    #[prost(bytes, tag = "1")]
    pub value: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RaftLogRequest {
    #[prost(uint64, tag = "1")]
    pub region_id: u64,
    #[prost(uint64, tag = "2")]
    pub log_index: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RaftLogResponse {
    #[prost(message, optional, tag = "1")]
    pub entry: ::std::option::Option<super::eraftpb::Entry>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegionInfoRequest {
    #[prost(uint64, tag = "1")]
    pub region_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegionInfoResponse {
    #[prost(message, optional, tag = "1")]
    pub raft_local_state: ::std::option::Option<super::raft_serverpb::RaftLocalState>,
    #[prost(message, optional, tag = "2")]
    pub raft_apply_state: ::std::option::Option<super::raft_serverpb::RaftApplyState>,
    #[prost(message, optional, tag = "3")]
    pub region_local_state: ::std::option::Option<super::raft_serverpb::RegionLocalState>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegionSizeRequest {
    #[prost(uint64, tag = "1")]
    pub region_id: u64,
    #[prost(string, repeated, tag = "2")]
    pub cfs: ::std::vec::Vec<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegionSizeResponse {
    #[prost(message, repeated, tag = "1")]
    pub entries: ::std::vec::Vec<region_size_response::Entry>,
}
pub mod region_size_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Entry {
        #[prost(string, tag = "1")]
        pub cf: std::string::String,
        #[prost(uint64, tag = "2")]
        pub size: u64,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScanMvccRequest {
    #[prost(bytes, tag = "1")]
    pub from_key: std::vec::Vec<u8>,
    #[prost(bytes, tag = "2")]
    pub to_key: std::vec::Vec<u8>,
    #[prost(uint64, tag = "3")]
    pub limit: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScanMvccResponse {
    #[prost(bytes, tag = "1")]
    pub key: std::vec::Vec<u8>,
    #[prost(message, optional, tag = "2")]
    pub info: ::std::option::Option<super::kvrpcpb::MvccInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompactRequest {
    #[prost(enumeration = "Db", tag = "1")]
    pub db: i32,
    #[prost(string, tag = "2")]
    pub cf: std::string::String,
    #[prost(bytes, tag = "3")]
    pub from_key: std::vec::Vec<u8>,
    #[prost(bytes, tag = "4")]
    pub to_key: std::vec::Vec<u8>,
    #[prost(uint32, tag = "5")]
    pub threads: u32,
    #[prost(enumeration = "BottommostLevelCompaction", tag = "6")]
    pub bottommost_level_compaction: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompactResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InjectFailPointRequest {
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    #[prost(string, tag = "2")]
    pub actions: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InjectFailPointResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecoverFailPointRequest {
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecoverFailPointResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFailPointsRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFailPointsResponse {
    #[prost(message, repeated, tag = "1")]
    pub entries: ::std::vec::Vec<list_fail_points_response::Entry>,
}
pub mod list_fail_points_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Entry {
        #[prost(string, tag = "1")]
        pub name: std::string::String,
        #[prost(string, tag = "2")]
        pub actions: std::string::String,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMetricsRequest {
    #[prost(bool, tag = "1")]
    pub all: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMetricsResponse {
    #[prost(string, tag = "1")]
    pub prometheus: std::string::String,
    #[prost(string, tag = "2")]
    pub rocksdb_kv: std::string::String,
    #[prost(string, tag = "3")]
    pub rocksdb_raft: std::string::String,
    #[prost(string, tag = "4")]
    pub jemalloc: std::string::String,
    #[prost(uint64, tag = "5")]
    pub store_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegionConsistencyCheckRequest {
    #[prost(uint64, tag = "1")]
    pub region_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegionConsistencyCheckResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModifyTikvConfigRequest {
    #[prost(enumeration = "Module", tag = "1")]
    pub module: i32,
    #[prost(string, tag = "2")]
    pub config_name: std::string::String,
    #[prost(string, tag = "3")]
    pub config_value: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModifyTikvConfigResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Property {
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    #[prost(string, tag = "2")]
    pub value: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRegionPropertiesRequest {
    #[prost(uint64, tag = "1")]
    pub region_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRegionPropertiesResponse {
    #[prost(message, repeated, tag = "1")]
    pub props: ::std::vec::Vec<Property>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Db {
    Invalid = 0,
    Kv = 1,
    Raft = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Module {
    Unused = 0,
    Kvdb = 1,
    Raftdb = 2,
    Readpool = 3,
    Server = 4,
    Storage = 5,
    Pd = 6,
    Metric = 7,
    Coprocessor = 8,
    Security = 9,
    Import = 10,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BottommostLevelCompaction {
    /// Skip bottommost level compaction
    Skip = 0,
    /// Force bottommost level compaction
    Force = 1,
    /// Compact bottommost level if there is a compaction filter.
    IfHaveCompactionFilter = 2,
}
const METHOD_DEBUG_GET: ::grpcio::Method<GetRequest, GetResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/debugpb.Debug/Get",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
};
const METHOD_DEBUG_RAFT_LOG: ::grpcio::Method<RaftLogRequest, RaftLogResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/debugpb.Debug/RaftLog",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
};
const METHOD_DEBUG_REGION_INFO: ::grpcio::Method<RegionInfoRequest, RegionInfoResponse> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::Unary,
        name: "/debugpb.Debug/RegionInfo",
        req_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pr_ser,
            de: ::grpcio::pr_de,
        },
        resp_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pr_ser,
            de: ::grpcio::pr_de,
        },
    };
const METHOD_DEBUG_REGION_SIZE: ::grpcio::Method<RegionSizeRequest, RegionSizeResponse> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::Unary,
        name: "/debugpb.Debug/RegionSize",
        req_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pr_ser,
            de: ::grpcio::pr_de,
        },
        resp_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pr_ser,
            de: ::grpcio::pr_de,
        },
    };
const METHOD_DEBUG_SCAN_MVCC: ::grpcio::Method<ScanMvccRequest, ScanMvccResponse> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::ServerStreaming,
        name: "/debugpb.Debug/ScanMvcc",
        req_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pr_ser,
            de: ::grpcio::pr_de,
        },
        resp_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pr_ser,
            de: ::grpcio::pr_de,
        },
    };
const METHOD_DEBUG_COMPACT: ::grpcio::Method<CompactRequest, CompactResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/debugpb.Debug/Compact",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
};
const METHOD_DEBUG_INJECT_FAIL_POINT: ::grpcio::Method<
    InjectFailPointRequest,
    InjectFailPointResponse,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/debugpb.Debug/InjectFailPoint",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
};
const METHOD_DEBUG_RECOVER_FAIL_POINT: ::grpcio::Method<
    RecoverFailPointRequest,
    RecoverFailPointResponse,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/debugpb.Debug/RecoverFailPoint",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
};
const METHOD_DEBUG_LIST_FAIL_POINTS: ::grpcio::Method<
    ListFailPointsRequest,
    ListFailPointsResponse,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/debugpb.Debug/ListFailPoints",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
};
const METHOD_DEBUG_GET_METRICS: ::grpcio::Method<GetMetricsRequest, GetMetricsResponse> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::Unary,
        name: "/debugpb.Debug/GetMetrics",
        req_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pr_ser,
            de: ::grpcio::pr_de,
        },
        resp_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pr_ser,
            de: ::grpcio::pr_de,
        },
    };
const METHOD_DEBUG_CHECK_REGION_CONSISTENCY: ::grpcio::Method<
    RegionConsistencyCheckRequest,
    RegionConsistencyCheckResponse,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/debugpb.Debug/CheckRegionConsistency",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
};
const METHOD_DEBUG_MODIFY_TIKV_CONFIG: ::grpcio::Method<
    ModifyTikvConfigRequest,
    ModifyTikvConfigResponse,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/debugpb.Debug/ModifyTikvConfig",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
};
const METHOD_DEBUG_GET_REGION_PROPERTIES: ::grpcio::Method<
    GetRegionPropertiesRequest,
    GetRegionPropertiesResponse,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/debugpb.Debug/GetRegionProperties",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
};
#[derive(Clone)]
pub struct DebugClient {
    client: ::grpcio::Client,
}
impl DebugClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        DebugClient {
            client: ::grpcio::Client::new(channel),
        }
    }
    pub fn get_opt(
        &self,
        req: &GetRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<GetResponse> {
        self.client.unary_call(&METHOD_DEBUG_GET, req, opt)
    }
    pub fn get(&self, req: &GetRequest) -> ::grpcio::Result<GetResponse> {
        self.get_opt(req, ::grpcio::CallOption::default())
    }
    pub fn get_async_opt(
        &self,
        req: &GetRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<GetResponse>> {
        self.client.unary_call_async(&METHOD_DEBUG_GET, req, opt)
    }
    pub fn get_async(
        &self,
        req: &GetRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<GetResponse>> {
        self.get_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn raft_log_opt(
        &self,
        req: &RaftLogRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<RaftLogResponse> {
        self.client.unary_call(&METHOD_DEBUG_RAFT_LOG, req, opt)
    }
    pub fn raft_log(&self, req: &RaftLogRequest) -> ::grpcio::Result<RaftLogResponse> {
        self.raft_log_opt(req, ::grpcio::CallOption::default())
    }
    pub fn raft_log_async_opt(
        &self,
        req: &RaftLogRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<RaftLogResponse>> {
        self.client
            .unary_call_async(&METHOD_DEBUG_RAFT_LOG, req, opt)
    }
    pub fn raft_log_async(
        &self,
        req: &RaftLogRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<RaftLogResponse>> {
        self.raft_log_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn region_info_opt(
        &self,
        req: &RegionInfoRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<RegionInfoResponse> {
        self.client.unary_call(&METHOD_DEBUG_REGION_INFO, req, opt)
    }
    pub fn region_info(&self, req: &RegionInfoRequest) -> ::grpcio::Result<RegionInfoResponse> {
        self.region_info_opt(req, ::grpcio::CallOption::default())
    }
    pub fn region_info_async_opt(
        &self,
        req: &RegionInfoRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<RegionInfoResponse>> {
        self.client
            .unary_call_async(&METHOD_DEBUG_REGION_INFO, req, opt)
    }
    pub fn region_info_async(
        &self,
        req: &RegionInfoRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<RegionInfoResponse>> {
        self.region_info_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn region_size_opt(
        &self,
        req: &RegionSizeRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<RegionSizeResponse> {
        self.client.unary_call(&METHOD_DEBUG_REGION_SIZE, req, opt)
    }
    pub fn region_size(&self, req: &RegionSizeRequest) -> ::grpcio::Result<RegionSizeResponse> {
        self.region_size_opt(req, ::grpcio::CallOption::default())
    }
    pub fn region_size_async_opt(
        &self,
        req: &RegionSizeRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<RegionSizeResponse>> {
        self.client
            .unary_call_async(&METHOD_DEBUG_REGION_SIZE, req, opt)
    }
    pub fn region_size_async(
        &self,
        req: &RegionSizeRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<RegionSizeResponse>> {
        self.region_size_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn scan_mvcc_opt(
        &self,
        req: &ScanMvccRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<ScanMvccResponse>> {
        self.client
            .server_streaming(&METHOD_DEBUG_SCAN_MVCC, req, opt)
    }
    pub fn scan_mvcc(
        &self,
        req: &ScanMvccRequest,
    ) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<ScanMvccResponse>> {
        self.scan_mvcc_opt(req, ::grpcio::CallOption::default())
    }
    pub fn compact_opt(
        &self,
        req: &CompactRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<CompactResponse> {
        self.client.unary_call(&METHOD_DEBUG_COMPACT, req, opt)
    }
    pub fn compact(&self, req: &CompactRequest) -> ::grpcio::Result<CompactResponse> {
        self.compact_opt(req, ::grpcio::CallOption::default())
    }
    pub fn compact_async_opt(
        &self,
        req: &CompactRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<CompactResponse>> {
        self.client
            .unary_call_async(&METHOD_DEBUG_COMPACT, req, opt)
    }
    pub fn compact_async(
        &self,
        req: &CompactRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<CompactResponse>> {
        self.compact_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn inject_fail_point_opt(
        &self,
        req: &InjectFailPointRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<InjectFailPointResponse> {
        self.client
            .unary_call(&METHOD_DEBUG_INJECT_FAIL_POINT, req, opt)
    }
    pub fn inject_fail_point(
        &self,
        req: &InjectFailPointRequest,
    ) -> ::grpcio::Result<InjectFailPointResponse> {
        self.inject_fail_point_opt(req, ::grpcio::CallOption::default())
    }
    pub fn inject_fail_point_async_opt(
        &self,
        req: &InjectFailPointRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<InjectFailPointResponse>> {
        self.client
            .unary_call_async(&METHOD_DEBUG_INJECT_FAIL_POINT, req, opt)
    }
    pub fn inject_fail_point_async(
        &self,
        req: &InjectFailPointRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<InjectFailPointResponse>> {
        self.inject_fail_point_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn recover_fail_point_opt(
        &self,
        req: &RecoverFailPointRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<RecoverFailPointResponse> {
        self.client
            .unary_call(&METHOD_DEBUG_RECOVER_FAIL_POINT, req, opt)
    }
    pub fn recover_fail_point(
        &self,
        req: &RecoverFailPointRequest,
    ) -> ::grpcio::Result<RecoverFailPointResponse> {
        self.recover_fail_point_opt(req, ::grpcio::CallOption::default())
    }
    pub fn recover_fail_point_async_opt(
        &self,
        req: &RecoverFailPointRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<RecoverFailPointResponse>> {
        self.client
            .unary_call_async(&METHOD_DEBUG_RECOVER_FAIL_POINT, req, opt)
    }
    pub fn recover_fail_point_async(
        &self,
        req: &RecoverFailPointRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<RecoverFailPointResponse>> {
        self.recover_fail_point_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn list_fail_points_opt(
        &self,
        req: &ListFailPointsRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<ListFailPointsResponse> {
        self.client
            .unary_call(&METHOD_DEBUG_LIST_FAIL_POINTS, req, opt)
    }
    pub fn list_fail_points(
        &self,
        req: &ListFailPointsRequest,
    ) -> ::grpcio::Result<ListFailPointsResponse> {
        self.list_fail_points_opt(req, ::grpcio::CallOption::default())
    }
    pub fn list_fail_points_async_opt(
        &self,
        req: &ListFailPointsRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<ListFailPointsResponse>> {
        self.client
            .unary_call_async(&METHOD_DEBUG_LIST_FAIL_POINTS, req, opt)
    }
    pub fn list_fail_points_async(
        &self,
        req: &ListFailPointsRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<ListFailPointsResponse>> {
        self.list_fail_points_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn get_metrics_opt(
        &self,
        req: &GetMetricsRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<GetMetricsResponse> {
        self.client.unary_call(&METHOD_DEBUG_GET_METRICS, req, opt)
    }
    pub fn get_metrics(&self, req: &GetMetricsRequest) -> ::grpcio::Result<GetMetricsResponse> {
        self.get_metrics_opt(req, ::grpcio::CallOption::default())
    }
    pub fn get_metrics_async_opt(
        &self,
        req: &GetMetricsRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<GetMetricsResponse>> {
        self.client
            .unary_call_async(&METHOD_DEBUG_GET_METRICS, req, opt)
    }
    pub fn get_metrics_async(
        &self,
        req: &GetMetricsRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<GetMetricsResponse>> {
        self.get_metrics_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn check_region_consistency_opt(
        &self,
        req: &RegionConsistencyCheckRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<RegionConsistencyCheckResponse> {
        self.client
            .unary_call(&METHOD_DEBUG_CHECK_REGION_CONSISTENCY, req, opt)
    }
    pub fn check_region_consistency(
        &self,
        req: &RegionConsistencyCheckRequest,
    ) -> ::grpcio::Result<RegionConsistencyCheckResponse> {
        self.check_region_consistency_opt(req, ::grpcio::CallOption::default())
    }
    pub fn check_region_consistency_async_opt(
        &self,
        req: &RegionConsistencyCheckRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<RegionConsistencyCheckResponse>> {
        self.client
            .unary_call_async(&METHOD_DEBUG_CHECK_REGION_CONSISTENCY, req, opt)
    }
    pub fn check_region_consistency_async(
        &self,
        req: &RegionConsistencyCheckRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<RegionConsistencyCheckResponse>> {
        self.check_region_consistency_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn modify_tikv_config_opt(
        &self,
        req: &ModifyTikvConfigRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<ModifyTikvConfigResponse> {
        self.client
            .unary_call(&METHOD_DEBUG_MODIFY_TIKV_CONFIG, req, opt)
    }
    pub fn modify_tikv_config(
        &self,
        req: &ModifyTikvConfigRequest,
    ) -> ::grpcio::Result<ModifyTikvConfigResponse> {
        self.modify_tikv_config_opt(req, ::grpcio::CallOption::default())
    }
    pub fn modify_tikv_config_async_opt(
        &self,
        req: &ModifyTikvConfigRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<ModifyTikvConfigResponse>> {
        self.client
            .unary_call_async(&METHOD_DEBUG_MODIFY_TIKV_CONFIG, req, opt)
    }
    pub fn modify_tikv_config_async(
        &self,
        req: &ModifyTikvConfigRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<ModifyTikvConfigResponse>> {
        self.modify_tikv_config_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn get_region_properties_opt(
        &self,
        req: &GetRegionPropertiesRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<GetRegionPropertiesResponse> {
        self.client
            .unary_call(&METHOD_DEBUG_GET_REGION_PROPERTIES, req, opt)
    }
    pub fn get_region_properties(
        &self,
        req: &GetRegionPropertiesRequest,
    ) -> ::grpcio::Result<GetRegionPropertiesResponse> {
        self.get_region_properties_opt(req, ::grpcio::CallOption::default())
    }
    pub fn get_region_properties_async_opt(
        &self,
        req: &GetRegionPropertiesRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<GetRegionPropertiesResponse>> {
        self.client
            .unary_call_async(&METHOD_DEBUG_GET_REGION_PROPERTIES, req, opt)
    }
    pub fn get_region_properties_async(
        &self,
        req: &GetRegionPropertiesRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<GetRegionPropertiesResponse>> {
        self.get_region_properties_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F)
    where
        F: ::futures::Future<Item = (), Error = ()> + Send + 'static,
    {
        self.client.spawn(f)
    }
}
pub trait Debug {
    fn get(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: GetRequest,
        sink: ::grpcio::UnarySink<GetResponse>,
    );
    fn raft_log(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: RaftLogRequest,
        sink: ::grpcio::UnarySink<RaftLogResponse>,
    );
    fn region_info(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: RegionInfoRequest,
        sink: ::grpcio::UnarySink<RegionInfoResponse>,
    );
    fn region_size(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: RegionSizeRequest,
        sink: ::grpcio::UnarySink<RegionSizeResponse>,
    );
    fn scan_mvcc(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: ScanMvccRequest,
        sink: ::grpcio::ServerStreamingSink<ScanMvccResponse>,
    );
    fn compact(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: CompactRequest,
        sink: ::grpcio::UnarySink<CompactResponse>,
    );
    fn inject_fail_point(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: InjectFailPointRequest,
        sink: ::grpcio::UnarySink<InjectFailPointResponse>,
    );
    fn recover_fail_point(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: RecoverFailPointRequest,
        sink: ::grpcio::UnarySink<RecoverFailPointResponse>,
    );
    fn list_fail_points(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: ListFailPointsRequest,
        sink: ::grpcio::UnarySink<ListFailPointsResponse>,
    );
    fn get_metrics(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: GetMetricsRequest,
        sink: ::grpcio::UnarySink<GetMetricsResponse>,
    );
    fn check_region_consistency(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: RegionConsistencyCheckRequest,
        sink: ::grpcio::UnarySink<RegionConsistencyCheckResponse>,
    );
    fn modify_tikv_config(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: ModifyTikvConfigRequest,
        sink: ::grpcio::UnarySink<ModifyTikvConfigResponse>,
    );
    fn get_region_properties(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: GetRegionPropertiesRequest,
        sink: ::grpcio::UnarySink<GetRegionPropertiesResponse>,
    );
}
pub fn create_debug<S: Debug + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_DEBUG_GET, move |ctx, req, resp| {
        instance.get(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_DEBUG_RAFT_LOG, move |ctx, req, resp| {
        instance.raft_log(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_DEBUG_REGION_INFO, move |ctx, req, resp| {
        instance.region_info(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_DEBUG_REGION_SIZE, move |ctx, req, resp| {
        instance.region_size(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder
        .add_server_streaming_handler(&METHOD_DEBUG_SCAN_MVCC, move |ctx, req, resp| {
            instance.scan_mvcc(ctx, req, resp)
        });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_DEBUG_COMPACT, move |ctx, req, resp| {
        instance.compact(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_DEBUG_INJECT_FAIL_POINT, move |ctx, req, resp| {
        instance.inject_fail_point(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_DEBUG_RECOVER_FAIL_POINT, move |ctx, req, resp| {
        instance.recover_fail_point(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_DEBUG_LIST_FAIL_POINTS, move |ctx, req, resp| {
        instance.list_fail_points(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_DEBUG_GET_METRICS, move |ctx, req, resp| {
        instance.get_metrics(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_DEBUG_CHECK_REGION_CONSISTENCY,
        move |ctx, req, resp| instance.check_region_consistency(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_DEBUG_MODIFY_TIKV_CONFIG, move |ctx, req, resp| {
        instance.modify_tikv_config(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_DEBUG_GET_REGION_PROPERTIES,
        move |ctx, req, resp| instance.get_region_properties(ctx, req, resp),
    );
    builder.build()
}
