// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_GET_MEMBERS: ::grpc::Method = ::grpc::Method {
    ty: ::grpc::MethodType::Unary,
    name: "/pdpb.PD/GetMembers",
};

const METHOD_TSO: ::grpc::Method = ::grpc::Method {
    ty: ::grpc::MethodType::Dulex,
    name: "/pdpb.PD/Tso",
};

const METHOD_BOOTSTRAP: ::grpc::Method = ::grpc::Method {
    ty: ::grpc::MethodType::Unary,
    name: "/pdpb.PD/Bootstrap",
};

const METHOD_IS_BOOTSTRAPPED: ::grpc::Method = ::grpc::Method {
    ty: ::grpc::MethodType::Unary,
    name: "/pdpb.PD/IsBootstrapped",
};

const METHOD_ALLOC_ID: ::grpc::Method = ::grpc::Method {
    ty: ::grpc::MethodType::Unary,
    name: "/pdpb.PD/AllocID",
};

const METHOD_GET_STORE: ::grpc::Method = ::grpc::Method {
    ty: ::grpc::MethodType::Unary,
    name: "/pdpb.PD/GetStore",
};

const METHOD_PUT_STORE: ::grpc::Method = ::grpc::Method {
    ty: ::grpc::MethodType::Unary,
    name: "/pdpb.PD/PutStore",
};

const METHOD_STORE_HEARTBEAT: ::grpc::Method = ::grpc::Method {
    ty: ::grpc::MethodType::Unary,
    name: "/pdpb.PD/StoreHeartbeat",
};

const METHOD_REGION_HEARTBEAT: ::grpc::Method = ::grpc::Method {
    ty: ::grpc::MethodType::Unary,
    name: "/pdpb.PD/RegionHeartbeat",
};

const METHOD_GET_REGION: ::grpc::Method = ::grpc::Method {
    ty: ::grpc::MethodType::Unary,
    name: "/pdpb.PD/GetRegion",
};

const METHOD_GET_REGION_BY_ID: ::grpc::Method = ::grpc::Method {
    ty: ::grpc::MethodType::Unary,
    name: "/pdpb.PD/GetRegionByID",
};

const METHOD_ASK_SPLIT: ::grpc::Method = ::grpc::Method {
    ty: ::grpc::MethodType::Unary,
    name: "/pdpb.PD/AskSplit",
};

const METHOD_REPORT_SPLIT: ::grpc::Method = ::grpc::Method {
    ty: ::grpc::MethodType::Unary,
    name: "/pdpb.PD/ReportSplit",
};

const METHOD_GET_CLUSTER_CONFIG: ::grpc::Method = ::grpc::Method {
    ty: ::grpc::MethodType::Unary,
    name: "/pdpb.PD/GetClusterConfig",
};

const METHOD_PUT_CLUSTER_CONFIG: ::grpc::Method = ::grpc::Method {
    ty: ::grpc::MethodType::Unary,
    name: "/pdpb.PD/PutClusterConfig",
};

pub struct PDClient {
    client: ::grpc::Client,
}

impl PDClient {
    pub fn new(channel: ::grpc::Channel) -> Self {
        PDClient {
            client: ::grpc::Client::new(channel),
        }
    }

    pub fn get_members_opt(&self, req: super::pdpb::GetMembersRequest, opt: ::grpc::CallOption) -> ::grpc::Result<super::pdpb::GetMembersResponse> {
        self.client.unary_call(&METHOD_GET_MEMBERS, req, opt)
    }

    pub fn get_members(&self, req: super::pdpb::GetMembersRequest) -> ::grpc::Result<super::pdpb::GetMembersResponse> {
        self.get_members_opt(req, ::grpc::CallOption::default())
    }

    pub fn get_members_async_opt(&self, req: super::pdpb::GetMembersRequest, opt: ::grpc::CallOption) -> ::grpc::Result<::grpc::UnaryCallHandler<super::pdpb::GetMembersResponse>> {
        self.client.unary_call_async(&METHOD_GET_MEMBERS, req, opt)
    }

