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

pub trait TiKV {
    fn KvGet(&self, p: super::kvrpcpb::GetRequest) -> ::grpc::result::GrpcResult<super::kvrpcpb::GetResponse>;

    fn KvScan(&self, p: super::kvrpcpb::ScanRequest) -> ::grpc::result::GrpcResult<super::kvrpcpb::ScanResponse>;

    fn KvPrewrite(&self, p: super::kvrpcpb::PrewriteRequest) -> ::grpc::result::GrpcResult<super::kvrpcpb::PrewriteResponse>;

    fn KvCommit(&self, p: super::kvrpcpb::CommitRequest) -> ::grpc::result::GrpcResult<super::kvrpcpb::CommitResponse>;

    fn KvCleanup(&self, p: super::kvrpcpb::CleanupRequest) -> ::grpc::result::GrpcResult<super::kvrpcpb::CleanupResponse>;

    fn KvBatchGet(&self, p: super::kvrpcpb::BatchGetRequest) -> ::grpc::result::GrpcResult<super::kvrpcpb::BatchGetResponse>;

    fn KvBatchRollback(&self, p: super::kvrpcpb::BatchRollbackRequest) -> ::grpc::result::GrpcResult<super::kvrpcpb::BatchRollbackResponse>;

    fn KvScanLock(&self, p: super::kvrpcpb::ScanLockRequest) -> ::grpc::result::GrpcResult<super::kvrpcpb::ScanLockResponse>;

    fn KvResolveLock(&self, p: super::kvrpcpb::ResolveLockRequest) -> ::grpc::result::GrpcResult<super::kvrpcpb::ResolveLockResponse>;

    fn KvGC(&self, p: super::kvrpcpb::GCRequest) -> ::grpc::result::GrpcResult<super::kvrpcpb::GCResponse>;

    fn RawGet(&self, p: super::kvrpcpb::RawGetRequest) -> ::grpc::result::GrpcResult<super::kvrpcpb::RawGetResponse>;

    fn RawPut(&self, p: super::kvrpcpb::RawPutRequest) -> ::grpc::result::GrpcResult<super::kvrpcpb::RawPutResponse>;

    fn RawDelete(&self, p: super::kvrpcpb::RawDeleteRequest) -> ::grpc::result::GrpcResult<super::kvrpcpb::RawDeleteResponse>;

    fn Coprocessor(&self, p: super::coprocessor::Request) -> ::grpc::result::GrpcResult<super::coprocessor::Response>;
}

