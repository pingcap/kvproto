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
    fn get_members(&self, o: ::grpc::RequestOptions, p: super::pdpb::GetMembersRequest) -> ::grpc::SingleResponse<super::pdpb::GetMembersResponse>;

    fn tso(&self, o: ::grpc::RequestOptions, p: ::grpc::StreamingRequest<super::pdpb::TsoRequest>) -> ::grpc::StreamingResponse<super::pdpb::TsoResponse>;

    fn bootstrap(&self, o: ::grpc::RequestOptions, p: super::pdpb::BootstrapRequest) -> ::grpc::SingleResponse<super::pdpb::BootstrapResponse>;

    fn is_bootstrapped(&self, o: ::grpc::RequestOptions, p: super::pdpb::IsBootstrappedRequest) -> ::grpc::SingleResponse<super::pdpb::IsBootstrappedResponse>;

    fn alloc_id(&self, o: ::grpc::RequestOptions, p: super::pdpb::AllocIDRequest) -> ::grpc::SingleResponse<super::pdpb::AllocIDResponse>;

    fn get_store(&self, o: ::grpc::RequestOptions, p: super::pdpb::GetStoreRequest) -> ::grpc::SingleResponse<super::pdpb::GetStoreResponse>;

    fn put_store(&self, o: ::grpc::RequestOptions, p: super::pdpb::PutStoreRequest) -> ::grpc::SingleResponse<super::pdpb::PutStoreResponse>;

    fn store_heartbeat(&self, o: ::grpc::RequestOptions, p: super::pdpb::StoreHeartbeatRequest) -> ::grpc::SingleResponse<super::pdpb::StoreHeartbeatResponse>;

    fn region_heartbeat(&self, o: ::grpc::RequestOptions, p: super::pdpb::RegionHeartbeatRequest) -> ::grpc::SingleResponse<super::pdpb::RegionHeartbeatResponse>;

    fn get_region(&self, o: ::grpc::RequestOptions, p: super::pdpb::GetRegionRequest) -> ::grpc::SingleResponse<super::pdpb::GetRegionResponse>;

    fn get_region_by_id(&self, o: ::grpc::RequestOptions, p: super::pdpb::GetRegionByIDRequest) -> ::grpc::SingleResponse<super::pdpb::GetRegionResponse>;

    fn ask_split(&self, o: ::grpc::RequestOptions, p: super::pdpb::AskSplitRequest) -> ::grpc::SingleResponse<super::pdpb::AskSplitResponse>;

    fn report_split(&self, o: ::grpc::RequestOptions, p: super::pdpb::ReportSplitRequest) -> ::grpc::SingleResponse<super::pdpb::ReportSplitResponse>;

    fn get_cluster_config(&self, o: ::grpc::RequestOptions, p: super::pdpb::GetClusterConfigRequest) -> ::grpc::SingleResponse<super::pdpb::GetClusterConfigResponse>;

    fn put_cluster_config(&self, o: ::grpc::RequestOptions, p: super::pdpb::PutClusterConfigRequest) -> ::grpc::SingleResponse<super::pdpb::PutClusterConfigResponse>;
}

// client

