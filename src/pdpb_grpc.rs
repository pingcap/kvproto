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
    fn GetMembers(&self, o: ::grpc::GrpcRequestOptions, p: super::pdpb::GetMembersRequest) -> ::grpc::GrpcSingleResponse<super::pdpb::GetMembersResponse>;

    fn Tso(&self, o: ::grpc::GrpcRequestOptions, p: ::grpc::GrpcStreamingRequest<super::pdpb::TsoRequest>) -> ::grpc::GrpcStreamingResponse<super::pdpb::TsoResponse>;

    fn Bootstrap(&self, o: ::grpc::GrpcRequestOptions, p: super::pdpb::BootstrapRequest) -> ::grpc::GrpcSingleResponse<super::pdpb::BootstrapResponse>;

    fn IsBootstrapped(&self, o: ::grpc::GrpcRequestOptions, p: super::pdpb::IsBootstrappedRequest) -> ::grpc::GrpcSingleResponse<super::pdpb::IsBootstrappedResponse>;

    fn AllocID(&self, o: ::grpc::GrpcRequestOptions, p: super::pdpb::AllocIDRequest) -> ::grpc::GrpcSingleResponse<super::pdpb::AllocIDResponse>;

    fn GetStore(&self, o: ::grpc::GrpcRequestOptions, p: super::pdpb::GetStoreRequest) -> ::grpc::GrpcSingleResponse<super::pdpb::GetStoreResponse>;

    fn PutStore(&self, o: ::grpc::GrpcRequestOptions, p: super::pdpb::PutStoreRequest) -> ::grpc::GrpcSingleResponse<super::pdpb::PutStoreResponse>;

    fn StoreHeartbeat(&self, o: ::grpc::GrpcRequestOptions, p: super::pdpb::StoreHeartbeatRequest) -> ::grpc::GrpcSingleResponse<super::pdpb::StoreHeartbeatResponse>;

    fn RegionHeartbeat(&self, o: ::grpc::GrpcRequestOptions, p: super::pdpb::RegionHeartbeatRequest) -> ::grpc::GrpcSingleResponse<super::pdpb::RegionHeartbeatResponse>;

    fn GetRegion(&self, o: ::grpc::GrpcRequestOptions, p: super::pdpb::GetRegionRequest) -> ::grpc::GrpcSingleResponse<super::pdpb::GetRegionResponse>;

    fn GetRegionByID(&self, o: ::grpc::GrpcRequestOptions, p: super::pdpb::GetRegionByIDRequest) -> ::grpc::GrpcSingleResponse<super::pdpb::GetRegionResponse>;

    fn AskSplit(&self, o: ::grpc::GrpcRequestOptions, p: super::pdpb::AskSplitRequest) -> ::grpc::GrpcSingleResponse<super::pdpb::AskSplitResponse>;

    fn ReportSplit(&self, o: ::grpc::GrpcRequestOptions, p: super::pdpb::ReportSplitRequest) -> ::grpc::GrpcSingleResponse<super::pdpb::ReportSplitResponse>;

    fn GetClusterConfig(&self, o: ::grpc::GrpcRequestOptions, p: super::pdpb::GetClusterConfigRequest) -> ::grpc::GrpcSingleResponse<super::pdpb::GetClusterConfigResponse>;

    fn PutClusterConfig(&self, o: ::grpc::GrpcRequestOptions, p: super::pdpb::PutClusterConfigRequest) -> ::grpc::GrpcSingleResponse<super::pdpb::PutClusterConfigResponse>;
}

// client

pub struct PDClient {
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

impl PDClient {
    pub fn with_client(grpc_client: ::grpc::client::GrpcClient) -> Self {
        PDClient {
            grpc_client: grpc_client,
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
    }

    pub fn new(host: &str, port: u16, tls: bool, conf: ::grpc::client::GrpcClientConf) -> ::grpc::result::GrpcResult<Self> {
        ::grpc::client::GrpcClient::new(host, port, tls, conf).map(|c| {
            PDClient::with_client(c)
        })
    }
}

impl PD for PDClient {
    fn GetMembers(&self, o: ::grpc::GrpcRequestOptions, p: super::pdpb::GetMembersRequest) -> ::grpc::GrpcSingleResponse<super::pdpb::GetMembersResponse> {
        self.grpc_client.call_unary(o, p, self.method_GetMembers.clone())
    }