pub trait TiKVAsync {
    fn KvGet(&self, p: super::kvrpcpb::GetRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::kvrpcpb::GetResponse>;

    fn KvScan(&self, p: super::kvrpcpb::ScanRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::kvrpcpb::ScanResponse>;

    fn KvPrewrite(&self, p: super::kvrpcpb::PrewriteRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::kvrpcpb::PrewriteResponse>;

    fn KvCommit(&self, p: super::kvrpcpb::CommitRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::kvrpcpb::CommitResponse>;

    fn KvCleanup(&self, p: super::kvrpcpb::CleanupRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::kvrpcpb::CleanupResponse>;

    fn KvBatchGet(&self, p: super::kvrpcpb::BatchGetRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::kvrpcpb::BatchGetResponse>;

    fn KvBatchRollback(&self, p: super::kvrpcpb::BatchRollbackRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::kvrpcpb::BatchRollbackResponse>;

    fn KvScanLock(&self, p: super::kvrpcpb::ScanLockRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::kvrpcpb::ScanLockResponse>;

    fn KvResolveLock(&self, p: super::kvrpcpb::ResolveLockRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::kvrpcpb::ResolveLockResponse>;

    fn KvGC(&self, p: super::kvrpcpb::GCRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::kvrpcpb::GCResponse>;

    fn RawGet(&self, p: super::kvrpcpb::RawGetRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::kvrpcpb::RawGetResponse>;

    fn RawPut(&self, p: super::kvrpcpb::RawPutRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::kvrpcpb::RawPutResponse>;

    fn RawDelete(&self, p: super::kvrpcpb::RawDeleteRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::kvrpcpb::RawDeleteResponse>;

    fn Coprocessor(&self, p: super::coprocessor::Request) -> ::grpc::futures_grpc::GrpcFutureSend<super::coprocessor::Response>;
}

// sync client

pub struct TiKVClient {
    async_client: TiKVAsyncClient,
}

impl TiKVClient {
    pub fn new(host: &str, port: u16, tls: bool, conf: ::grpc::client::GrpcClientConf) -> ::grpc::result::GrpcResult<Self> {
        TiKVAsyncClient::new(host, port, tls, conf).map(|c| {
            TiKVClient {
                async_client: c,
            }
        })
    }
}

impl TiKV for TiKVClient {
    fn KvGet(&self, p: super::kvrpcpb::GetRequest) -> ::grpc::result::GrpcResult<super::kvrpcpb::GetResponse> {
        ::futures::Future::wait(self.async_client.KvGet(p))
    }

    fn KvScan(&self, p: super::kvrpcpb::ScanRequest) -> ::grpc::result::GrpcResult<super::kvrpcpb::ScanResponse> {
        ::futures::Future::wait(self.async_client.KvScan(p))
    }

    fn KvPrewrite(&self, p: super::kvrpcpb::PrewriteRequest) -> ::grpc::result::GrpcResult<super::kvrpcpb::PrewriteResponse> {
        ::futures::Future::wait(self.async_client.KvPrewrite(p))
    }

    fn KvCommit(&self, p: super::kvrpcpb::CommitRequest) -> ::grpc::result::GrpcResult<super::kvrpcpb::CommitResponse> {
        ::futures::Future::wait(self.async_client.KvCommit(p))
    }

    fn KvCleanup(&self, p: super::kvrpcpb::CleanupRequest) -> ::grpc::result::GrpcResult<super::kvrpcpb::CleanupResponse> {
        ::futures::Future::wait(self.async_client.KvCleanup(p))
    }

    fn KvBatchGet(&self, p: super::kvrpcpb::BatchGetRequest) -> ::grpc::result::GrpcResult<super::kvrpcpb::BatchGetResponse> {
        ::futures::Future::wait(self.async_client.KvBatchGet(p))
    }

    fn KvBatchRollback(&self, p: super::kvrpcpb::BatchRollbackRequest) -> ::grpc::result::GrpcResult<super::kvrpcpb::BatchRollbackResponse> {
        ::futures::Future::wait(self.async_client.KvBatchRollback(p))
    }

    fn KvScanLock(&self, p: super::kvrpcpb::ScanLockRequest) -> ::grpc::result::GrpcResult<super::kvrpcpb::ScanLockResponse> {
        ::futures::Future::wait(self.async_client.KvScanLock(p))
    }

    fn KvResolveLock(&self, p: super::kvrpcpb::ResolveLockRequest) -> ::grpc::result::GrpcResult<super::kvrpcpb::ResolveLockResponse> {
        ::futures::Future::wait(self.async_client.KvResolveLock(p))
    }

    fn KvGC(&self, p: super::kvrpcpb::GCRequest) -> ::grpc::result::GrpcResult<super::kvrpcpb::GCResponse> {
        ::futures::Future::wait(self.async_client.KvGC(p))
    }

    fn RawGet(&self, p: super::kvrpcpb::RawGetRequest) -> ::grpc::result::GrpcResult<super::kvrpcpb::RawGetResponse> {
        ::futures::Future::wait(self.async_client.RawGet(p))
    }

    fn RawPut(&self, p: super::kvrpcpb::RawPutRequest) -> ::grpc::result::GrpcResult<super::kvrpcpb::RawPutResponse> {
        ::futures::Future::wait(self.async_client.RawPut(p))
    }

    fn RawDelete(&self, p: super::kvrpcpb::RawDeleteRequest) -> ::grpc::result::GrpcResult<super::kvrpcpb::RawDeleteResponse> {
        ::futures::Future::wait(self.async_client.RawDelete(p))
    }

    fn Coprocessor(&self, p: super::coprocessor::Request) -> ::grpc::result::GrpcResult<super::coprocessor::Response> {
        ::futures::Future::wait(self.async_client.Coprocessor(p))
    }
}

// async client

pub struct TiKVAsyncClient {
    grpc_client: ::grpc::client::GrpcClient,
    method_KvGet: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::kvrpcpb::GetRequest, super::kvrpcpb::GetResponse>>,
    method_KvScan: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::kvrpcpb::ScanRequest, super::kvrpcpb::ScanResponse>>,
    method_KvPrewrite: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::kvrpcpb::PrewriteRequest, super::kvrpcpb::PrewriteResponse>>,
    method_KvCommit: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::kvrpcpb::CommitRequest, super::kvrpcpb::CommitResponse>>,
    method_KvCleanup: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::kvrpcpb::CleanupRequest, super::kvrpcpb::CleanupResponse>>,
    method_KvBatchGet: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::kvrpcpb::BatchGetRequest, super::kvrpcpb::BatchGetResponse>>,
    method_KvBatchRollback: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::kvrpcpb::BatchRollbackRequest, super::kvrpcpb::BatchRollbackResponse>>,
    method_KvScanLock: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::kvrpcpb::ScanLockRequest, super::kvrpcpb::ScanLockResponse>>,
    method_KvResolveLock: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::kvrpcpb::ResolveLockRequest, super::kvrpcpb::ResolveLockResponse>>,
    method_KvGC: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::kvrpcpb::GCRequest, super::kvrpcpb::GCResponse>>,
    method_RawGet: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::kvrpcpb::RawGetRequest, super::kvrpcpb::RawGetResponse>>,
    method_RawPut: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::kvrpcpb::RawPutRequest, super::kvrpcpb::RawPutResponse>>,
    method_RawDelete: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::kvrpcpb::RawDeleteRequest, super::kvrpcpb::RawDeleteResponse>>,
    method_Coprocessor: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::coprocessor::Request, super::coprocessor::Response>>,
}

impl TiKVAsyncClient {
    pub fn new(host: &str, port: u16, tls: bool, conf: ::grpc::client::GrpcClientConf) -> ::grpc::result::GrpcResult<Self> {
        ::grpc::client::GrpcClient::new(host, port, tls, conf).map(|c| {
            TiKVAsyncClient {
                grpc_client: c,
                method_KvGet: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/tikvpb.TiKV/KvGet".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_KvScan: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/tikvpb.TiKV/KvScan".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_KvPrewrite: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/tikvpb.TiKV/KvPrewrite".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_KvCommit: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/tikvpb.TiKV/KvCommit".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_KvCleanup: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/tikvpb.TiKV/KvCleanup".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_KvBatchGet: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/tikvpb.TiKV/KvBatchGet".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_KvBatchRollback: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/tikvpb.TiKV/KvBatchRollback".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_KvScanLock: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/tikvpb.TiKV/KvScanLock".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_KvResolveLock: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/tikvpb.TiKV/KvResolveLock".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_KvGC: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/tikvpb.TiKV/KvGC".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_RawGet: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/tikvpb.TiKV/RawGet".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_RawPut: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/tikvpb.TiKV/RawPut".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_RawDelete: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/tikvpb.TiKV/RawDelete".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_Coprocessor: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/tikvpb.TiKV/Coprocessor".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
            }
        })
    }
}

impl TiKVAsync for TiKVAsyncClient {
    fn KvGet(&self, p: super::kvrpcpb::GetRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::kvrpcpb::GetResponse> {
        self.grpc_client.call_unary(p, self.method_KvGet.clone())
    }

    fn KvScan(&self, p: super::kvrpcpb::ScanRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::kvrpcpb::ScanResponse> {
        self.grpc_client.call_unary(p, self.method_KvScan.clone())
    }

    fn KvPrewrite(&self, p: super::kvrpcpb::PrewriteRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::kvrpcpb::PrewriteResponse> {
        self.grpc_client.call_unary(p, self.method_KvPrewrite.clone())
    }

    fn KvCommit(&self, p: super::kvrpcpb::CommitRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::kvrpcpb::CommitResponse> {
        self.grpc_client.call_unary(p, self.method_KvCommit.clone())
    }

    fn KvCleanup(&self, p: super::kvrpcpb::CleanupRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::kvrpcpb::CleanupResponse> {
        self.grpc_client.call_unary(p, self.method_KvCleanup.clone())
    }

    fn KvBatchGet(&self, p: super::kvrpcpb::BatchGetRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::kvrpcpb::BatchGetResponse> {
        self.grpc_client.call_unary(p, self.method_KvBatchGet.clone())
    }

    fn KvBatchRollback(&self, p: super::kvrpcpb::BatchRollbackRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::kvrpcpb::BatchRollbackResponse> {
        self.grpc_client.call_unary(p, self.method_KvBatchRollback.clone())
    }

    fn KvScanLock(&self, p: super::kvrpcpb::ScanLockRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::kvrpcpb::ScanLockResponse> {
        self.grpc_client.call_unary(p, self.method_KvScanLock.clone())
    }

    fn KvResolveLock(&self, p: super::kvrpcpb::ResolveLockRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::kvrpcpb::ResolveLockResponse> {
        self.grpc_client.call_unary(p, self.method_KvResolveLock.clone())
    }

    fn KvGC(&self, p: super::kvrpcpb::GCRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::kvrpcpb::GCResponse> {
        self.grpc_client.call_unary(p, self.method_KvGC.clone())
    }

    fn RawGet(&self, p: super::kvrpcpb::RawGetRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::kvrpcpb::RawGetResponse> {
        self.grpc_client.call_unary(p, self.method_RawGet.clone())
    }

    fn RawPut(&self, p: super::kvrpcpb::RawPutRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::kvrpcpb::RawPutResponse> {
        self.grpc_client.call_unary(p, self.method_RawPut.clone())
    }

    fn RawDelete(&self, p: super::kvrpcpb::RawDeleteRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::kvrpcpb::RawDeleteResponse> {
        self.grpc_client.call_unary(p, self.method_RawDelete.clone())
    }

    fn Coprocessor(&self, p: super::coprocessor::Request) -> ::grpc::futures_grpc::GrpcFutureSend<super::coprocessor::Response> {
        self.grpc_client.call_unary(p, self.method_Coprocessor.clone())
    }
}

// sync server

pub struct TiKVServer {
    async_server: TiKVAsyncServer,
}

impl ::std::ops::Deref for TiKVServer {
    type Target = TiKVAsyncServer;

    fn deref(&self) -> &Self::Target {
        &self.async_server
    }
}

struct TiKVServerHandlerToAsync {
    handler: ::std::sync::Arc<TiKV + Send + Sync>,
    cpupool: ::futures_cpupool::CpuPool,
}

impl TiKVAsync for TiKVServerHandlerToAsync {
    fn KvGet(&self, p: super::kvrpcpb::GetRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::kvrpcpb::GetResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.KvGet(p)
        })
    }

    fn KvScan(&self, p: super::kvrpcpb::ScanRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::kvrpcpb::ScanResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.KvScan(p)
        })
    }

    fn KvPrewrite(&self, p: super::kvrpcpb::PrewriteRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::kvrpcpb::PrewriteResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.KvPrewrite(p)
        })
    }

    fn KvCommit(&self, p: super::kvrpcpb::CommitRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::kvrpcpb::CommitResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.KvCommit(p)
        })
    }

    fn KvCleanup(&self, p: super::kvrpcpb::CleanupRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::kvrpcpb::CleanupResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.KvCleanup(p)
        })
    }

    fn KvBatchGet(&self, p: super::kvrpcpb::BatchGetRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::kvrpcpb::BatchGetResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.KvBatchGet(p)
        })
    }

    fn KvBatchRollback(&self, p: super::kvrpcpb::BatchRollbackRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::kvrpcpb::BatchRollbackResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.KvBatchRollback(p)
        })
    }

    fn KvScanLock(&self, p: super::kvrpcpb::ScanLockRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::kvrpcpb::ScanLockResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.KvScanLock(p)
        })
    }

    fn KvResolveLock(&self, p: super::kvrpcpb::ResolveLockRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::kvrpcpb::ResolveLockResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.KvResolveLock(p)
        })
    }

    fn KvGC(&self, p: super::kvrpcpb::GCRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::kvrpcpb::GCResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.KvGC(p)
        })
    }

    fn RawGet(&self, p: super::kvrpcpb::RawGetRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::kvrpcpb::RawGetResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.RawGet(p)
        })
    }

    fn RawPut(&self, p: super::kvrpcpb::RawPutRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::kvrpcpb::RawPutResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.RawPut(p)
        })
    }

    fn RawDelete(&self, p: super::kvrpcpb::RawDeleteRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::kvrpcpb::RawDeleteResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.RawDelete(p)
        })
    }