pub struct PDClient {
    grpc_client: ::grpc::Client,
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
    pub fn with_client(grpc_client: ::grpc::Client) -> Self {
        PDClient {
            grpc_client: grpc_client,
            method_GetMembers: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                name: "/pdpb.PD/GetMembers".to_string(),
                streaming: ::grpc::method::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_Tso: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                name: "/pdpb.PD/Tso".to_string(),
                streaming: ::grpc::method::GrpcStreaming::Bidi,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_Bootstrap: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                name: "/pdpb.PD/Bootstrap".to_string(),
                streaming: ::grpc::method::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_IsBootstrapped: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                name: "/pdpb.PD/IsBootstrapped".to_string(),
                streaming: ::grpc::method::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_AllocID: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                name: "/pdpb.PD/AllocID".to_string(),
                streaming: ::grpc::method::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_GetStore: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                name: "/pdpb.PD/GetStore".to_string(),
                streaming: ::grpc::method::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_PutStore: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                name: "/pdpb.PD/PutStore".to_string(),
                streaming: ::grpc::method::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_StoreHeartbeat: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                name: "/pdpb.PD/StoreHeartbeat".to_string(),
                streaming: ::grpc::method::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_RegionHeartbeat: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                name: "/pdpb.PD/RegionHeartbeat".to_string(),
                streaming: ::grpc::method::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_GetRegion: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                name: "/pdpb.PD/GetRegion".to_string(),
                streaming: ::grpc::method::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_GetRegionByID: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                name: "/pdpb.PD/GetRegionByID".to_string(),
                streaming: ::grpc::method::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_AskSplit: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                name: "/pdpb.PD/AskSplit".to_string(),
                streaming: ::grpc::method::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_ReportSplit: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                name: "/pdpb.PD/ReportSplit".to_string(),
                streaming: ::grpc::method::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_GetClusterConfig: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                name: "/pdpb.PD/GetClusterConfig".to_string(),
                streaming: ::grpc::method::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_PutClusterConfig: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                name: "/pdpb.PD/PutClusterConfig".to_string(),
                streaming: ::grpc::method::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }

    pub fn new(host: &str, port: u16, tls: bool, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new(host, port, tls, conf).map(|c| {
            PDClient::with_client(c)
        })
    }
}

impl PD for PDClient {
    fn get_members(&self, o: ::grpc::RequestOptions, p: super::pdpb::GetMembersRequest) -> ::grpc::SingleResponse<super::pdpb::GetMembersResponse> {
        self.grpc_client.call_unary(o, p, self.method_GetMembers.clone())
    }

    fn tso(&self, o: ::grpc::RequestOptions, p: ::grpc::StreamingRequest<super::pdpb::TsoRequest>) -> ::grpc::StreamingResponse<super::pdpb::TsoResponse> {
        self.grpc_client.call_bidi(o, p, self.method_Tso.clone())
    }

    fn bootstrap(&self, o: ::grpc::RequestOptions, p: super::pdpb::BootstrapRequest) -> ::grpc::SingleResponse<super::pdpb::BootstrapResponse> {
        self.grpc_client.call_unary(o, p, self.method_Bootstrap.clone())
    }

    fn is_bootstrapped(&self, o: ::grpc::RequestOptions, p: super::pdpb::IsBootstrappedRequest) -> ::grpc::SingleResponse<super::pdpb::IsBootstrappedResponse> {
        self.grpc_client.call_unary(o, p, self.method_IsBootstrapped.clone())
    }

    fn alloc_id(&self, o: ::grpc::RequestOptions, p: super::pdpb::AllocIDRequest) -> ::grpc::SingleResponse<super::pdpb::AllocIDResponse> {
        self.grpc_client.call_unary(o, p, self.method_AllocID.clone())
    }

    fn get_store(&self, o: ::grpc::RequestOptions, p: super::pdpb::GetStoreRequest) -> ::grpc::SingleResponse<super::pdpb::GetStoreResponse> {
        self.grpc_client.call_unary(o, p, self.method_GetStore.clone())
    }

    fn put_store(&self, o: ::grpc::RequestOptions, p: super::pdpb::PutStoreRequest) -> ::grpc::SingleResponse<super::pdpb::PutStoreResponse> {
        self.grpc_client.call_unary(o, p, self.method_PutStore.clone())
    }

    fn store_heartbeat(&self, o: ::grpc::RequestOptions, p: super::pdpb::StoreHeartbeatRequest) -> ::grpc::SingleResponse<super::pdpb::StoreHeartbeatResponse> {
        self.grpc_client.call_unary(o, p, self.method_StoreHeartbeat.clone())
    }

    fn region_heartbeat(&self, o: ::grpc::RequestOptions, p: super::pdpb::RegionHeartbeatRequest) -> ::grpc::SingleResponse<super::pdpb::RegionHeartbeatResponse> {
        self.grpc_client.call_unary(o, p, self.method_RegionHeartbeat.clone())
    }

    fn get_region(&self, o: ::grpc::RequestOptions, p: super::pdpb::GetRegionRequest) -> ::grpc::SingleResponse<super::pdpb::GetRegionResponse> {
        self.grpc_client.call_unary(o, p, self.method_GetRegion.clone())
    }

