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

const METHOD_PD_GET_MEMBERS: ::grpc::Method<super::pdpb::GetMembersRequest, super::pdpb::GetMembersResponse> = ::grpc::Method {
    ty: ::grpc::MethodType::Unary,
    name: "/pdpb.PD/GetMembers",
    req_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
    resp_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
};

const METHOD_PD_TSO: ::grpc::Method<super::pdpb::TsoRequest, super::pdpb::TsoResponse> = ::grpc::Method {
    ty: ::grpc::MethodType::Duplex,
    name: "/pdpb.PD/Tso",
    req_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
    resp_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
};

const METHOD_PD_BOOTSTRAP: ::grpc::Method<super::pdpb::BootstrapRequest, super::pdpb::BootstrapResponse> = ::grpc::Method {
    ty: ::grpc::MethodType::Unary,
    name: "/pdpb.PD/Bootstrap",
    req_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
    resp_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
};

const METHOD_PD_IS_BOOTSTRAPPED: ::grpc::Method<super::pdpb::IsBootstrappedRequest, super::pdpb::IsBootstrappedResponse> = ::grpc::Method {
    ty: ::grpc::MethodType::Unary,
    name: "/pdpb.PD/IsBootstrapped",
    req_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
    resp_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
};

const METHOD_PD_ALLOC_ID: ::grpc::Method<super::pdpb::AllocIDRequest, super::pdpb::AllocIDResponse> = ::grpc::Method {
    ty: ::grpc::MethodType::Unary,
    name: "/pdpb.PD/AllocID",
    req_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
    resp_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
};

const METHOD_PD_GET_STORE: ::grpc::Method<super::pdpb::GetStoreRequest, super::pdpb::GetStoreResponse> = ::grpc::Method {
    ty: ::grpc::MethodType::Unary,
    name: "/pdpb.PD/GetStore",
    req_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
    resp_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
};

const METHOD_PD_PUT_STORE: ::grpc::Method<super::pdpb::PutStoreRequest, super::pdpb::PutStoreResponse> = ::grpc::Method {
    ty: ::grpc::MethodType::Unary,
    name: "/pdpb.PD/PutStore",
    req_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
    resp_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
};

const METHOD_PD_STORE_HEARTBEAT: ::grpc::Method<super::pdpb::StoreHeartbeatRequest, super::pdpb::StoreHeartbeatResponse> = ::grpc::Method {
    ty: ::grpc::MethodType::Unary,
    name: "/pdpb.PD/StoreHeartbeat",
    req_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
    resp_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
};

const METHOD_PD_REGION_HEARTBEAT: ::grpc::Method<super::pdpb::RegionHeartbeatRequest, super::pdpb::RegionHeartbeatResponse> = ::grpc::Method {
    ty: ::grpc::MethodType::Unary,
    name: "/pdpb.PD/RegionHeartbeat",
    req_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
    resp_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
};

const METHOD_PD_GET_REGION: ::grpc::Method<super::pdpb::GetRegionRequest, super::pdpb::GetRegionResponse> = ::grpc::Method {
    ty: ::grpc::MethodType::Unary,
    name: "/pdpb.PD/GetRegion",
    req_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
    resp_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
};

const METHOD_PD_GET_REGION_BY_ID: ::grpc::Method<super::pdpb::GetRegionByIDRequest, super::pdpb::GetRegionResponse> = ::grpc::Method {
    ty: ::grpc::MethodType::Unary,
    name: "/pdpb.PD/GetRegionByID",
    req_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
    resp_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
};

const METHOD_PD_ASK_SPLIT: ::grpc::Method<super::pdpb::AskSplitRequest, super::pdpb::AskSplitResponse> = ::grpc::Method {
    ty: ::grpc::MethodType::Unary,
    name: "/pdpb.PD/AskSplit",
    req_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
    resp_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
};

const METHOD_PD_REPORT_SPLIT: ::grpc::Method<super::pdpb::ReportSplitRequest, super::pdpb::ReportSplitResponse> = ::grpc::Method {
    ty: ::grpc::MethodType::Unary,
    name: "/pdpb.PD/ReportSplit",
    req_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
    resp_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
};

const METHOD_PD_GET_CLUSTER_CONFIG: ::grpc::Method<super::pdpb::GetClusterConfigRequest, super::pdpb::GetClusterConfigResponse> = ::grpc::Method {
    ty: ::grpc::MethodType::Unary,
    name: "/pdpb.PD/GetClusterConfig",
    req_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
    resp_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
};