    fn Coprocessor(&self, p: super::coprocessor::Request) -> ::grpc::futures_grpc::GrpcFutureSend<super::coprocessor::Response> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.Coprocessor(p)
        })
    }
}

impl TiKVServer {
    pub fn new<A : ::std::net::ToSocketAddrs, H : TiKV + Send + Sync + 'static>(addr: A, conf: ::grpc::server::GrpcServerConf, h: H) -> Self {
        let h = TiKVServerHandlerToAsync {
            cpupool: ::futures_cpupool::CpuPool::new_num_cpus(),
            handler: ::std::sync::Arc::new(h),
        };
        TiKVServer {
            async_server: TiKVAsyncServer::new(addr, conf, h),
        }
    }
}

// async server

pub struct TiKVAsyncServer {
    grpc_server: ::grpc::server::GrpcServer,
}

impl ::std::ops::Deref for TiKVAsyncServer {
    type Target = ::grpc::server::GrpcServer;

    fn deref(&self) -> &Self::Target {
        &self.grpc_server
    }
}

impl TiKVAsyncServer {
    pub fn new<A : ::std::net::ToSocketAddrs, H : TiKVAsync + 'static + Sync + Send + 'static>(addr: A, conf: ::grpc::server::GrpcServerConf, h: H) -> Self {
        let service_definition = TiKVAsyncServer::new_service_def(h);
        TiKVAsyncServer {
            grpc_server: ::grpc::server::GrpcServer::new(addr, conf, service_definition),
        }
    }

    pub fn new_service_def<H : TiKVAsync + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::server::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::server::ServerServiceDefinition::new(
            vec![
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/tikvpb.TiKV/KvGet".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.KvGet(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/tikvpb.TiKV/KvScan".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.KvScan(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/tikvpb.TiKV/KvPrewrite".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.KvPrewrite(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/tikvpb.TiKV/KvCommit".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.KvCommit(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/tikvpb.TiKV/KvCleanup".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.KvCleanup(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/tikvpb.TiKV/KvBatchGet".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.KvBatchGet(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/tikvpb.TiKV/KvBatchRollback".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.KvBatchRollback(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/tikvpb.TiKV/KvScanLock".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.KvScanLock(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/tikvpb.TiKV/KvResolveLock".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.KvResolveLock(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/tikvpb.TiKV/KvGC".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.KvGC(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/tikvpb.TiKV/RawGet".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.RawGet(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/tikvpb.TiKV/RawPut".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.RawPut(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/tikvpb.TiKV/RawDelete".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.RawDelete(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/tikvpb.TiKV/Coprocessor".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.Coprocessor(p))
                    },
                ),
            ],
        )
    }
}
