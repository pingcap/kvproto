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

pub trait Raft {
    fn Raft(&self, p: ::grpc::iter::GrpcIterator<super::raft_serverpb::RaftMessage>) -> ::grpc::result::GrpcResult<super::raft_serverpb::Done>;

    fn Snapshot(&self, p: ::grpc::iter::GrpcIterator<super::raft_serverpb::SnapshotChunk>) -> ::grpc::result::GrpcResult<super::raft_serverpb::Done>;
}

pub trait RaftAsync {
    fn Raft(&self, p: ::grpc::futures_grpc::GrpcStreamSend<super::raft_serverpb::RaftMessage>) -> ::grpc::futures_grpc::GrpcFutureSend<super::raft_serverpb::Done>;

    fn Snapshot(&self, p: ::grpc::futures_grpc::GrpcStreamSend<super::raft_serverpb::SnapshotChunk>) -> ::grpc::futures_grpc::GrpcFutureSend<super::raft_serverpb::Done>;
}

// sync client

pub struct RaftClient {
    async_client: RaftAsyncClient,
}

impl RaftClient {
    pub fn new(host: &str, port: u16, tls: bool, conf: ::grpc::client::GrpcClientConf) -> ::grpc::result::GrpcResult<Self> {
        RaftAsyncClient::new(host, port, tls, conf).map(|c| {
            RaftClient {
                async_client: c,
            }
        })
    }
}

impl Raft for RaftClient {
    fn Raft(&self, p: ::grpc::iter::GrpcIterator<super::raft_serverpb::RaftMessage>) -> ::grpc::result::GrpcResult<super::raft_serverpb::Done> {
        let p = ::futures::stream::Stream::boxed(::futures::stream::iter(::std::iter::IntoIterator::into_iter(p)));
        ::futures::Future::wait(self.async_client.Raft(p))
    }

    fn Snapshot(&self, p: ::grpc::iter::GrpcIterator<super::raft_serverpb::SnapshotChunk>) -> ::grpc::result::GrpcResult<super::raft_serverpb::Done> {
        let p = ::futures::stream::Stream::boxed(::futures::stream::iter(::std::iter::IntoIterator::into_iter(p)));
        ::futures::Future::wait(self.async_client.Snapshot(p))
    }
}

// async client

pub struct RaftAsyncClient {
    grpc_client: ::grpc::client::GrpcClient,
    method_Raft: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::raft_serverpb::RaftMessage, super::raft_serverpb::Done>>,
    method_Snapshot: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::raft_serverpb::SnapshotChunk, super::raft_serverpb::Done>>,
}

impl RaftAsyncClient {
    pub fn new(host: &str, port: u16, tls: bool, conf: ::grpc::client::GrpcClientConf) -> ::grpc::result::GrpcResult<Self> {
        ::grpc::client::GrpcClient::new(host, port, tls, conf).map(|c| {
            RaftAsyncClient {
                grpc_client: c,
                method_Raft: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/raft_serverpb.Raft/Raft".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::ClientStreaming,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_Snapshot: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/raft_serverpb.Raft/Snapshot".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::ClientStreaming,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
            }
        })
    }
}

impl RaftAsync for RaftAsyncClient {
    fn Raft(&self, p: ::grpc::futures_grpc::GrpcStreamSend<super::raft_serverpb::RaftMessage>) -> ::grpc::futures_grpc::GrpcFutureSend<super::raft_serverpb::Done> {
        self.grpc_client.call_client_streaming(p, self.method_Raft.clone())
    }

    fn Snapshot(&self, p: ::grpc::futures_grpc::GrpcStreamSend<super::raft_serverpb::SnapshotChunk>) -> ::grpc::futures_grpc::GrpcFutureSend<super::raft_serverpb::Done> {
        self.grpc_client.call_client_streaming(p, self.method_Snapshot.clone())
    }
}

// sync server

pub struct RaftServer {
    async_server: RaftAsyncServer,
}

impl ::std::ops::Deref for RaftServer {
    type Target = RaftAsyncServer;

    fn deref(&self) -> &Self::Target {
        &self.async_server
    }
}

struct RaftServerHandlerToAsync {
    handler: ::std::sync::Arc<Raft + Send + Sync>,
    cpupool: ::futures_cpupool::CpuPool,
}

impl RaftAsync for RaftServerHandlerToAsync {
    fn Raft(&self, p: ::grpc::futures_grpc::GrpcStreamSend<super::raft_serverpb::RaftMessage>) -> ::grpc::futures_grpc::GrpcFutureSend<super::raft_serverpb::Done> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_client_streaming(&self.cpupool, p, move |p| {
            h.Raft(p)
        })
    }

    fn Snapshot(&self, p: ::grpc::futures_grpc::GrpcStreamSend<super::raft_serverpb::SnapshotChunk>) -> ::grpc::futures_grpc::GrpcFutureSend<super::raft_serverpb::Done> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_client_streaming(&self.cpupool, p, move |p| {
            h.Snapshot(p)
        })
    }
}

impl RaftServer {
    pub fn new<A : ::std::net::ToSocketAddrs, H : Raft + Send + Sync + 'static>(addr: A, conf: ::grpc::server::GrpcServerConf, h: H) -> Self {
        let h = RaftServerHandlerToAsync {
            cpupool: ::futures_cpupool::CpuPool::new_num_cpus(),
            handler: ::std::sync::Arc::new(h),
        };
        RaftServer {
            async_server: RaftAsyncServer::new(addr, conf, h),
        }
    }
}

// async server

pub struct RaftAsyncServer {
    grpc_server: ::grpc::server::GrpcServer,
}

impl ::std::ops::Deref for RaftAsyncServer {
    type Target = ::grpc::server::GrpcServer;

    fn deref(&self) -> &Self::Target {
        &self.grpc_server
    }
}

impl RaftAsyncServer {
    pub fn new<A : ::std::net::ToSocketAddrs, H : RaftAsync + 'static + Sync + Send + 'static>(addr: A, conf: ::grpc::server::GrpcServerConf, h: H) -> Self {
        let service_definition = RaftAsyncServer::new_service_def(h);
        RaftAsyncServer {
            grpc_server: ::grpc::server::GrpcServer::new(addr, conf, service_definition),
        }
    }

    pub fn new_service_def<H : RaftAsync + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::server::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::server::ServerServiceDefinition::new(
            vec![
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/raft_serverpb.Raft/Raft".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::ClientStreaming,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerClientStreaming::new(move |p| handler_copy.Raft(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/raft_serverpb.Raft/Snapshot".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::ClientStreaming,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerClientStreaming::new(move |p| handler_copy.Snapshot(p))
                    },
                ),
            ],
        )
    }
}