const METHOD_PD_PUT_CLUSTER_CONFIG: ::grpc::Method<super::pdpb::PutClusterConfigRequest, super::pdpb::PutClusterConfigResponse> = ::grpc::Method {
    ty: ::grpc::MethodType::Unary,
    name: "/pdpb.PD/PutClusterConfig",
    req_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
    resp_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
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
        self.client.unary_call(&METHOD_PD_GET_MEMBERS, req, opt)
    }

    pub fn get_members(&self, req: super::pdpb::GetMembersRequest) -> ::grpc::Result<super::pdpb::GetMembersResponse> {
        self.get_members_opt(req, ::grpc::CallOption::default())
    }

    pub fn get_members_async_opt(&self, req: super::pdpb::GetMembersRequest, opt: ::grpc::CallOption) -> ::grpc::UnaryCallHandler<super::pdpb::GetMembersResponse> {
        self.client.unary_call_async(&METHOD_PD_GET_MEMBERS, req, opt)
    }

    pub fn get_members_async(&self, req: super::pdpb::GetMembersRequest) -> ::grpc::UnaryCallHandler<super::pdpb::GetMembersResponse> {
        self.get_members_async_opt(req, ::grpc::CallOption::default())
    }

    pub fn tso_opt(&self, opt: ::grpc::CallOption) -> ::grpc::DuplexCallHandler<super::pdpb::TsoRequest, super::pdpb::TsoResponse> {
        self.client.duplex_streaming(&METHOD_PD_TSO, opt)
    }

    pub fn tso(&self) -> ::grpc::DuplexCallHandler<super::pdpb::TsoRequest, super::pdpb::TsoResponse> {
        self.tso_opt(::grpc::CallOption::default())
    }

    pub fn bootstrap_opt(&self, req: super::pdpb::BootstrapRequest, opt: ::grpc::CallOption) -> ::grpc::Result<super::pdpb::BootstrapResponse> {
        self.client.unary_call(&METHOD_PD_BOOTSTRAP, req, opt)
    }

    pub fn bootstrap(&self, req: super::pdpb::BootstrapRequest) -> ::grpc::Result<super::pdpb::BootstrapResponse> {
        self.bootstrap_opt(req, ::grpc::CallOption::default())
    }

    pub fn bootstrap_async_opt(&self, req: super::pdpb::BootstrapRequest, opt: ::grpc::CallOption) -> ::grpc::UnaryCallHandler<super::pdpb::BootstrapResponse> {
        self.client.unary_call_async(&METHOD_PD_BOOTSTRAP, req, opt)
    }

    pub fn bootstrap_async(&self, req: super::pdpb::BootstrapRequest) -> ::grpc::UnaryCallHandler<super::pdpb::BootstrapResponse> {
        self.bootstrap_async_opt(req, ::grpc::CallOption::default())
    }

    pub fn is_bootstrapped_opt(&self, req: super::pdpb::IsBootstrappedRequest, opt: ::grpc::CallOption) -> ::grpc::Result<super::pdpb::IsBootstrappedResponse> {
        self.client.unary_call(&METHOD_PD_IS_BOOTSTRAPPED, req, opt)
    }

    pub fn is_bootstrapped(&self, req: super::pdpb::IsBootstrappedRequest) -> ::grpc::Result<super::pdpb::IsBootstrappedResponse> {
        self.is_bootstrapped_opt(req, ::grpc::CallOption::default())
    }

    pub fn is_bootstrapped_async_opt(&self, req: super::pdpb::IsBootstrappedRequest, opt: ::grpc::CallOption) -> ::grpc::UnaryCallHandler<super::pdpb::IsBootstrappedResponse> {
        self.client.unary_call_async(&METHOD_PD_IS_BOOTSTRAPPED, req, opt)
    }

    pub fn is_bootstrapped_async(&self, req: super::pdpb::IsBootstrappedRequest) -> ::grpc::UnaryCallHandler<super::pdpb::IsBootstrappedResponse> {
        self.is_bootstrapped_async_opt(req, ::grpc::CallOption::default())
    }

    pub fn alloc_id_opt(&self, req: super::pdpb::AllocIDRequest, opt: ::grpc::CallOption) -> ::grpc::Result<super::pdpb::AllocIDResponse> {
        self.client.unary_call(&METHOD_PD_ALLOC_ID, req, opt)
    }

    pub fn alloc_id(&self, req: super::pdpb::AllocIDRequest) -> ::grpc::Result<super::pdpb::AllocIDResponse> {
        self.alloc_id_opt(req, ::grpc::CallOption::default())
    }

    pub fn alloc_id_async_opt(&self, req: super::pdpb::AllocIDRequest, opt: ::grpc::CallOption) -> ::grpc::UnaryCallHandler<super::pdpb::AllocIDResponse> {
        self.client.unary_call_async(&METHOD_PD_ALLOC_ID, req, opt)
    }

    pub fn alloc_id_async(&self, req: super::pdpb::AllocIDRequest) -> ::grpc::UnaryCallHandler<super::pdpb::AllocIDResponse> {
        self.alloc_id_async_opt(req, ::grpc::CallOption::default())
    }

    pub fn get_store_opt(&self, req: super::pdpb::GetStoreRequest, opt: ::grpc::CallOption) -> ::grpc::Result<super::pdpb::GetStoreResponse> {
        self.client.unary_call(&METHOD_PD_GET_STORE, req, opt)
    }

    pub fn get_store(&self, req: super::pdpb::GetStoreRequest) -> ::grpc::Result<super::pdpb::GetStoreResponse> {
        self.get_store_opt(req, ::grpc::CallOption::default())
    }

    pub fn get_store_async_opt(&self, req: super::pdpb::GetStoreRequest, opt: ::grpc::CallOption) -> ::grpc::UnaryCallHandler<super::pdpb::GetStoreResponse> {
        self.client.unary_call_async(&METHOD_PD_GET_STORE, req, opt)
    }

    pub fn get_store_async(&self, req: super::pdpb::GetStoreRequest) -> ::grpc::UnaryCallHandler<super::pdpb::GetStoreResponse> {
        self.get_store_async_opt(req, ::grpc::CallOption::default())
    }

    pub fn put_store_opt(&self, req: super::pdpb::PutStoreRequest, opt: ::grpc::CallOption) -> ::grpc::Result<super::pdpb::PutStoreResponse> {
        self.client.unary_call(&METHOD_PD_PUT_STORE, req, opt)
    }

    pub fn put_store(&self, req: super::pdpb::PutStoreRequest) -> ::grpc::Result<super::pdpb::PutStoreResponse> {
        self.put_store_opt(req, ::grpc::CallOption::default())
    }

    pub fn put_store_async_opt(&self, req: super::pdpb::PutStoreRequest, opt: ::grpc::CallOption) -> ::grpc::UnaryCallHandler<super::pdpb::PutStoreResponse> {
        self.client.unary_call_async(&METHOD_PD_PUT_STORE, req, opt)
    }

    pub fn put_store_async(&self, req: super::pdpb::PutStoreRequest) -> ::grpc::UnaryCallHandler<super::pdpb::PutStoreResponse> {
        self.put_store_async_opt(req, ::grpc::CallOption::default())
    }

    pub fn store_heartbeat_opt(&self, req: super::pdpb::StoreHeartbeatRequest, opt: ::grpc::CallOption) -> ::grpc::Result<super::pdpb::StoreHeartbeatResponse> {
        self.client.unary_call(&METHOD_PD_STORE_HEARTBEAT, req, opt)
    }

    pub fn store_heartbeat(&self, req: super::pdpb::StoreHeartbeatRequest) -> ::grpc::Result<super::pdpb::StoreHeartbeatResponse> {
        self.store_heartbeat_opt(req, ::grpc::CallOption::default())
    }

    pub fn store_heartbeat_async_opt(&self, req: super::pdpb::StoreHeartbeatRequest, opt: ::grpc::CallOption) -> ::grpc::UnaryCallHandler<super::pdpb::StoreHeartbeatResponse> {
        self.client.unary_call_async(&METHOD_PD_STORE_HEARTBEAT, req, opt)
    }

    pub fn store_heartbeat_async(&self, req: super::pdpb::StoreHeartbeatRequest) -> ::grpc::UnaryCallHandler<super::pdpb::StoreHeartbeatResponse> {
        self.store_heartbeat_async_opt(req, ::grpc::CallOption::default())
    }

    pub fn region_heartbeat_opt(&self, req: super::pdpb::RegionHeartbeatRequest, opt: ::grpc::CallOption) -> ::grpc::Result<super::pdpb::RegionHeartbeatResponse> {
        self.client.unary_call(&METHOD_PD_REGION_HEARTBEAT, req, opt)
    }

    pub fn region_heartbeat(&self, req: super::pdpb::RegionHeartbeatRequest) -> ::grpc::Result<super::pdpb::RegionHeartbeatResponse> {
        self.region_heartbeat_opt(req, ::grpc::CallOption::default())
    }

    pub fn region_heartbeat_async_opt(&self, req: super::pdpb::RegionHeartbeatRequest, opt: ::grpc::CallOption) -> ::grpc::UnaryCallHandler<super::pdpb::RegionHeartbeatResponse> {
        self.client.unary_call_async(&METHOD_PD_REGION_HEARTBEAT, req, opt)
    }

    pub fn region_heartbeat_async(&self, req: super::pdpb::RegionHeartbeatRequest) -> ::grpc::UnaryCallHandler<super::pdpb::RegionHeartbeatResponse> {
        self.region_heartbeat_async_opt(req, ::grpc::CallOption::default())
    }

    pub fn get_region_opt(&self, req: super::pdpb::GetRegionRequest, opt: ::grpc::CallOption) -> ::grpc::Result<super::pdpb::GetRegionResponse> {
        self.client.unary_call(&METHOD_PD_GET_REGION, req, opt)
    }

    pub fn get_region(&self, req: super::pdpb::GetRegionRequest) -> ::grpc::Result<super::pdpb::GetRegionResponse> {
        self.get_region_opt(req, ::grpc::CallOption::default())
    }

    pub fn get_region_async_opt(&self, req: super::pdpb::GetRegionRequest, opt: ::grpc::CallOption) -> ::grpc::UnaryCallHandler<super::pdpb::GetRegionResponse> {
        self.client.unary_call_async(&METHOD_PD_GET_REGION, req, opt)
    }

    pub fn get_region_async(&self, req: super::pdpb::GetRegionRequest) -> ::grpc::UnaryCallHandler<super::pdpb::GetRegionResponse> {
        self.get_region_async_opt(req, ::grpc::CallOption::default())
    }

    pub fn get_region_by_id_opt(&self, req: super::pdpb::GetRegionByIDRequest, opt: ::grpc::CallOption) -> ::grpc::Result<super::pdpb::GetRegionResponse> {
        self.client.unary_call(&METHOD_PD_GET_REGION_BY_ID, req, opt)
    }

    pub fn get_region_by_id(&self, req: super::pdpb::GetRegionByIDRequest) -> ::grpc::Result<super::pdpb::GetRegionResponse> {
        self.get_region_by_id_opt(req, ::grpc::CallOption::default())
    }

    pub fn get_region_by_id_async_opt(&self, req: super::pdpb::GetRegionByIDRequest, opt: ::grpc::CallOption) -> ::grpc::UnaryCallHandler<super::pdpb::GetRegionResponse> {
        self.client.unary_call_async(&METHOD_PD_GET_REGION_BY_ID, req, opt)
    }

    pub fn get_region_by_id_async(&self, req: super::pdpb::GetRegionByIDRequest) -> ::grpc::UnaryCallHandler<super::pdpb::GetRegionResponse> {
        self.get_region_by_id_async_opt(req, ::grpc::CallOption::default())
    }

    pub fn ask_split_opt(&self, req: super::pdpb::AskSplitRequest, opt: ::grpc::CallOption) -> ::grpc::Result<super::pdpb::AskSplitResponse> {
        self.client.unary_call(&METHOD_PD_ASK_SPLIT, req, opt)
    }

    pub fn ask_split(&self, req: super::pdpb::AskSplitRequest) -> ::grpc::Result<super::pdpb::AskSplitResponse> {
        self.ask_split_opt(req, ::grpc::CallOption::default())
    }

    pub fn ask_split_async_opt(&self, req: super::pdpb::AskSplitRequest, opt: ::grpc::CallOption) -> ::grpc::UnaryCallHandler<super::pdpb::AskSplitResponse> {
        self.client.unary_call_async(&METHOD_PD_ASK_SPLIT, req, opt)
    }

    pub fn ask_split_async(&self, req: super::pdpb::AskSplitRequest) -> ::grpc::UnaryCallHandler<super::pdpb::AskSplitResponse> {
        self.ask_split_async_opt(req, ::grpc::CallOption::default())
    }

    pub fn report_split_opt(&self, req: super::pdpb::ReportSplitRequest, opt: ::grpc::CallOption) -> ::grpc::Result<super::pdpb::ReportSplitResponse> {
        self.client.unary_call(&METHOD_PD_REPORT_SPLIT, req, opt)
    }

    pub fn report_split(&self, req: super::pdpb::ReportSplitRequest) -> ::grpc::Result<super::pdpb::ReportSplitResponse> {
        self.report_split_opt(req, ::grpc::CallOption::default())
    }

    pub fn report_split_async_opt(&self, req: super::pdpb::ReportSplitRequest, opt: ::grpc::CallOption) -> ::grpc::UnaryCallHandler<super::pdpb::ReportSplitResponse> {
        self.client.unary_call_async(&METHOD_PD_REPORT_SPLIT, req, opt)
    }

    pub fn report_split_async(&self, req: super::pdpb::ReportSplitRequest) -> ::grpc::UnaryCallHandler<super::pdpb::ReportSplitResponse> {
        self.report_split_async_opt(req, ::grpc::CallOption::default())
    }

    pub fn get_cluster_config_opt(&self, req: super::pdpb::GetClusterConfigRequest, opt: ::grpc::CallOption) -> ::grpc::Result<super::pdpb::GetClusterConfigResponse> {
        self.client.unary_call(&METHOD_PD_GET_CLUSTER_CONFIG, req, opt)
    }

    pub fn get_cluster_config(&self, req: super::pdpb::GetClusterConfigRequest) -> ::grpc::Result<super::pdpb::GetClusterConfigResponse> {
        self.get_cluster_config_opt(req, ::grpc::CallOption::default())
    }

    pub fn get_cluster_config_async_opt(&self, req: super::pdpb::GetClusterConfigRequest, opt: ::grpc::CallOption) -> ::grpc::UnaryCallHandler<super::pdpb::GetClusterConfigResponse> {
        self.client.unary_call_async(&METHOD_PD_GET_CLUSTER_CONFIG, req, opt)
    }

    pub fn get_cluster_config_async(&self, req: super::pdpb::GetClusterConfigRequest) -> ::grpc::UnaryCallHandler<super::pdpb::GetClusterConfigResponse> {
        self.get_cluster_config_async_opt(req, ::grpc::CallOption::default())
    }

    pub fn put_cluster_config_opt(&self, req: super::pdpb::PutClusterConfigRequest, opt: ::grpc::CallOption) -> ::grpc::Result<super::pdpb::PutClusterConfigResponse> {
        self.client.unary_call(&METHOD_PD_PUT_CLUSTER_CONFIG, req, opt)
    }

    pub fn put_cluster_config(&self, req: super::pdpb::PutClusterConfigRequest) -> ::grpc::Result<super::pdpb::PutClusterConfigResponse> {
        self.put_cluster_config_opt(req, ::grpc::CallOption::default())
    }

    pub fn put_cluster_config_async_opt(&self, req: super::pdpb::PutClusterConfigRequest, opt: ::grpc::CallOption) -> ::grpc::UnaryCallHandler<super::pdpb::PutClusterConfigResponse> {
        self.client.unary_call_async(&METHOD_PD_PUT_CLUSTER_CONFIG, req, opt)
    }

    pub fn put_cluster_config_async(&self, req: super::pdpb::PutClusterConfigRequest) -> ::grpc::UnaryCallHandler<super::pdpb::PutClusterConfigResponse> {
        self.put_cluster_config_async_opt(req, ::grpc::CallOption::default())
    }
}