    fn Tso(&self, o: ::grpc::GrpcRequestOptions, p: ::grpc::GrpcStreamingRequest<super::pdpb::TsoRequest>) -> ::grpc::GrpcStreamingResponse<super::pdpb::TsoResponse> {
        self.grpc_client.call_bidi(o, p, self.method_Tso.clone())
    }

    fn Bootstrap(&self, o: ::grpc::GrpcRequestOptions, p: super::pdpb::BootstrapRequest) -> ::grpc::GrpcSingleResponse<super::pdpb::BootstrapResponse> {
        self.grpc_client.call_unary(o, p, self.method_Bootstrap.clone())
    }

    fn IsBootstrapped(&self, o: ::grpc::GrpcRequestOptions, p: super::pdpb::IsBootstrappedRequest) -> ::grpc::GrpcSingleResponse<super::pdpb::IsBootstrappedResponse> {
        self.grpc_client.call_unary(o, p, self.method_IsBootstrapped.clone())
    }

    fn AllocID(&self, o: ::grpc::GrpcRequestOptions, p: super::pdpb::AllocIDRequest) -> ::grpc::GrpcSingleResponse<super::pdpb::AllocIDResponse> {
        self.grpc_client.call_unary(o, p, self.method_AllocID.clone())
    }

    fn GetStore(&self, o: ::grpc::GrpcRequestOptions, p: super::pdpb::GetStoreRequest) -> ::grpc::GrpcSingleResponse<super::pdpb::GetStoreResponse> {
        self.grpc_client.call_unary(o, p, self.method_GetStore.clone())
    }

    fn PutStore(&self, o: ::grpc::GrpcRequestOptions, p: super::pdpb::PutStoreRequest) -> ::grpc::GrpcSingleResponse<super::pdpb::PutStoreResponse> {
        self.grpc_client.call_unary(o, p, self.method_PutStore.clone())
    }

    fn StoreHeartbeat(&self, o: ::grpc::GrpcRequestOptions, p: super::pdpb::StoreHeartbeatRequest) -> ::grpc::GrpcSingleResponse<super::pdpb::StoreHeartbeatResponse> {
        self.grpc_client.call_unary(o, p, self.method_StoreHeartbeat.clone())
    }

    fn RegionHeartbeat(&self, o: ::grpc::GrpcRequestOptions, p: super::pdpb::RegionHeartbeatRequest) -> ::grpc::GrpcSingleResponse<super::pdpb::RegionHeartbeatResponse> {
        self.grpc_client.call_unary(o, p, self.method_RegionHeartbeat.clone())
    }

    fn GetRegion(&self, o: ::grpc::GrpcRequestOptions, p: super::pdpb::GetRegionRequest) -> ::grpc::GrpcSingleResponse<super::pdpb::GetRegionResponse> {
        self.grpc_client.call_unary(o, p, self.method_GetRegion.clone())
    }

    fn GetRegionByID(&self, o: ::grpc::GrpcRequestOptions, p: super::pdpb::GetRegionByIDRequest) -> ::grpc::GrpcSingleResponse<super::pdpb::GetRegionResponse> {
        self.grpc_client.call_unary(o, p, self.method_GetRegionByID.clone())
    }

    fn AskSplit(&self, o: ::grpc::GrpcRequestOptions, p: super::pdpb::AskSplitRequest) -> ::grpc::GrpcSingleResponse<super::pdpb::AskSplitResponse> {
        self.grpc_client.call_unary(o, p, self.method_AskSplit.clone())
    }

