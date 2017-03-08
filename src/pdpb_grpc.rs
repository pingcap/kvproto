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


// interface

pub trait PD {
    fn GetMembers(&self, p: super::pdpb::GetMembersRequest) -> ::grpc::result::GrpcResult<super::pdpb::GetMembersResponse>;

    fn Tso(&self, p: ::grpc::iter::GrpcIterator<super::pdpb::TsoRequest>) -> ::grpc::iter::GrpcIterator<super::pdpb::TsoResponse>;

    fn Bootstrap(&self, p: super::pdpb::BootstrapRequest) -> ::grpc::result::GrpcResult<super::pdpb::BootstrapResponse>;

    fn IsBootstrapped(&self, p: super::pdpb::IsBootstrappedRequest) -> ::grpc::result::GrpcResult<super::pdpb::IsBootstrappedResponse>;

    fn AllocID(&self, p: super::pdpb::AllocIDRequest) -> ::grpc::result::GrpcResult<super::pdpb::AllocIDResponse>;

    fn GetStore(&self, p: super::pdpb::GetStoreRequest) -> ::grpc::result::GrpcResult<super::pdpb::GetStoreResponse>;

    fn PutStore(&self, p: super::pdpb::PutStoreRequest) -> ::grpc::result::GrpcResult<super::pdpb::PutStoreResponse>;

    fn StoreHeartbeat(&self, p: super::pdpb::StoreHeartbeatRequest) -> ::grpc::result::GrpcResult<super::pdpb::StoreHeartbeatResponse>;

    fn RegionHeartbeat(&self, p: super::pdpb::RegionHeartbeatRequest) -> ::grpc::result::GrpcResult<super::pdpb::RegionHeartbeatResponse>;

    fn GetRegion(&self, p: super::pdpb::GetRegionRequest) -> ::grpc::result::GrpcResult<super::pdpb::GetRegionResponse>;

    fn GetRegionByID(&self, p: super::pdpb::GetRegionByIDRequest) -> ::grpc::result::GrpcResult<super::pdpb::GetRegionResponse>;

    fn AskSplit(&self, p: super::pdpb::AskSplitRequest) -> ::grpc::result::GrpcResult<super::pdpb::AskSplitResponse>;

    fn ReportSplit(&self, p: super::pdpb::ReportSplitRequest) -> ::grpc::result::GrpcResult<super::pdpb::ReportSplitResponse>;

    fn GetClusterConfig(&self, p: super::pdpb::GetClusterConfigRequest) -> ::grpc::result::GrpcResult<super::pdpb::GetClusterConfigResponse>;

    fn PutClusterConfig(&self, p: super::pdpb::PutClusterConfigRequest) -> ::grpc::result::GrpcResult<super::pdpb::PutClusterConfigResponse>;
}