pub trait PD {
    fn get_members(&self, ctx: ::grpc::RpcContext, req: super::pdpb::GetMembersRequest, sink: ::grpc::UnarySink<super::pdpb::GetMembersResponse>);
    fn tso(&self, ctx: ::grpc::RpcContext, stream: ::grpc::RequestStream<super::pdpb::TsoRequest>, sink: ::grpc::DuplexSink<super::pdpb::TsoResponse>);
    fn bootstrap(&self, ctx: ::grpc::RpcContext, req: super::pdpb::BootstrapRequest, sink: ::grpc::UnarySink<super::pdpb::BootstrapResponse>);
    fn is_bootstrapped(&self, ctx: ::grpc::RpcContext, req: super::pdpb::IsBootstrappedRequest, sink: ::grpc::UnarySink<super::pdpb::IsBootstrappedResponse>);
    fn alloc_id(&self, ctx: ::grpc::RpcContext, req: super::pdpb::AllocIDRequest, sink: ::grpc::UnarySink<super::pdpb::AllocIDResponse>);
    fn get_store(&self, ctx: ::grpc::RpcContext, req: super::pdpb::GetStoreRequest, sink: ::grpc::UnarySink<super::pdpb::GetStoreResponse>);
    fn put_store(&self, ctx: ::grpc::RpcContext, req: super::pdpb::PutStoreRequest, sink: ::grpc::UnarySink<super::pdpb::PutStoreResponse>);
    fn store_heartbeat(&self, ctx: ::grpc::RpcContext, req: super::pdpb::StoreHeartbeatRequest, sink: ::grpc::UnarySink<super::pdpb::StoreHeartbeatResponse>);
    fn region_heartbeat(&self, ctx: ::grpc::RpcContext, req: super::pdpb::RegionHeartbeatRequest, sink: ::grpc::UnarySink<super::pdpb::RegionHeartbeatResponse>);
    fn get_region(&self, ctx: ::grpc::RpcContext, req: super::pdpb::GetRegionRequest, sink: ::grpc::UnarySink<super::pdpb::GetRegionResponse>);
    fn get_region_by_id(&self, ctx: ::grpc::RpcContext, req: super::pdpb::GetRegionByIDRequest, sink: ::grpc::UnarySink<super::pdpb::GetRegionResponse>);
    fn ask_split(&self, ctx: ::grpc::RpcContext, req: super::pdpb::AskSplitRequest, sink: ::grpc::UnarySink<super::pdpb::AskSplitResponse>);
    fn report_split(&self, ctx: ::grpc::RpcContext, req: super::pdpb::ReportSplitRequest, sink: ::grpc::UnarySink<super::pdpb::ReportSplitResponse>);
    fn get_cluster_config(&self, ctx: ::grpc::RpcContext, req: super::pdpb::GetClusterConfigRequest, sink: ::grpc::UnarySink<super::pdpb::GetClusterConfigResponse>);
    fn put_cluster_config(&self, ctx: ::grpc::RpcContext, req: super::pdpb::PutClusterConfigRequest, sink: ::grpc::UnarySink<super::pdpb::PutClusterConfigResponse>);
}