    pub fn get_members_async(&self, req: super::pdpb::GetMembersRequest) -> ::grpc::Result<::grpc::UnaryCallHandler<super::pdpb::GetMembersResponse>> {
        self.get_members_async_opt(req, ::grpc::CallOption::default())
    }

    pub fn tso_opt(&self, opt: ::grpc::CallOption) -> ::grpc::Result<::grpc::DuplexStreamingCallHandler<super::pdpb::TsoRequest, super::pdpb::TsoResponse>> {
        self.client.duplex_streaming(&METHOD_TSO, opt)
    }

    pub fn tso(&self) -> ::grpc::Result<::grpc::DuplexStreamingCallHandler<super::pdpb::TsoRequest, super::pdpb::TsoResponse>> {
        self.tso_opt(::grpc::CallOption::default())
    }

    pub fn bootstrap_opt(&self, req: super::pdpb::BootstrapRequest, opt: ::grpc::CallOption) -> ::grpc::Result<super::pdpb::BootstrapResponse> {
        self.client.unary_call(&METHOD_BOOTSTRAP, req, opt)
    }

    pub fn bootstrap(&self, req: super::pdpb::BootstrapRequest) -> ::grpc::Result<super::pdpb::BootstrapResponse> {
        self.bootstrap_opt(req, ::grpc::CallOption::default())
    }

    pub fn bootstrap_async_opt(&self, req: super::pdpb::BootstrapRequest, opt: ::grpc::CallOption) -> ::grpc::Result<::grpc::UnaryCallHandler<super::pdpb::BootstrapResponse>> {
        self.client.unary_call_async(&METHOD_BOOTSTRAP, req, opt)
    }

    pub fn bootstrap_async(&self, req: super::pdpb::BootstrapRequest) -> ::grpc::Result<::grpc::UnaryCallHandler<super::pdpb::BootstrapResponse>> {
        self.bootstrap_async_opt(req, ::grpc::CallOption::default())
    }

    pub fn is_bootstrapped_opt(&self, req: super::pdpb::IsBootstrappedRequest, opt: ::grpc::CallOption) -> ::grpc::Result<super::pdpb::IsBootstrappedResponse> {
        self.client.unary_call(&METHOD_IS_BOOTSTRAPPED, req, opt)
    }

    pub fn is_bootstrapped(&self, req: super::pdpb::IsBootstrappedRequest) -> ::grpc::Result<super::pdpb::IsBootstrappedResponse> {
        self.is_bootstrapped_opt(req, ::grpc::CallOption::default())
    }

    pub fn is_bootstrapped_async_opt(&self, req: super::pdpb::IsBootstrappedRequest, opt: ::grpc::CallOption) -> ::grpc::Result<::grpc::UnaryCallHandler<super::pdpb::IsBootstrappedResponse>> {
        self.client.unary_call_async(&METHOD_IS_BOOTSTRAPPED, req, opt)
    }

    pub fn is_bootstrapped_async(&self, req: super::pdpb::IsBootstrappedRequest) -> ::grpc::Result<::grpc::UnaryCallHandler<super::pdpb::IsBootstrappedResponse>> {
        self.is_bootstrapped_async_opt(req, ::grpc::CallOption::default())
    }

    pub fn alloc_id_opt(&self, req: super::pdpb::AllocIDRequest, opt: ::grpc::CallOption) -> ::grpc::Result<super::pdpb::AllocIDResponse> {
        self.client.unary_call(&METHOD_ALLOC_ID, req, opt)
    }

    pub fn alloc_id(&self, req: super::pdpb::AllocIDRequest) -> ::grpc::Result<super::pdpb::AllocIDResponse> {
        self.alloc_id_opt(req, ::grpc::CallOption::default())
    }

    pub fn alloc_id_async_opt(&self, req: super::pdpb::AllocIDRequest, opt: ::grpc::CallOption) -> ::grpc::Result<::grpc::UnaryCallHandler<super::pdpb::AllocIDResponse>> {
        self.client.unary_call_async(&METHOD_ALLOC_ID, req, opt)
    }