pub trait PDAsync {
    fn GetMembers(&self, p: super::pdpb::GetMembersRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb::GetMembersResponse>;

    fn Tso(&self, p: ::grpc::futures_grpc::GrpcStreamSend<super::pdpb::TsoRequest>) -> ::grpc::futures_grpc::GrpcStreamSend<super::pdpb::TsoResponse>;

    fn Bootstrap(&self, p: super::pdpb::BootstrapRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb::BootstrapResponse>;

    fn IsBootstrapped(&self, p: super::pdpb::IsBootstrappedRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb::IsBootstrappedResponse>;

    fn AllocID(&self, p: super::pdpb::AllocIDRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb::AllocIDResponse>;

    fn GetStore(&self, p: super::pdpb::GetStoreRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb::GetStoreResponse>;

    fn PutStore(&self, p: super::pdpb::PutStoreRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb::PutStoreResponse>;

    fn StoreHeartbeat(&self, p: super::pdpb::StoreHeartbeatRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb::StoreHeartbeatResponse>;

    fn RegionHeartbeat(&self, p: super::pdpb::RegionHeartbeatRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb::RegionHeartbeatResponse>;

    fn GetRegion(&self, p: super::pdpb::GetRegionRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb::GetRegionResponse>;

    fn GetRegionByID(&self, p: super::pdpb::GetRegionByIDRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb::GetRegionResponse>;

    fn AskSplit(&self, p: super::pdpb::AskSplitRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb::AskSplitResponse>;

    fn ReportSplit(&self, p: super::pdpb::ReportSplitRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb::ReportSplitResponse>;

    fn GetClusterConfig(&self, p: super::pdpb::GetClusterConfigRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb::GetClusterConfigResponse>;

    fn PutClusterConfig(&self, p: super::pdpb::PutClusterConfigRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb::PutClusterConfigResponse>;
}

// sync client

pub struct PDClient {
    async_client: PDAsyncClient,
}

impl PDClient {
    pub fn new(host: &str, port: u16, tls: bool, conf: ::grpc::client::GrpcClientConf) -> ::grpc::result::GrpcResult<Self> {
        PDAsyncClient::new(host, port, tls, conf).map(|c| {
            PDClient {
                async_client: c,
            }
        })
    }
}

impl PD for PDClient {
    fn GetMembers(&self, p: super::pdpb::GetMembersRequest) -> ::grpc::result::GrpcResult<super::pdpb::GetMembersResponse> {
        ::futures::Future::wait(self.async_client.GetMembers(p))
    }

    fn Tso(&self, p: ::grpc::iter::GrpcIterator<super::pdpb::TsoRequest>) -> ::grpc::iter::GrpcIterator<super::pdpb::TsoResponse> {
        let p = ::futures::stream::Stream::boxed(::futures::stream::iter(::std::iter::IntoIterator::into_iter(p)));
        ::grpc::rt::stream_to_iter(self.async_client.Tso(p))
    }

    fn Bootstrap(&self, p: super::pdpb::BootstrapRequest) -> ::grpc::result::GrpcResult<super::pdpb::BootstrapResponse> {
        ::futures::Future::wait(self.async_client.Bootstrap(p))
    }

    fn IsBootstrapped(&self, p: super::pdpb::IsBootstrappedRequest) -> ::grpc::result::GrpcResult<super::pdpb::IsBootstrappedResponse> {
        ::futures::Future::wait(self.async_client.IsBootstrapped(p))
    }

    fn AllocID(&self, p: super::pdpb::AllocIDRequest) -> ::grpc::result::GrpcResult<super::pdpb::AllocIDResponse> {
        ::futures::Future::wait(self.async_client.AllocID(p))
    }

    fn GetStore(&self, p: super::pdpb::GetStoreRequest) -> ::grpc::result::GrpcResult<super::pdpb::GetStoreResponse> {
        ::futures::Future::wait(self.async_client.GetStore(p))
    }

    fn PutStore(&self, p: super::pdpb::PutStoreRequest) -> ::grpc::result::GrpcResult<super::pdpb::PutStoreResponse> {
        ::futures::Future::wait(self.async_client.PutStore(p))
    }

    fn StoreHeartbeat(&self, p: super::pdpb::StoreHeartbeatRequest) -> ::grpc::result::GrpcResult<super::pdpb::StoreHeartbeatResponse> {
        ::futures::Future::wait(self.async_client.StoreHeartbeat(p))
    }

    fn RegionHeartbeat(&self, p: super::pdpb::RegionHeartbeatRequest) -> ::grpc::result::GrpcResult<super::pdpb::RegionHeartbeatResponse> {
        ::futures::Future::wait(self.async_client.RegionHeartbeat(p))
    }

    fn GetRegion(&self, p: super::pdpb::GetRegionRequest) -> ::grpc::result::GrpcResult<super::pdpb::GetRegionResponse> {
        ::futures::Future::wait(self.async_client.GetRegion(p))
    }

    fn GetRegionByID(&self, p: super::pdpb::GetRegionByIDRequest) -> ::grpc::result::GrpcResult<super::pdpb::GetRegionResponse> {
        ::futures::Future::wait(self.async_client.GetRegionByID(p))
    }

    fn AskSplit(&self, p: super::pdpb::AskSplitRequest) -> ::grpc::result::GrpcResult<super::pdpb::AskSplitResponse> {
        ::futures::Future::wait(self.async_client.AskSplit(p))
    }

    fn ReportSplit(&self, p: super::pdpb::ReportSplitRequest) -> ::grpc::result::GrpcResult<super::pdpb::ReportSplitResponse> {
        ::futures::Future::wait(self.async_client.ReportSplit(p))
    }

    fn GetClusterConfig(&self, p: super::pdpb::GetClusterConfigRequest) -> ::grpc::result::GrpcResult<super::pdpb::GetClusterConfigResponse> {
        ::futures::Future::wait(self.async_client.GetClusterConfig(p))
    }

    fn PutClusterConfig(&self, p: super::pdpb::PutClusterConfigRequest) -> ::grpc::result::GrpcResult<super::pdpb::PutClusterConfigResponse> {
        ::futures::Future::wait(self.async_client.PutClusterConfig(p))
    }
}

// async client

pub struct PDAsyncClient {
    grpc_client: ::grpc::client::GrpcClient,
    method_GetMembers: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::pdpb::GetMembersRequest, super::pdpb::GetMembersResponse>>,
    method_Tso: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::pdpb::TsoRequest, super::pdpb::TsoResponse>>,
    method_Bootstrap: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::pdpb::BootstrapRequest, super::pdpb::BootstrapResponse>>,
    method_IsBootstrapped: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::pdpb::IsBootstrappedRequest, super::pdpb::IsBootstrappedResponse>>,
    method_AllocID: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::pdpb::AllocIDRequest, super::pdpb::AllocIDResponse>>,
    method_GetStore: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::pdpb::GetStoreRequest, super::pdpb::GetStoreResponse>>,
    method_PutStore: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::pdpb::PutStoreRequest, super::pdpb::PutStoreResponse>>,
    method_StoreHeartbeat: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::pdpb::StoreHeartbeatRequest, super::pdpb::StoreHeartbeatResponse>>,
    method_RegionHeartbeat: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::pdpb::RegionHeartbeatRequest, super::pdpb::RegionHeartbeatResponse>>,
    method_GetRegion: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::pdpb::GetRegionRequest, super::pdpb::GetRegionResponse>>,
    method_GetRegionByID: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::pdpb::GetRegionByIDRequest, super::pdpb::GetRegionResponse>>,
    method_AskSplit: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::pdpb::AskSplitRequest, super::pdpb::AskSplitResponse>>,
    method_ReportSplit: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::pdpb::ReportSplitRequest, super::pdpb::ReportSplitResponse>>,
    method_GetClusterConfig: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::pdpb::GetClusterConfigRequest, super::pdpb::GetClusterConfigResponse>>,
    method_PutClusterConfig: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::pdpb::PutClusterConfigRequest, super::pdpb::PutClusterConfigResponse>>,
}

impl PDAsyncClient {
    pub fn new(host: &str, port: u16, tls: bool, conf: ::grpc::client::GrpcClientConf) -> ::grpc::result::GrpcResult<Self> {
        ::grpc::client::GrpcClient::new(host, port, tls, conf).map(|c| {
            PDAsyncClient {
                grpc_client: c,
                method_GetMembers: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/pdpb.PD/GetMembers".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_Tso: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/pdpb.PD/Tso".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Bidi,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_Bootstrap: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/pdpb.PD/Bootstrap".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_IsBootstrapped: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/pdpb.PD/IsBootstrapped".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_AllocID: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/pdpb.PD/AllocID".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_GetStore: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/pdpb.PD/GetStore".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_PutStore: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/pdpb.PD/PutStore".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_StoreHeartbeat: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/pdpb.PD/StoreHeartbeat".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_RegionHeartbeat: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/pdpb.PD/RegionHeartbeat".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_GetRegion: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/pdpb.PD/GetRegion".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_GetRegionByID: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/pdpb.PD/GetRegionByID".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_AskSplit: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/pdpb.PD/AskSplit".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_ReportSplit: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/pdpb.PD/ReportSplit".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_GetClusterConfig: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/pdpb.PD/GetClusterConfig".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_PutClusterConfig: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/pdpb.PD/PutClusterConfig".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
            }
        })
    }
}

impl PDAsync for PDAsyncClient {
    fn GetMembers(&self, p: super::pdpb::GetMembersRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb::GetMembersResponse> {
        self.grpc_client.call_unary(p, self.method_GetMembers.clone())
    }

    fn Tso(&self, p: ::grpc::futures_grpc::GrpcStreamSend<super::pdpb::TsoRequest>) -> ::grpc::futures_grpc::GrpcStreamSend<super::pdpb::TsoResponse> {
        self.grpc_client.call_bidi(p, self.method_Tso.clone())
    }

    fn Bootstrap(&self, p: super::pdpb::BootstrapRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb::BootstrapResponse> {
        self.grpc_client.call_unary(p, self.method_Bootstrap.clone())
    }

    fn IsBootstrapped(&self, p: super::pdpb::IsBootstrappedRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb::IsBootstrappedResponse> {
        self.grpc_client.call_unary(p, self.method_IsBootstrapped.clone())
    }

    fn AllocID(&self, p: super::pdpb::AllocIDRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb::AllocIDResponse> {
        self.grpc_client.call_unary(p, self.method_AllocID.clone())
    }

    fn GetStore(&self, p: super::pdpb::GetStoreRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb::GetStoreResponse> {
        self.grpc_client.call_unary(p, self.method_GetStore.clone())
    }

    fn PutStore(&self, p: super::pdpb::PutStoreRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb::PutStoreResponse> {
        self.grpc_client.call_unary(p, self.method_PutStore.clone())
    }

    fn StoreHeartbeat(&self, p: super::pdpb::StoreHeartbeatRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb::StoreHeartbeatResponse> {
        self.grpc_client.call_unary(p, self.method_StoreHeartbeat.clone())
    }

    fn RegionHeartbeat(&self, p: super::pdpb::RegionHeartbeatRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb::RegionHeartbeatResponse> {
        self.grpc_client.call_unary(p, self.method_RegionHeartbeat.clone())
    }

    fn GetRegion(&self, p: super::pdpb::GetRegionRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb::GetRegionResponse> {
        self.grpc_client.call_unary(p, self.method_GetRegion.clone())
    }

    fn GetRegionByID(&self, p: super::pdpb::GetRegionByIDRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb::GetRegionResponse> {
        self.grpc_client.call_unary(p, self.method_GetRegionByID.clone())
    }

    fn AskSplit(&self, p: super::pdpb::AskSplitRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb::AskSplitResponse> {
        self.grpc_client.call_unary(p, self.method_AskSplit.clone())
    }

    fn ReportSplit(&self, p: super::pdpb::ReportSplitRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb::ReportSplitResponse> {
        self.grpc_client.call_unary(p, self.method_ReportSplit.clone())
    }

    fn GetClusterConfig(&self, p: super::pdpb::GetClusterConfigRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb::GetClusterConfigResponse> {
        self.grpc_client.call_unary(p, self.method_GetClusterConfig.clone())
    }

    fn PutClusterConfig(&self, p: super::pdpb::PutClusterConfigRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb::PutClusterConfigResponse> {
        self.grpc_client.call_unary(p, self.method_PutClusterConfig.clone())
    }
}

// sync server

pub struct PDServer {
    async_server: PDAsyncServer,
}

struct PDServerHandlerToAsync {
    handler: ::std::sync::Arc<PD + Send + Sync>,
    cpupool: ::futures_cpupool::CpuPool,
}

impl PDAsync for PDServerHandlerToAsync {
    fn GetMembers(&self, p: super::pdpb::GetMembersRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb::GetMembersResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.GetMembers(p)
        })
    }

    fn Tso(&self, p: ::grpc::futures_grpc::GrpcStreamSend<super::pdpb::TsoRequest>) -> ::grpc::futures_grpc::GrpcStreamSend<super::pdpb::TsoResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_bidi(&self.cpupool, p, move |p| {
            h.Tso(p)
        })
    }

    fn Bootstrap(&self, p: super::pdpb::BootstrapRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb::BootstrapResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.Bootstrap(p)
        })
    }

    fn IsBootstrapped(&self, p: super::pdpb::IsBootstrappedRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb::IsBootstrappedResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.IsBootstrapped(p)
        })
    }

    fn AllocID(&self, p: super::pdpb::AllocIDRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb::AllocIDResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.AllocID(p)
        })
    }

    fn GetStore(&self, p: super::pdpb::GetStoreRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb::GetStoreResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.GetStore(p)
        })
    }

    fn PutStore(&self, p: super::pdpb::PutStoreRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb::PutStoreResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.PutStore(p)
        })
    }

    fn StoreHeartbeat(&self, p: super::pdpb::StoreHeartbeatRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb::StoreHeartbeatResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.StoreHeartbeat(p)
        })
    }

    fn RegionHeartbeat(&self, p: super::pdpb::RegionHeartbeatRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb::RegionHeartbeatResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.RegionHeartbeat(p)
        })
    }

    fn GetRegion(&self, p: super::pdpb::GetRegionRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb::GetRegionResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.GetRegion(p)
        })
    }

    fn GetRegionByID(&self, p: super::pdpb::GetRegionByIDRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb::GetRegionResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.GetRegionByID(p)
        })
    }

    fn AskSplit(&self, p: super::pdpb::AskSplitRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb::AskSplitResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.AskSplit(p)
        })
    }

    fn ReportSplit(&self, p: super::pdpb::ReportSplitRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb::ReportSplitResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.ReportSplit(p)
        })
    }

    fn GetClusterConfig(&self, p: super::pdpb::GetClusterConfigRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb::GetClusterConfigResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.GetClusterConfig(p)
        })
    }

    fn PutClusterConfig(&self, p: super::pdpb::PutClusterConfigRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb::PutClusterConfigResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.PutClusterConfig(p)
        })
    }
}