pub fn create_pd<S: PD + Send + Clone + 'static>(s: S) -> ::grpc::Service {
    let mut builder = ::grpc::ServiceBuilder::new();
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_PD_GET_MEMBERS, move |ctx, req, resp| {
        instance.get_members(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_duplex_streaming_handler(&METHOD_PD_TSO, move |ctx, req, resp| {
        instance.tso(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_PD_BOOTSTRAP, move |ctx, req, resp| {
        instance.bootstrap(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_PD_IS_BOOTSTRAPPED, move |ctx, req, resp| {
        instance.is_bootstrapped(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_PD_ALLOC_ID, move |ctx, req, resp| {
        instance.alloc_id(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_PD_GET_STORE, move |ctx, req, resp| {
        instance.get_store(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_PD_PUT_STORE, move |ctx, req, resp| {
        instance.put_store(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_PD_STORE_HEARTBEAT, move |ctx, req, resp| {
        instance.store_heartbeat(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_PD_REGION_HEARTBEAT, move |ctx, req, resp| {
        instance.region_heartbeat(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_PD_GET_REGION, move |ctx, req, resp| {
        instance.get_region(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_PD_GET_REGION_BY_ID, move |ctx, req, resp| {
        instance.get_region_by_id(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_PD_ASK_SPLIT, move |ctx, req, resp| {
        instance.ask_split(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_PD_REPORT_SPLIT, move |ctx, req, resp| {
        instance.report_split(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_PD_GET_CLUSTER_CONFIG, move |ctx, req, resp| {
        instance.get_cluster_config(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_PD_PUT_CLUSTER_CONFIG, move |ctx, req, resp| {
        instance.put_cluster_config(ctx, req, resp)
    });
    builder.build()
}