    pub fn alloc_id_async(&self, req: super::pdpb::AllocIDRequest) -> ::grpc::Result<::grpc::UnaryCallHandler<super::pdpb::AllocIDResponse>> {
        self.alloc_id_async_opt(req, ::grpc::CallOption::default())
    }

    pub fn get_store_opt(&self, req: super::pdpb::GetStoreRequest, opt: ::grpc::CallOption) -> ::grpc::Result<super::pdpb::GetStoreResponse> {
        self.client.unary_call(&METHOD_GET_STORE, req, opt)
    }

    pub fn get_store(&self, req: super::pdpb::GetStoreRequest) -> ::grpc::Result<super::pdpb::GetStoreResponse> {
        self.get_store_opt(req, ::grpc::CallOption::default())
    }

    pub fn get_store_async_opt(&self, req: super::pdpb::GetStoreRequest, opt: ::grpc::CallOption) -> ::grpc::Result<::grpc::UnaryCallHandler<super::pdpb::GetStoreResponse>> {
        self.client.unary_call_async(&METHOD_GET_STORE, req, opt)
    }

    pub fn get_store_async(&self, req: super::pdpb::GetStoreRequest) -> ::grpc::Result<::grpc::UnaryCallHandler<super::pdpb::GetStoreResponse>> {
        self.get_store_async_opt(req, ::grpc::CallOption::default())
    }

    pub fn put_store_opt(&self, req: super::pdpb::PutStoreRequest, opt: ::grpc::CallOption) -> ::grpc::Result<super::pdpb::PutStoreResponse> {
        self.client.unary_call(&METHOD_PUT_STORE, req, opt)
    }

    pub fn put_store(&self, req: super::pdpb::PutStoreRequest) -> ::grpc::Result<super::pdpb::PutStoreResponse> {
        self.put_store_opt(req, ::grpc::CallOption::default())
    }

    pub fn put_store_async_opt(&self, req: super::pdpb::PutStoreRequest, opt: ::grpc::CallOption) -> ::grpc::Result<::grpc::UnaryCallHandler<super::pdpb::PutStoreResponse>> {
        self.client.unary_call_async(&METHOD_PUT_STORE, req, opt)
    }

    pub fn put_store_async(&self, req: super::pdpb::PutStoreRequest) -> ::grpc::Result<::grpc::UnaryCallHandler<super::pdpb::PutStoreResponse>> {
        self.put_store_async_opt(req, ::grpc::CallOption::default())
    }

    pub fn store_heartbeat_opt(&self, req: super::pdpb::StoreHeartbeatRequest, opt: ::grpc::CallOption) -> ::grpc::Result<super::pdpb::StoreHeartbeatResponse> {
        self.client.unary_call(&METHOD_STORE_HEARTBEAT, req, opt)
    }

    pub fn store_heartbeat(&self, req: super::pdpb::StoreHeartbeatRequest) -> ::grpc::Result<super::pdpb::StoreHeartbeatResponse> {
        self.store_heartbeat_opt(req, ::grpc::CallOption::default())
    }

    pub fn store_heartbeat_async_opt(&self, req: super::pdpb::StoreHeartbeatRequest, opt: ::grpc::CallOption) -> ::grpc::Result<::grpc::UnaryCallHandler<super::pdpb::StoreHeartbeatResponse>> {
        self.client.unary_call_async(&METHOD_STORE_HEARTBEAT, req, opt)
    }

    pub fn store_heartbeat_async(&self, req: super::pdpb::StoreHeartbeatRequest) -> ::grpc::Result<::grpc::UnaryCallHandler<super::pdpb::StoreHeartbeatResponse>> {
        self.store_heartbeat_async_opt(req, ::grpc::CallOption::default())
    }

    pub fn region_heartbeat_opt(&self, req: super::pdpb::RegionHeartbeatRequest, opt: ::grpc::CallOption) -> ::grpc::Result<super::pdpb::RegionHeartbeatResponse> {
        self.client.unary_call(&METHOD_REGION_HEARTBEAT, req, opt)
    }