impl PDServer {
    pub fn new<A : ::std::net::ToSocketAddrs, H : PD + Send + Sync + 'static>(addr: A, conf: ::grpc::server::GrpcServerConf, h: H) -> Self {
        let h = PDServerHandlerToAsync {
            cpupool: ::futures_cpupool::CpuPool::new_num_cpus(),
            handler: ::std::sync::Arc::new(h),
        };
        PDServer {
            async_server: PDAsyncServer::new(addr, conf, h),
        }
    }
}

// async server

pub struct PDAsyncServer {
    grpc_server: ::grpc::server::GrpcServer,
}

impl PDAsyncServer {
    pub fn new<A : ::std::net::ToSocketAddrs, H : PDAsync + 'static + Sync + Send + 'static>(addr: A, conf: ::grpc::server::GrpcServerConf, h: H) -> Self {
        let service_definition = PDAsyncServer::new_service_def(h);
        PDAsyncServer {
            grpc_server: ::grpc::server::GrpcServer::new(addr, conf, service_definition),
        }
    }

    pub fn new_service_def<H : PDAsync + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::server::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::server::ServerServiceDefinition::new(
            vec![
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/pdpb.PD/GetMembers".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.GetMembers(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/pdpb.PD/Tso".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Bidi,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerBidi::new(move |p| handler_copy.Tso(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/pdpb.PD/Bootstrap".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.Bootstrap(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/pdpb.PD/IsBootstrapped".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.IsBootstrapped(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/pdpb.PD/AllocID".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.AllocID(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/pdpb.PD/GetStore".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.GetStore(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/pdpb.PD/PutStore".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.PutStore(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/pdpb.PD/StoreHeartbeat".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.StoreHeartbeat(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/pdpb.PD/RegionHeartbeat".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.RegionHeartbeat(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/pdpb.PD/GetRegion".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.GetRegion(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/pdpb.PD/GetRegionByID".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.GetRegionByID(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/pdpb.PD/AskSplit".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.AskSplit(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/pdpb.PD/ReportSplit".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.ReportSplit(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/pdpb.PD/GetClusterConfig".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.GetClusterConfig(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/pdpb.PD/PutClusterConfig".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.PutClusterConfig(p))
                    },
                ),
            ],
        )
    }
}
