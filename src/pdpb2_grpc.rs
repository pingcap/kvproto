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
    fn GetMembers(&self, p: super::pdpb2::GetMembersRequest) -> ::grpc::result::GrpcResult<super::pdpb2::GetMembersResponse>;

    fn Tso(&self, p: ::grpc::iter::GrpcIterator<super::pdpb2::TsoRequest>) -> ::grpc::iter::GrpcIterator<super::pdpb2::TsoResponse>;

    fn Bootstrap(&self, p: super::pdpb2::BootstrapRequest) -> ::grpc::result::GrpcResult<super::pdpb2::BootstrapResponse>;

    fn IsBootstrapped(&self, p: super::pdpb2::IsBootstrappedRequest) -> ::grpc::result::GrpcResult<super::pdpb2::IsBootstrappedResponse>;

    fn AllocID(&self, p: super::pdpb2::AllocIDRequest) -> ::grpc::result::GrpcResult<super::pdpb2::AllocIDResponse>;

    fn GetStore(&self, p: super::pdpb2::GetStoreRequest) -> ::grpc::result::GrpcResult<super::pdpb2::GetStoreResponse>;

    fn PutStore(&self, p: super::pdpb2::PutStoreRequest) -> ::grpc::result::GrpcResult<super::pdpb2::PutStoreResponse>;

    fn StoreHeartbeat(&self, p: super::pdpb2::StoreHeartbeatRequest) -> ::grpc::result::GrpcResult<super::pdpb2::StoreHeartbeatResponse>;

    fn RegionHeartbeat(&self, p: super::pdpb2::RegionHeartbeatRequest) -> ::grpc::result::GrpcResult<super::pdpb2::RegionHeartbeatResponse>;

    fn GetRegion(&self, p: super::pdpb2::GetRegionRequest) -> ::grpc::result::GrpcResult<super::pdpb2::GetRegionResponse>;

    fn GetRegionByID(&self, p: super::pdpb2::GetRegionByIDRequest) -> ::grpc::result::GrpcResult<super::pdpb2::GetRegionResponse>;

    fn AskSplit(&self, p: super::pdpb2::AskSplitRequest) -> ::grpc::result::GrpcResult<super::pdpb2::AskSplitResponse>;

    fn ReportSplit(&self, p: super::pdpb2::ReportSplitRequest) -> ::grpc::result::GrpcResult<super::pdpb2::ReportSplitResponse>;

    fn GetClusterConfig(&self, p: super::pdpb2::GetClusterConfigRequest) -> ::grpc::result::GrpcResult<super::pdpb2::GetClusterConfigResponse>;

    fn PutClusterConfig(&self, p: super::pdpb2::PutClusterConfigRequest) -> ::grpc::result::GrpcResult<super::pdpb2::PutClusterConfigResponse>;
}