    fn get_region_by_id(&self, o: ::grpc::RequestOptions, p: super::pdpb::GetRegionByIDRequest) -> ::grpc::SingleResponse<super::pdpb::GetRegionResponse> {
        self.grpc_client.call_unary(o, p, self.method_GetRegionByID.clone())
    }

    fn ask_split(&self, o: ::grpc::RequestOptions, p: super::pdpb::AskSplitRequest) -> ::grpc::SingleResponse<super::pdpb::AskSplitResponse> {
        self.grpc_client.call_unary(o, p, self.method_AskSplit.clone())
    }

    fn report_split(&self, o: ::grpc::RequestOptions, p: super::pdpb::ReportSplitRequest) -> ::grpc::SingleResponse<super::pdpb::ReportSplitResponse> {
        self.grpc_client.call_unary(o, p, self.method_ReportSplit.clone())
    }

    fn get_cluster_config(&self, o: ::grpc::RequestOptions, p: super::pdpb::GetClusterConfigRequest) -> ::grpc::SingleResponse<super::pdpb::GetClusterConfigResponse> {
        self.grpc_client.call_unary(o, p, self.method_GetClusterConfig.clone())
    }

    fn put_cluster_config(&self, o: ::grpc::RequestOptions, p: super::pdpb::PutClusterConfigRequest) -> ::grpc::SingleResponse<super::pdpb::PutClusterConfigResponse> {
        self.grpc_client.call_unary(o, p, self.method_PutClusterConfig.clone())
    }
}

// server

pub struct PDServer {
    pub grpc_server: ::grpc::Server,
}

impl ::std::ops::Deref for PDServer {
    type Target = ::grpc::Server;

    fn deref(&self) -> &Self::Target {
        &self.grpc_server
    }
}

impl PDServer {
    pub fn new<A : ::std::net::ToSocketAddrs, H : PD + 'static + Sync + Send + 'static>(addr: A, conf: ::grpc::ServerConf, h: H) -> Self {
        let service_definition = PDServer::new_service_def(h);
        PDServer {
            grpc_server: ::grpc::Server::new_plain(addr, conf, service_definition),
        }
    }

    pub fn new_pool<A : ::std::net::ToSocketAddrs, H : PD + 'static + Sync + Send + 'static>(addr: A, conf: ::grpc::ServerConf, h: H, cpu_pool: ::futures_cpupool::CpuPool) -> Self {
        let service_definition = PDServer::new_service_def(h);
        PDServer {
            grpc_server: ::grpc::Server::new_plain_pool(addr, conf, service_definition, cpu_pool),
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
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |o, p| handler_copy.get_members(o, p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/pdpb.PD/Tso".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Bidi,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerBidi::new(move |o, p| handler_copy.tso(o, p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/pdpb.PD/Bootstrap".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |o, p| handler_copy.bootstrap(o, p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/pdpb.PD/IsBootstrapped".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |o, p| handler_copy.is_bootstrapped(o, p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/pdpb.PD/AllocID".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |o, p| handler_copy.alloc_id(o, p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/pdpb.PD/GetStore".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |o, p| handler_copy.get_store(o, p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/pdpb.PD/PutStore".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |o, p| handler_copy.put_store(o, p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/pdpb.PD/StoreHeartbeat".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |o, p| handler_copy.store_heartbeat(o, p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/pdpb.PD/RegionHeartbeat".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |o, p| handler_copy.region_heartbeat(o, p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/pdpb.PD/GetRegion".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |o, p| handler_copy.get_region(o, p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/pdpb.PD/GetRegionByID".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |o, p| handler_copy.get_region_by_id(o, p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/pdpb.PD/AskSplit".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |o, p| handler_copy.ask_split(o, p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/pdpb.PD/ReportSplit".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |o, p| handler_copy.report_split(o, p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/pdpb.PD/GetClusterConfig".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |o, p| handler_copy.get_cluster_config(o, p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/pdpb.PD/PutClusterConfig".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |o, p| handler_copy.put_cluster_config(o, p))
                    },
                ),
            ],
        )
    }
}