    pub fn region_heartbeat(&self, req: super::pdpb::RegionHeartbeatRequest) -> ::grpc::Result<super::pdpb::RegionHeartbeatResponse> {
        self.region_heartbeat_opt(req, ::grpc::CallOption::default())
    }

    pub fn region_heartbeat_async_opt(&self, req: super::pdpb::RegionHeartbeatRequest, opt: ::grpc::CallOption) -> ::grpc::Result<::grpc::UnaryCallHandler<super::pdpb::RegionHeartbeatResponse>> {
        self.client.unary_call_async(&METHOD_REGION_HEARTBEAT, req, opt)
    }

    pub fn region_heartbeat_async(&self, req: super::pdpb::RegionHeartbeatRequest) -> ::grpc::Result<::grpc::UnaryCallHandler<super::pdpb::RegionHeartbeatResponse>> {
        self.region_heartbeat_async_opt(req, ::grpc::CallOption::default())
    }

    pub fn get_region_opt(&self, req: super::pdpb::GetRegionRequest, opt: ::grpc::CallOption) -> ::grpc::Result<super::pdpb::GetRegionResponse> {
        self.client.unary_call(&METHOD_GET_REGION, req, opt)
    }

    pub fn get_region(&self, req: super::pdpb::GetRegionRequest) -> ::grpc::Result<super::pdpb::GetRegionResponse> {
        self.get_region_opt(req, ::grpc::CallOption::default())
    }

    pub fn get_region_async_opt(&self, req: super::pdpb::GetRegionRequest, opt: ::grpc::CallOption) -> ::grpc::Result<::grpc::UnaryCallHandler<super::pdpb::GetRegionResponse>> {
        self.client.unary_call_async(&METHOD_GET_REGION, req, opt)
    }

    pub fn get_region_async(&self, req: super::pdpb::GetRegionRequest) -> ::grpc::Result<::grpc::UnaryCallHandler<super::pdpb::GetRegionResponse>> {
        self.get_region_async_opt(req, ::grpc::CallOption::default())
    }

    pub fn get_region_by_id_opt(&self, req: super::pdpb::GetRegionByIDRequest, opt: ::grpc::CallOption) -> ::grpc::Result<super::pdpb::GetRegionResponse> {
        self.client.unary_call(&METHOD_GET_REGION_BY_ID, req, opt)
    }

    pub fn get_region_by_id(&self, req: super::pdpb::GetRegionByIDRequest) -> ::grpc::Result<super::pdpb::GetRegionResponse> {
        self.get_region_by_id_opt(req, ::grpc::CallOption::default())
    }

    pub fn get_region_by_id_async_opt(&self, req: super::pdpb::GetRegionByIDRequest, opt: ::grpc::CallOption) -> ::grpc::Result<::grpc::UnaryCallHandler<super::pdpb::GetRegionResponse>> {
        self.client.unary_call_async(&METHOD_GET_REGION_BY_ID, req, opt)
    }

    pub fn get_region_by_id_async(&self, req: super::pdpb::GetRegionByIDRequest) -> ::grpc::Result<::grpc::UnaryCallHandler<super::pdpb::GetRegionResponse>> {
        self.get_region_by_id_async_opt(req, ::grpc::CallOption::default())
    }

    pub fn ask_split_opt(&self, req: super::pdpb::AskSplitRequest, opt: ::grpc::CallOption) -> ::grpc::Result<super::pdpb::AskSplitResponse> {
        self.client.unary_call(&METHOD_ASK_SPLIT, req, opt)
    }

    pub fn ask_split(&self, req: super::pdpb::AskSplitRequest) -> ::grpc::Result<super::pdpb::AskSplitResponse> {
        self.ask_split_opt(req, ::grpc::CallOption::default())
    }

    pub fn ask_split_async_opt(&self, req: super::pdpb::AskSplitRequest, opt: ::grpc::CallOption) -> ::grpc::Result<::grpc::UnaryCallHandler<super::pdpb::AskSplitResponse>> {
        self.client.unary_call_async(&METHOD_ASK_SPLIT, req, opt)
    }