pub trait PDAsync {
    fn GetMembers(&self, p: super::pdpb2::GetMembersRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb2::GetMembersResponse>;

    fn Tso(&self, p: ::grpc::futures_grpc::GrpcStreamSend<super::pdpb2::TsoRequest>) -> ::grpc::futures_grpc::GrpcStreamSend<super::pdpb2::TsoResponse>;

    fn Bootstrap(&self, p: super::pdpb2::BootstrapRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb2::BootstrapResponse>;

    fn IsBootstrapped(&self, p: super::pdpb2::IsBootstrappedRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb2::IsBootstrappedResponse>;

    fn AllocID(&self, p: super::pdpb2::AllocIDRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb2::AllocIDResponse>;

    fn GetStore(&self, p: super::pdpb2::GetStoreRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb2::GetStoreResponse>;

    fn PutStore(&self, p: super::pdpb2::PutStoreRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb2::PutStoreResponse>;

    fn StoreHeartbeat(&self, p: super::pdpb2::StoreHeartbeatRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb2::StoreHeartbeatResponse>;

    fn RegionHeartbeat(&self, p: super::pdpb2::RegionHeartbeatRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb2::RegionHeartbeatResponse>;

    fn GetRegion(&self, p: super::pdpb2::GetRegionRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb2::GetRegionResponse>;

    fn GetRegionByID(&self, p: super::pdpb2::GetRegionByIDRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb2::GetRegionResponse>;

    fn AskSplit(&self, p: super::pdpb2::AskSplitRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb2::AskSplitResponse>;

    fn ReportSplit(&self, p: super::pdpb2::ReportSplitRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb2::ReportSplitResponse>;

    fn GetClusterConfig(&self, p: super::pdpb2::GetClusterConfigRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb2::GetClusterConfigResponse>;

    fn PutClusterConfig(&self, p: super::pdpb2::PutClusterConfigRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb2::PutClusterConfigResponse>;
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
    fn GetMembers(&self, p: super::pdpb2::GetMembersRequest) -> ::grpc::result::GrpcResult<super::pdpb2::GetMembersResponse> {
        ::futures::Future::wait(self.async_client.GetMembers(p))
    }

    fn Tso(&self, p: ::grpc::iter::GrpcIterator<super::pdpb2::TsoRequest>) -> ::grpc::iter::GrpcIterator<super::pdpb2::TsoResponse> {
        let p = ::futures::stream::Stream::boxed(::futures::stream::iter(::std::iter::IntoIterator::into_iter(p)));
        ::grpc::rt::stream_to_iter(self.async_client.Tso(p))
    }

    fn Bootstrap(&self, p: super::pdpb2::BootstrapRequest) -> ::grpc::result::GrpcResult<super::pdpb2::BootstrapResponse> {
        ::futures::Future::wait(self.async_client.Bootstrap(p))
    }

    fn IsBootstrapped(&self, p: super::pdpb2::IsBootstrappedRequest) -> ::grpc::result::GrpcResult<super::pdpb2::IsBootstrappedResponse> {
        ::futures::Future::wait(self.async_client.IsBootstrapped(p))
    }

    fn AllocID(&self, p: super::pdpb2::AllocIDRequest) -> ::grpc::result::GrpcResult<super::pdpb2::AllocIDResponse> {
        ::futures::Future::wait(self.async_client.AllocID(p))
    }

    fn GetStore(&self, p: super::pdpb2::GetStoreRequest) -> ::grpc::result::GrpcResult<super::pdpb2::GetStoreResponse> {
        ::futures::Future::wait(self.async_client.GetStore(p))
    }

    fn PutStore(&self, p: super::pdpb2::PutStoreRequest) -> ::grpc::result::GrpcResult<super::pdpb2::PutStoreResponse> {
        ::futures::Future::wait(self.async_client.PutStore(p))
    }

    fn StoreHeartbeat(&self, p: super::pdpb2::StoreHeartbeatRequest) -> ::grpc::result::GrpcResult<super::pdpb2::StoreHeartbeatResponse> {
        ::futures::Future::wait(self.async_client.StoreHeartbeat(p))
    }

    fn RegionHeartbeat(&self, p: super::pdpb2::RegionHeartbeatRequest) -> ::grpc::result::GrpcResult<super::pdpb2::RegionHeartbeatResponse> {
        ::futures::Future::wait(self.async_client.RegionHeartbeat(p))
    }

    fn GetRegion(&self, p: super::pdpb2::GetRegionRequest) -> ::grpc::result::GrpcResult<super::pdpb2::GetRegionResponse> {
        ::futures::Future::wait(self.async_client.GetRegion(p))
    }

    fn GetRegionByID(&self, p: super::pdpb2::GetRegionByIDRequest) -> ::grpc::result::GrpcResult<super::pdpb2::GetRegionResponse> {
        ::futures::Future::wait(self.async_client.GetRegionByID(p))
    }

    fn AskSplit(&self, p: super::pdpb2::AskSplitRequest) -> ::grpc::result::GrpcResult<super::pdpb2::AskSplitResponse> {
        ::futures::Future::wait(self.async_client.AskSplit(p))
    }

    fn ReportSplit(&self, p: super::pdpb2::ReportSplitRequest) -> ::grpc::result::GrpcResult<super::pdpb2::ReportSplitResponse> {
        ::futures::Future::wait(self.async_client.ReportSplit(p))
    }

    fn GetClusterConfig(&self, p: super::pdpb2::GetClusterConfigRequest) -> ::grpc::result::GrpcResult<super::pdpb2::GetClusterConfigResponse> {
        ::futures::Future::wait(self.async_client.GetClusterConfig(p))
    }

    fn PutClusterConfig(&self, p: super::pdpb2::PutClusterConfigRequest) -> ::grpc::result::GrpcResult<super::pdpb2::PutClusterConfigResponse> {
        ::futures::Future::wait(self.async_client.PutClusterConfig(p))
    }
}

// async client

pub struct PDAsyncClient {
    grpc_client: ::grpc::client::GrpcClient,
    method_GetMembers: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::pdpb2::GetMembersRequest, super::pdpb2::GetMembersResponse>>,
    method_Tso: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::pdpb2::TsoRequest, super::pdpb2::TsoResponse>>,
    method_Bootstrap: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::pdpb2::BootstrapRequest, super::pdpb2::BootstrapResponse>>,
    method_IsBootstrapped: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::pdpb2::IsBootstrappedRequest, super::pdpb2::IsBootstrappedResponse>>,
    method_AllocID: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::pdpb2::AllocIDRequest, super::pdpb2::AllocIDResponse>>,
    method_GetStore: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::pdpb2::GetStoreRequest, super::pdpb2::GetStoreResponse>>,
    method_PutStore: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::pdpb2::PutStoreRequest, super::pdpb2::PutStoreResponse>>,
    method_StoreHeartbeat: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::pdpb2::StoreHeartbeatRequest, super::pdpb2::StoreHeartbeatResponse>>,
    method_RegionHeartbeat: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::pdpb2::RegionHeartbeatRequest, super::pdpb2::RegionHeartbeatResponse>>,
    method_GetRegion: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::pdpb2::GetRegionRequest, super::pdpb2::GetRegionResponse>>,
    method_GetRegionByID: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::pdpb2::GetRegionByIDRequest, super::pdpb2::GetRegionResponse>>,
    method_AskSplit: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::pdpb2::AskSplitRequest, super::pdpb2::AskSplitResponse>>,
    method_ReportSplit: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::pdpb2::ReportSplitRequest, super::pdpb2::ReportSplitResponse>>,
    method_GetClusterConfig: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::pdpb2::GetClusterConfigRequest, super::pdpb2::GetClusterConfigResponse>>,
    method_PutClusterConfig: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::pdpb2::PutClusterConfigRequest, super::pdpb2::PutClusterConfigResponse>>,
}

impl PDAsyncClient {
    pub fn new(host: &str, port: u16, tls: bool, conf: ::grpc::client::GrpcClientConf) -> ::grpc::result::GrpcResult<Self> {
        ::grpc::client::GrpcClient::new(host, port, tls, conf).map(|c| {
            PDAsyncClient {
                grpc_client: c,
                method_GetMembers: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/pdpb2.PD/GetMembers".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_Tso: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/pdpb2.PD/Tso".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Bidi,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_Bootstrap: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/pdpb2.PD/Bootstrap".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_IsBootstrapped: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/pdpb2.PD/IsBootstrapped".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_AllocID: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/pdpb2.PD/AllocID".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_GetStore: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/pdpb2.PD/GetStore".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_PutStore: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/pdpb2.PD/PutStore".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_StoreHeartbeat: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/pdpb2.PD/StoreHeartbeat".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_RegionHeartbeat: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/pdpb2.PD/RegionHeartbeat".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_GetRegion: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/pdpb2.PD/GetRegion".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_GetRegionByID: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/pdpb2.PD/GetRegionByID".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_AskSplit: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/pdpb2.PD/AskSplit".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_ReportSplit: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/pdpb2.PD/ReportSplit".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_GetClusterConfig: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/pdpb2.PD/GetClusterConfig".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_PutClusterConfig: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/pdpb2.PD/PutClusterConfig".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
            }
        })
    }
}

impl PDAsync for PDAsyncClient {
    fn GetMembers(&self, p: super::pdpb2::GetMembersRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb2::GetMembersResponse> {
        self.grpc_client.call_unary(p, self.method_GetMembers.clone())
    }

    fn Tso(&self, p: ::grpc::futures_grpc::GrpcStreamSend<super::pdpb2::TsoRequest>) -> ::grpc::futures_grpc::GrpcStreamSend<super::pdpb2::TsoResponse> {
        self.grpc_client.call_bidi(p, self.method_Tso.clone())
    }

    fn Bootstrap(&self, p: super::pdpb2::BootstrapRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb2::BootstrapResponse> {
        self.grpc_client.call_unary(p, self.method_Bootstrap.clone())
    }

    fn IsBootstrapped(&self, p: super::pdpb2::IsBootstrappedRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb2::IsBootstrappedResponse> {
        self.grpc_client.call_unary(p, self.method_IsBootstrapped.clone())
    }

    fn AllocID(&self, p: super::pdpb2::AllocIDRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb2::AllocIDResponse> {
        self.grpc_client.call_unary(p, self.method_AllocID.clone())
    }

    fn GetStore(&self, p: super::pdpb2::GetStoreRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb2::GetStoreResponse> {
        self.grpc_client.call_unary(p, self.method_GetStore.clone())
    }

    fn PutStore(&self, p: super::pdpb2::PutStoreRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb2::PutStoreResponse> {
        self.grpc_client.call_unary(p, self.method_PutStore.clone())
    }

    fn StoreHeartbeat(&self, p: super::pdpb2::StoreHeartbeatRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb2::StoreHeartbeatResponse> {
        self.grpc_client.call_unary(p, self.method_StoreHeartbeat.clone())
    }

    fn RegionHeartbeat(&self, p: super::pdpb2::RegionHeartbeatRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb2::RegionHeartbeatResponse> {
        self.grpc_client.call_unary(p, self.method_RegionHeartbeat.clone())
    }

    fn GetRegion(&self, p: super::pdpb2::GetRegionRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb2::GetRegionResponse> {
        self.grpc_client.call_unary(p, self.method_GetRegion.clone())
    }

    fn GetRegionByID(&self, p: super::pdpb2::GetRegionByIDRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb2::GetRegionResponse> {
        self.grpc_client.call_unary(p, self.method_GetRegionByID.clone())
    }

    fn AskSplit(&self, p: super::pdpb2::AskSplitRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb2::AskSplitResponse> {
        self.grpc_client.call_unary(p, self.method_AskSplit.clone())
    }

    fn ReportSplit(&self, p: super::pdpb2::ReportSplitRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb2::ReportSplitResponse> {
        self.grpc_client.call_unary(p, self.method_ReportSplit.clone())
    }

    fn GetClusterConfig(&self, p: super::pdpb2::GetClusterConfigRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb2::GetClusterConfigResponse> {
        self.grpc_client.call_unary(p, self.method_GetClusterConfig.clone())
    }

    fn PutClusterConfig(&self, p: super::pdpb2::PutClusterConfigRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb2::PutClusterConfigResponse> {
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
    fn GetMembers(&self, p: super::pdpb2::GetMembersRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb2::GetMembersResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.GetMembers(p)
        })
    }

    fn Tso(&self, p: ::grpc::futures_grpc::GrpcStreamSend<super::pdpb2::TsoRequest>) -> ::grpc::futures_grpc::GrpcStreamSend<super::pdpb2::TsoResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_bidi(&self.cpupool, p, move |p| {
            h.Tso(p)
        })
    }

    fn Bootstrap(&self, p: super::pdpb2::BootstrapRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb2::BootstrapResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.Bootstrap(p)
        })
    }

    fn IsBootstrapped(&self, p: super::pdpb2::IsBootstrappedRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb2::IsBootstrappedResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.IsBootstrapped(p)
        })
    }

    fn AllocID(&self, p: super::pdpb2::AllocIDRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb2::AllocIDResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.AllocID(p)
        })
    }

    fn GetStore(&self, p: super::pdpb2::GetStoreRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb2::GetStoreResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.GetStore(p)
        })
    }

    fn PutStore(&self, p: super::pdpb2::PutStoreRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb2::PutStoreResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.PutStore(p)
        })
    }

    fn StoreHeartbeat(&self, p: super::pdpb2::StoreHeartbeatRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb2::StoreHeartbeatResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.StoreHeartbeat(p)
        })
    }

    fn RegionHeartbeat(&self, p: super::pdpb2::RegionHeartbeatRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb2::RegionHeartbeatResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.RegionHeartbeat(p)
        })
    }

    fn GetRegion(&self, p: super::pdpb2::GetRegionRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb2::GetRegionResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.GetRegion(p)
        })
    }

    fn GetRegionByID(&self, p: super::pdpb2::GetRegionByIDRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb2::GetRegionResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.GetRegionByID(p)
        })
    }

    fn AskSplit(&self, p: super::pdpb2::AskSplitRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb2::AskSplitResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.AskSplit(p)
        })
    }

    fn ReportSplit(&self, p: super::pdpb2::ReportSplitRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb2::ReportSplitResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.ReportSplit(p)
        })
    }

    fn GetClusterConfig(&self, p: super::pdpb2::GetClusterConfigRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb2::GetClusterConfigResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.GetClusterConfig(p)
        })
    }

    fn PutClusterConfig(&self, p: super::pdpb2::PutClusterConfigRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::pdpb2::PutClusterConfigResponse> {
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
                        name: "/pdpb2.PD/GetMembers".to_string(),
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
                        name: "/pdpb2.PD/Tso".to_string(),
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
                        name: "/pdpb2.PD/Bootstrap".to_string(),
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
                        name: "/pdpb2.PD/IsBootstrapped".to_string(),
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
                        name: "/pdpb2.PD/AllocID".to_string(),
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
                        name: "/pdpb2.PD/GetStore".to_string(),
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
                        name: "/pdpb2.PD/PutStore".to_string(),
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
                        name: "/pdpb2.PD/StoreHeartbeat".to_string(),
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
                        name: "/pdpb2.PD/RegionHeartbeat".to_string(),
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
                        name: "/pdpb2.PD/GetRegion".to_string(),
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
                        name: "/pdpb2.PD/GetRegionByID".to_string(),
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
                        name: "/pdpb2.PD/AskSplit".to_string(),
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
                        name: "/pdpb2.PD/ReportSplit".to_string(),
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
                        name: "/pdpb2.PD/GetClusterConfig".to_string(),
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
                        name: "/pdpb2.PD/PutClusterConfig".to_string(),
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