    fn ReportSplit(&self, o: ::grpc::GrpcRequestOptions, p: super::pdpb::ReportSplitRequest) -> ::grpc::GrpcSingleResponse<super::pdpb::ReportSplitResponse> {
        self.grpc_client.call_unary(o, p, self.method_ReportSplit.clone())
    }

    fn GetClusterConfig(&self, o: ::grpc::GrpcRequestOptions, p: super::pdpb::GetClusterConfigRequest) -> ::grpc::GrpcSingleResponse<super::pdpb::GetClusterConfigResponse> {
        self.grpc_client.call_unary(o, p, self.method_GetClusterConfig.clone())
    }

    fn PutClusterConfig(&self, o: ::grpc::GrpcRequestOptions, p: super::pdpb::PutClusterConfigRequest) -> ::grpc::GrpcSingleResponse<super::pdpb::PutClusterConfigResponse> {
        self.grpc_client.call_unary(o, p, self.method_PutClusterConfig.clone())
    }
}

// server

pub struct PDServer {
    pub grpc_server: ::grpc::server::GrpcServer,
}

impl ::std::ops::Deref for PDServer {
    type Target = ::grpc::server::GrpcServer;

    fn deref(&self) -> &Self::Target {
        &self.grpc_server
    }
}

impl PDServer {
    pub fn new<A : ::std::net::ToSocketAddrs, H : PD + 'static + Sync + Send + 'static>(addr: A, conf: ::grpc::server::GrpcServerConf, h: H) -> Self {
        let service_definition = PDServer::new_service_def(h);
        PDServer {
            grpc_server: ::grpc::server::GrpcServer::new_plain(addr, conf, service_definition),
        }
    }

    pub fn new_pool<A : ::std::net::ToSocketAddrs, H : PD + 'static + Sync + Send + 'static>(addr: A, conf: ::grpc::server::GrpcServerConf, h: H, cpu_pool: ::futures_cpupool::CpuPool) -> Self {
        let service_definition = PDServer::new_service_def(h);
        PDServer {
            grpc_server: ::grpc::server::GrpcServer::new_plain_pool(addr, conf, service_definition, cpu_pool),
        }
    }

    pub fn new_service_def<H : PD + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::server::ServerServiceDefinition {
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
                        ::grpc::server::MethodHandlerUnary::new(move |o, p| handler_copy.GetMembers(o, p))
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
                        ::grpc::server::MethodHandlerBidi::new(move |o, p| handler_copy.Tso(o, p))
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
                        ::grpc::server::MethodHandlerUnary::new(move |o, p| handler_copy.Bootstrap(o, p))
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
                        ::grpc::server::MethodHandlerUnary::new(move |o, p| handler_copy.IsBootstrapped(o, p))
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
                        ::grpc::server::MethodHandlerUnary::new(move |o, p| handler_copy.AllocID(o, p))
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
                        ::grpc::server::MethodHandlerUnary::new(move |o, p| handler_copy.GetStore(o, p))
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
                        ::grpc::server::MethodHandlerUnary::new(move |o, p| handler_copy.PutStore(o, p))
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
                        ::grpc::server::MethodHandlerUnary::new(move |o, p| handler_copy.StoreHeartbeat(o, p))
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
                        ::grpc::server::MethodHandlerUnary::new(move |o, p| handler_copy.RegionHeartbeat(o, p))
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
                        ::grpc::server::MethodHandlerUnary::new(move |o, p| handler_copy.GetRegion(o, p))
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
                        ::grpc::server::MethodHandlerUnary::new(move |o, p| handler_copy.GetRegionByID(o, p))
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
                        ::grpc::server::MethodHandlerUnary::new(move |o, p| handler_copy.AskSplit(o, p))
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
                        ::grpc::server::MethodHandlerUnary::new(move |o, p| handler_copy.ReportSplit(o, p))
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
                        ::grpc::server::MethodHandlerUnary::new(move |o, p| handler_copy.GetClusterConfig(o, p))
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
                        ::grpc::server::MethodHandlerUnary::new(move |o, p| handler_copy.PutClusterConfig(o, p))
                    },
                ),
            ],
        )
    }
}