    pub fn ask_split_async(&self, req: super::pdpb::AskSplitRequest) -> ::grpc::Result<::grpc::UnaryCallHandler<super::pdpb::AskSplitResponse>> {
        self.ask_split_async_opt(req, ::grpc::CallOption::default())
    }

    pub fn report_split_opt(&self, req: super::pdpb::ReportSplitRequest, opt: ::grpc::CallOption) -> ::grpc::Result<super::pdpb::ReportSplitResponse> {
        self.client.unary_call(&METHOD_REPORT_SPLIT, req, opt)
    }

    pub fn report_split(&self, req: super::pdpb::ReportSplitRequest) -> ::grpc::Result<super::pdpb::ReportSplitResponse> {
        self.report_split_opt(req, ::grpc::CallOption::default())
    }

    pub fn report_split_async_opt(&self, req: super::pdpb::ReportSplitRequest, opt: ::grpc::CallOption) -> ::grpc::Result<::grpc::UnaryCallHandler<super::pdpb::ReportSplitResponse>> {
        self.client.unary_call_async(&METHOD_REPORT_SPLIT, req, opt)
    }

    pub fn report_split_async(&self, req: super::pdpb::ReportSplitRequest) -> ::grpc::Result<::grpc::UnaryCallHandler<super::pdpb::ReportSplitResponse>> {
        self.report_split_async_opt(req, ::grpc::CallOption::default())
    }

    pub fn get_cluster_config_opt(&self, req: super::pdpb::GetClusterConfigRequest, opt: ::grpc::CallOption) -> ::grpc::Result<super::pdpb::GetClusterConfigResponse> {
        self.client.unary_call(&METHOD_GET_CLUSTER_CONFIG, req, opt)
    }

    pub fn get_cluster_config(&self, req: super::pdpb::GetClusterConfigRequest) -> ::grpc::Result<super::pdpb::GetClusterConfigResponse> {
        self.get_cluster_config_opt(req, ::grpc::CallOption::default())
    }

    pub fn get_cluster_config_async_opt(&self, req: super::pdpb::GetClusterConfigRequest, opt: ::grpc::CallOption) -> ::grpc::Result<::grpc::UnaryCallHandler<super::pdpb::GetClusterConfigResponse>> {
        self.client.unary_call_async(&METHOD_GET_CLUSTER_CONFIG, req, opt)
    }

    pub fn get_cluster_config_async(&self, req: super::pdpb::GetClusterConfigRequest) -> ::grpc::Result<::grpc::UnaryCallHandler<super::pdpb::GetClusterConfigResponse>> {
        self.get_cluster_config_async_opt(req, ::grpc::CallOption::default())
    }

    pub fn put_cluster_config_opt(&self, req: super::pdpb::PutClusterConfigRequest, opt: ::grpc::CallOption) -> ::grpc::Result<super::pdpb::PutClusterConfigResponse> {
        self.client.unary_call(&METHOD_PUT_CLUSTER_CONFIG, req, opt)
    }

    pub fn put_cluster_config(&self, req: super::pdpb::PutClusterConfigRequest) -> ::grpc::Result<super::pdpb::PutClusterConfigResponse> {
        self.put_cluster_config_opt(req, ::grpc::CallOption::default())
    }

    pub fn put_cluster_config_async_opt(&self, req: super::pdpb::PutClusterConfigRequest, opt: ::grpc::CallOption) -> ::grpc::Result<::grpc::UnaryCallHandler<super::pdpb::PutClusterConfigResponse>> {
        self.client.unary_call_async(&METHOD_PUT_CLUSTER_CONFIG, req, opt)
    }

    pub fn put_cluster_config_async(&self, req: super::pdpb::PutClusterConfigRequest) -> ::grpc::Result<::grpc::UnaryCallHandler<super::pdpb::PutClusterConfigResponse>> {
        self.put_cluster_config_async_opt(req, ::grpc::CallOption::default())
    }
}
