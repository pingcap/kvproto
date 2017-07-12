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

const METHOD_TIKV_KV_GET: ::grpc::Method<super::kvrpcpb::GetRequest, super::kvrpcpb::GetResponse> = ::grpc::Method {
    ty: ::grpc::MethodType::Unary,
    name: "/tikvpb.Tikv/KvGet",
    req_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
    resp_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
};

const METHOD_TIKV_KV_SCAN: ::grpc::Method<super::kvrpcpb::ScanRequest, super::kvrpcpb::ScanResponse> = ::grpc::Method {
    ty: ::grpc::MethodType::Unary,
    name: "/tikvpb.Tikv/KvScan",
    req_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
    resp_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
};

const METHOD_TIKV_KV_PREWRITE: ::grpc::Method<super::kvrpcpb::PrewriteRequest, super::kvrpcpb::PrewriteResponse> = ::grpc::Method {
    ty: ::grpc::MethodType::Unary,
    name: "/tikvpb.Tikv/KvPrewrite",
    req_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
    resp_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
};

const METHOD_TIKV_KV_COMMIT: ::grpc::Method<super::kvrpcpb::CommitRequest, super::kvrpcpb::CommitResponse> = ::grpc::Method {
    ty: ::grpc::MethodType::Unary,
    name: "/tikvpb.Tikv/KvCommit",
    req_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
    resp_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
};

const METHOD_TIKV_KV_IMPORT: ::grpc::Method<super::kvrpcpb::ImportRequest, super::kvrpcpb::ImportResponse> = ::grpc::Method {
    ty: ::grpc::MethodType::Unary,
    name: "/tikvpb.Tikv/KvImport",
    req_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
    resp_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
};

const METHOD_TIKV_KV_CLEANUP: ::grpc::Method<super::kvrpcpb::CleanupRequest, super::kvrpcpb::CleanupResponse> = ::grpc::Method {
    ty: ::grpc::MethodType::Unary,
    name: "/tikvpb.Tikv/KvCleanup",
    req_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
    resp_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
};

const METHOD_TIKV_KV_BATCH_GET: ::grpc::Method<super::kvrpcpb::BatchGetRequest, super::kvrpcpb::BatchGetResponse> = ::grpc::Method {
    ty: ::grpc::MethodType::Unary,
    name: "/tikvpb.Tikv/KvBatchGet",
    req_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
    resp_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
};

const METHOD_TIKV_KV_BATCH_ROLLBACK: ::grpc::Method<super::kvrpcpb::BatchRollbackRequest, super::kvrpcpb::BatchRollbackResponse> = ::grpc::Method {
    ty: ::grpc::MethodType::Unary,
    name: "/tikvpb.Tikv/KvBatchRollback",
    req_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
    resp_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
};

const METHOD_TIKV_KV_SCAN_LOCK: ::grpc::Method<super::kvrpcpb::ScanLockRequest, super::kvrpcpb::ScanLockResponse> = ::grpc::Method {
    ty: ::grpc::MethodType::Unary,
    name: "/tikvpb.Tikv/KvScanLock",
    req_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
    resp_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
};

const METHOD_TIKV_KV_RESOLVE_LOCK: ::grpc::Method<super::kvrpcpb::ResolveLockRequest, super::kvrpcpb::ResolveLockResponse> = ::grpc::Method {
    ty: ::grpc::MethodType::Unary,
    name: "/tikvpb.Tikv/KvResolveLock",
    req_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
    resp_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
};

const METHOD_TIKV_KV_GC: ::grpc::Method<super::kvrpcpb::GCRequest, super::kvrpcpb::GCResponse> = ::grpc::Method {
    ty: ::grpc::MethodType::Unary,
    name: "/tikvpb.Tikv/KvGC",
    req_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
    resp_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
};

const METHOD_TIKV_RAW_GET: ::grpc::Method<super::kvrpcpb::RawGetRequest, super::kvrpcpb::RawGetResponse> = ::grpc::Method {
    ty: ::grpc::MethodType::Unary,
    name: "/tikvpb.Tikv/RawGet",
    req_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
    resp_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
};

const METHOD_TIKV_RAW_PUT: ::grpc::Method<super::kvrpcpb::RawPutRequest, super::kvrpcpb::RawPutResponse> = ::grpc::Method {
    ty: ::grpc::MethodType::Unary,
    name: "/tikvpb.Tikv/RawPut",
    req_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
    resp_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
};

const METHOD_TIKV_RAW_DELETE: ::grpc::Method<super::kvrpcpb::RawDeleteRequest, super::kvrpcpb::RawDeleteResponse> = ::grpc::Method {
    ty: ::grpc::MethodType::Unary,
    name: "/tikvpb.Tikv/RawDelete",
    req_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
    resp_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
};

const METHOD_TIKV_RAW_SCAN: ::grpc::Method<super::kvrpcpb::RawScanRequest, super::kvrpcpb::RawScanResponse> = ::grpc::Method {
    ty: ::grpc::MethodType::Unary,
    name: "/tikvpb.Tikv/RawScan",
    req_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
    resp_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
};

const METHOD_TIKV_COPROCESSOR: ::grpc::Method<super::coprocessor::Request, super::coprocessor::Response> = ::grpc::Method {
    ty: ::grpc::MethodType::Unary,
    name: "/tikvpb.Tikv/Coprocessor",
    req_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
    resp_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
};

const METHOD_TIKV_RAFT: ::grpc::Method<super::raft_serverpb::RaftMessage, super::raft_serverpb::Done> = ::grpc::Method {
    ty: ::grpc::MethodType::ClientStreaming,
    name: "/tikvpb.Tikv/Raft",
    req_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
    resp_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
};

const METHOD_TIKV_SNAPSHOT: ::grpc::Method<super::raft_serverpb::SnapshotChunk, super::raft_serverpb::Done> = ::grpc::Method {
    ty: ::grpc::MethodType::ClientStreaming,
    name: "/tikvpb.Tikv/Snapshot",
    req_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
    resp_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
};

const METHOD_TIKV_MVCC_GET_BY_KEY: ::grpc::Method<super::kvrpcpb::MvccGetByKeyRequest, super::kvrpcpb::MvccGetByKeyResponse> = ::grpc::Method {
    ty: ::grpc::MethodType::Unary,
    name: "/tikvpb.Tikv/MvccGetByKey",
    req_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
    resp_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
};

const METHOD_TIKV_MVCC_GET_BY_START_TS: ::grpc::Method<super::kvrpcpb::MvccGetByStartTsRequest, super::kvrpcpb::MvccGetByStartTsResponse> = ::grpc::Method {
    ty: ::grpc::MethodType::Unary,
    name: "/tikvpb.Tikv/MvccGetByStartTs",
    req_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
    resp_mar: ::grpc::Marshaller { ser: ::grpc::pb_ser, de: ::grpc::pb_de },
};

pub struct TikvClient {
    client: ::grpc::Client,
}

impl TikvClient {
    pub fn new(channel: ::grpc::Channel) -> Self {
        TikvClient {
            client: ::grpc::Client::new(channel),
        }
    }

    pub fn kv_get_opt(&self, req: super::kvrpcpb::GetRequest, opt: ::grpc::CallOption) -> ::grpc::Result<super::kvrpcpb::GetResponse> {
        self.client.unary_call(&METHOD_TIKV_KV_GET, req, opt)
    }

    pub fn kv_get(&self, req: super::kvrpcpb::GetRequest) -> ::grpc::Result<super::kvrpcpb::GetResponse> {
        self.kv_get_opt(req, ::grpc::CallOption::default())
    }

    pub fn kv_get_async_opt(&self, req: super::kvrpcpb::GetRequest, opt: ::grpc::CallOption) -> ::grpc::ClientUnaryReceiver<super::kvrpcpb::GetResponse> {
        self.client.unary_call_async(&METHOD_TIKV_KV_GET, req, opt)
    }

    pub fn kv_get_async(&self, req: super::kvrpcpb::GetRequest) -> ::grpc::ClientUnaryReceiver<super::kvrpcpb::GetResponse> {
        self.kv_get_async_opt(req, ::grpc::CallOption::default())
    }

    pub fn kv_scan_opt(&self, req: super::kvrpcpb::ScanRequest, opt: ::grpc::CallOption) -> ::grpc::Result<super::kvrpcpb::ScanResponse> {
        self.client.unary_call(&METHOD_TIKV_KV_SCAN, req, opt)
    }

    pub fn kv_scan(&self, req: super::kvrpcpb::ScanRequest) -> ::grpc::Result<super::kvrpcpb::ScanResponse> {
        self.kv_scan_opt(req, ::grpc::CallOption::default())
    }

    pub fn kv_scan_async_opt(&self, req: super::kvrpcpb::ScanRequest, opt: ::grpc::CallOption) -> ::grpc::ClientUnaryReceiver<super::kvrpcpb::ScanResponse> {
        self.client.unary_call_async(&METHOD_TIKV_KV_SCAN, req, opt)
    }

    pub fn kv_scan_async(&self, req: super::kvrpcpb::ScanRequest) -> ::grpc::ClientUnaryReceiver<super::kvrpcpb::ScanResponse> {
        self.kv_scan_async_opt(req, ::grpc::CallOption::default())
    }

    pub fn kv_prewrite_opt(&self, req: super::kvrpcpb::PrewriteRequest, opt: ::grpc::CallOption) -> ::grpc::Result<super::kvrpcpb::PrewriteResponse> {
        self.client.unary_call(&METHOD_TIKV_KV_PREWRITE, req, opt)
    }

    pub fn kv_prewrite(&self, req: super::kvrpcpb::PrewriteRequest) -> ::grpc::Result<super::kvrpcpb::PrewriteResponse> {
        self.kv_prewrite_opt(req, ::grpc::CallOption::default())
    }

    pub fn kv_prewrite_async_opt(&self, req: super::kvrpcpb::PrewriteRequest, opt: ::grpc::CallOption) -> ::grpc::ClientUnaryReceiver<super::kvrpcpb::PrewriteResponse> {
        self.client.unary_call_async(&METHOD_TIKV_KV_PREWRITE, req, opt)
    }

    pub fn kv_prewrite_async(&self, req: super::kvrpcpb::PrewriteRequest) -> ::grpc::ClientUnaryReceiver<super::kvrpcpb::PrewriteResponse> {
        self.kv_prewrite_async_opt(req, ::grpc::CallOption::default())
    }

    pub fn kv_commit_opt(&self, req: super::kvrpcpb::CommitRequest, opt: ::grpc::CallOption) -> ::grpc::Result<super::kvrpcpb::CommitResponse> {
        self.client.unary_call(&METHOD_TIKV_KV_COMMIT, req, opt)
    }

    pub fn kv_commit(&self, req: super::kvrpcpb::CommitRequest) -> ::grpc::Result<super::kvrpcpb::CommitResponse> {
        self.kv_commit_opt(req, ::grpc::CallOption::default())
    }

    pub fn kv_commit_async_opt(&self, req: super::kvrpcpb::CommitRequest, opt: ::grpc::CallOption) -> ::grpc::ClientUnaryReceiver<super::kvrpcpb::CommitResponse> {
        self.client.unary_call_async(&METHOD_TIKV_KV_COMMIT, req, opt)
    }

    pub fn kv_commit_async(&self, req: super::kvrpcpb::CommitRequest) -> ::grpc::ClientUnaryReceiver<super::kvrpcpb::CommitResponse> {
        self.kv_commit_async_opt(req, ::grpc::CallOption::default())
    }

    pub fn kv_import_opt(&self, req: super::kvrpcpb::ImportRequest, opt: ::grpc::CallOption) -> ::grpc::Result<super::kvrpcpb::ImportResponse> {
        self.client.unary_call(&METHOD_TIKV_KV_IMPORT, req, opt)
    }

    pub fn kv_import(&self, req: super::kvrpcpb::ImportRequest) -> ::grpc::Result<super::kvrpcpb::ImportResponse> {
        self.kv_import_opt(req, ::grpc::CallOption::default())
    }

    pub fn kv_import_async_opt(&self, req: super::kvrpcpb::ImportRequest, opt: ::grpc::CallOption) -> ::grpc::ClientUnaryReceiver<super::kvrpcpb::ImportResponse> {
        self.client.unary_call_async(&METHOD_TIKV_KV_IMPORT, req, opt)
    }

    pub fn kv_import_async(&self, req: super::kvrpcpb::ImportRequest) -> ::grpc::ClientUnaryReceiver<super::kvrpcpb::ImportResponse> {
        self.kv_import_async_opt(req, ::grpc::CallOption::default())
    }

    pub fn kv_cleanup_opt(&self, req: super::kvrpcpb::CleanupRequest, opt: ::grpc::CallOption) -> ::grpc::Result<super::kvrpcpb::CleanupResponse> {
        self.client.unary_call(&METHOD_TIKV_KV_CLEANUP, req, opt)
    }

    pub fn kv_cleanup(&self, req: super::kvrpcpb::CleanupRequest) -> ::grpc::Result<super::kvrpcpb::CleanupResponse> {
        self.kv_cleanup_opt(req, ::grpc::CallOption::default())
    }

    pub fn kv_cleanup_async_opt(&self, req: super::kvrpcpb::CleanupRequest, opt: ::grpc::CallOption) -> ::grpc::ClientUnaryReceiver<super::kvrpcpb::CleanupResponse> {
        self.client.unary_call_async(&METHOD_TIKV_KV_CLEANUP, req, opt)
    }

    pub fn kv_cleanup_async(&self, req: super::kvrpcpb::CleanupRequest) -> ::grpc::ClientUnaryReceiver<super::kvrpcpb::CleanupResponse> {
        self.kv_cleanup_async_opt(req, ::grpc::CallOption::default())
    }

    pub fn kv_batch_get_opt(&self, req: super::kvrpcpb::BatchGetRequest, opt: ::grpc::CallOption) -> ::grpc::Result<super::kvrpcpb::BatchGetResponse> {
        self.client.unary_call(&METHOD_TIKV_KV_BATCH_GET, req, opt)
    }

    pub fn kv_batch_get(&self, req: super::kvrpcpb::BatchGetRequest) -> ::grpc::Result<super::kvrpcpb::BatchGetResponse> {
        self.kv_batch_get_opt(req, ::grpc::CallOption::default())
    }

    pub fn kv_batch_get_async_opt(&self, req: super::kvrpcpb::BatchGetRequest, opt: ::grpc::CallOption) -> ::grpc::ClientUnaryReceiver<super::kvrpcpb::BatchGetResponse> {
        self.client.unary_call_async(&METHOD_TIKV_KV_BATCH_GET, req, opt)
    }

    pub fn kv_batch_get_async(&self, req: super::kvrpcpb::BatchGetRequest) -> ::grpc::ClientUnaryReceiver<super::kvrpcpb::BatchGetResponse> {
        self.kv_batch_get_async_opt(req, ::grpc::CallOption::default())
    }

    pub fn kv_batch_rollback_opt(&self, req: super::kvrpcpb::BatchRollbackRequest, opt: ::grpc::CallOption) -> ::grpc::Result<super::kvrpcpb::BatchRollbackResponse> {
        self.client.unary_call(&METHOD_TIKV_KV_BATCH_ROLLBACK, req, opt)
    }

    pub fn kv_batch_rollback(&self, req: super::kvrpcpb::BatchRollbackRequest) -> ::grpc::Result<super::kvrpcpb::BatchRollbackResponse> {
        self.kv_batch_rollback_opt(req, ::grpc::CallOption::default())
    }

    pub fn kv_batch_rollback_async_opt(&self, req: super::kvrpcpb::BatchRollbackRequest, opt: ::grpc::CallOption) -> ::grpc::ClientUnaryReceiver<super::kvrpcpb::BatchRollbackResponse> {
        self.client.unary_call_async(&METHOD_TIKV_KV_BATCH_ROLLBACK, req, opt)
    }

    pub fn kv_batch_rollback_async(&self, req: super::kvrpcpb::BatchRollbackRequest) -> ::grpc::ClientUnaryReceiver<super::kvrpcpb::BatchRollbackResponse> {
        self.kv_batch_rollback_async_opt(req, ::grpc::CallOption::default())
    }

    pub fn kv_scan_lock_opt(&self, req: super::kvrpcpb::ScanLockRequest, opt: ::grpc::CallOption) -> ::grpc::Result<super::kvrpcpb::ScanLockResponse> {
        self.client.unary_call(&METHOD_TIKV_KV_SCAN_LOCK, req, opt)
    }

    pub fn kv_scan_lock(&self, req: super::kvrpcpb::ScanLockRequest) -> ::grpc::Result<super::kvrpcpb::ScanLockResponse> {
        self.kv_scan_lock_opt(req, ::grpc::CallOption::default())
    }

    pub fn kv_scan_lock_async_opt(&self, req: super::kvrpcpb::ScanLockRequest, opt: ::grpc::CallOption) -> ::grpc::ClientUnaryReceiver<super::kvrpcpb::ScanLockResponse> {
        self.client.unary_call_async(&METHOD_TIKV_KV_SCAN_LOCK, req, opt)
    }

    pub fn kv_scan_lock_async(&self, req: super::kvrpcpb::ScanLockRequest) -> ::grpc::ClientUnaryReceiver<super::kvrpcpb::ScanLockResponse> {
        self.kv_scan_lock_async_opt(req, ::grpc::CallOption::default())
    }

    pub fn kv_resolve_lock_opt(&self, req: super::kvrpcpb::ResolveLockRequest, opt: ::grpc::CallOption) -> ::grpc::Result<super::kvrpcpb::ResolveLockResponse> {
        self.client.unary_call(&METHOD_TIKV_KV_RESOLVE_LOCK, req, opt)
    }

    pub fn kv_resolve_lock(&self, req: super::kvrpcpb::ResolveLockRequest) -> ::grpc::Result<super::kvrpcpb::ResolveLockResponse> {
        self.kv_resolve_lock_opt(req, ::grpc::CallOption::default())
    }

    pub fn kv_resolve_lock_async_opt(&self, req: super::kvrpcpb::ResolveLockRequest, opt: ::grpc::CallOption) -> ::grpc::ClientUnaryReceiver<super::kvrpcpb::ResolveLockResponse> {
        self.client.unary_call_async(&METHOD_TIKV_KV_RESOLVE_LOCK, req, opt)
    }

    pub fn kv_resolve_lock_async(&self, req: super::kvrpcpb::ResolveLockRequest) -> ::grpc::ClientUnaryReceiver<super::kvrpcpb::ResolveLockResponse> {
        self.kv_resolve_lock_async_opt(req, ::grpc::CallOption::default())
    }

    pub fn kv_gc_opt(&self, req: super::kvrpcpb::GCRequest, opt: ::grpc::CallOption) -> ::grpc::Result<super::kvrpcpb::GCResponse> {
        self.client.unary_call(&METHOD_TIKV_KV_GC, req, opt)
    }

    pub fn kv_gc(&self, req: super::kvrpcpb::GCRequest) -> ::grpc::Result<super::kvrpcpb::GCResponse> {
        self.kv_gc_opt(req, ::grpc::CallOption::default())
    }

    pub fn kv_gc_async_opt(&self, req: super::kvrpcpb::GCRequest, opt: ::grpc::CallOption) -> ::grpc::ClientUnaryReceiver<super::kvrpcpb::GCResponse> {
        self.client.unary_call_async(&METHOD_TIKV_KV_GC, req, opt)
    }

    pub fn kv_gc_async(&self, req: super::kvrpcpb::GCRequest) -> ::grpc::ClientUnaryReceiver<super::kvrpcpb::GCResponse> {
        self.kv_gc_async_opt(req, ::grpc::CallOption::default())
    }

    pub fn raw_get_opt(&self, req: super::kvrpcpb::RawGetRequest, opt: ::grpc::CallOption) -> ::grpc::Result<super::kvrpcpb::RawGetResponse> {
        self.client.unary_call(&METHOD_TIKV_RAW_GET, req, opt)
    }

    pub fn raw_get(&self, req: super::kvrpcpb::RawGetRequest) -> ::grpc::Result<super::kvrpcpb::RawGetResponse> {
        self.raw_get_opt(req, ::grpc::CallOption::default())
    }

    pub fn raw_get_async_opt(&self, req: super::kvrpcpb::RawGetRequest, opt: ::grpc::CallOption) -> ::grpc::ClientUnaryReceiver<super::kvrpcpb::RawGetResponse> {
        self.client.unary_call_async(&METHOD_TIKV_RAW_GET, req, opt)
    }

    pub fn raw_get_async(&self, req: super::kvrpcpb::RawGetRequest) -> ::grpc::ClientUnaryReceiver<super::kvrpcpb::RawGetResponse> {
        self.raw_get_async_opt(req, ::grpc::CallOption::default())
    }

    pub fn raw_put_opt(&self, req: super::kvrpcpb::RawPutRequest, opt: ::grpc::CallOption) -> ::grpc::Result<super::kvrpcpb::RawPutResponse> {
        self.client.unary_call(&METHOD_TIKV_RAW_PUT, req, opt)
    }

    pub fn raw_put(&self, req: super::kvrpcpb::RawPutRequest) -> ::grpc::Result<super::kvrpcpb::RawPutResponse> {
        self.raw_put_opt(req, ::grpc::CallOption::default())
    }

    pub fn raw_put_async_opt(&self, req: super::kvrpcpb::RawPutRequest, opt: ::grpc::CallOption) -> ::grpc::ClientUnaryReceiver<super::kvrpcpb::RawPutResponse> {
        self.client.unary_call_async(&METHOD_TIKV_RAW_PUT, req, opt)
    }

    pub fn raw_put_async(&self, req: super::kvrpcpb::RawPutRequest) -> ::grpc::ClientUnaryReceiver<super::kvrpcpb::RawPutResponse> {
        self.raw_put_async_opt(req, ::grpc::CallOption::default())
    }

    pub fn raw_delete_opt(&self, req: super::kvrpcpb::RawDeleteRequest, opt: ::grpc::CallOption) -> ::grpc::Result<super::kvrpcpb::RawDeleteResponse> {
        self.client.unary_call(&METHOD_TIKV_RAW_DELETE, req, opt)
    }

    pub fn raw_delete(&self, req: super::kvrpcpb::RawDeleteRequest) -> ::grpc::Result<super::kvrpcpb::RawDeleteResponse> {
        self.raw_delete_opt(req, ::grpc::CallOption::default())
    }

    pub fn raw_delete_async_opt(&self, req: super::kvrpcpb::RawDeleteRequest, opt: ::grpc::CallOption) -> ::grpc::ClientUnaryReceiver<super::kvrpcpb::RawDeleteResponse> {
        self.client.unary_call_async(&METHOD_TIKV_RAW_DELETE, req, opt)
    }

    pub fn raw_delete_async(&self, req: super::kvrpcpb::RawDeleteRequest) -> ::grpc::ClientUnaryReceiver<super::kvrpcpb::RawDeleteResponse> {
        self.raw_delete_async_opt(req, ::grpc::CallOption::default())
    }

    pub fn raw_scan_opt(&self, req: super::kvrpcpb::RawScanRequest, opt: ::grpc::CallOption) -> ::grpc::Result<super::kvrpcpb::RawScanResponse> {
        self.client.unary_call(&METHOD_TIKV_RAW_SCAN, req, opt)
    }

    pub fn raw_scan(&self, req: super::kvrpcpb::RawScanRequest) -> ::grpc::Result<super::kvrpcpb::RawScanResponse> {
        self.raw_scan_opt(req, ::grpc::CallOption::default())
    }

    pub fn raw_scan_async_opt(&self, req: super::kvrpcpb::RawScanRequest, opt: ::grpc::CallOption) -> ::grpc::ClientUnaryReceiver<super::kvrpcpb::RawScanResponse> {
        self.client.unary_call_async(&METHOD_TIKV_RAW_SCAN, req, opt)
    }

    pub fn raw_scan_async(&self, req: super::kvrpcpb::RawScanRequest) -> ::grpc::ClientUnaryReceiver<super::kvrpcpb::RawScanResponse> {
        self.raw_scan_async_opt(req, ::grpc::CallOption::default())
    }

    pub fn coprocessor_opt(&self, req: super::coprocessor::Request, opt: ::grpc::CallOption) -> ::grpc::Result<super::coprocessor::Response> {
        self.client.unary_call(&METHOD_TIKV_COPROCESSOR, req, opt)
    }

    pub fn coprocessor(&self, req: super::coprocessor::Request) -> ::grpc::Result<super::coprocessor::Response> {
        self.coprocessor_opt(req, ::grpc::CallOption::default())
    }

    pub fn coprocessor_async_opt(&self, req: super::coprocessor::Request, opt: ::grpc::CallOption) -> ::grpc::ClientUnaryReceiver<super::coprocessor::Response> {
        self.client.unary_call_async(&METHOD_TIKV_COPROCESSOR, req, opt)
    }

    pub fn coprocessor_async(&self, req: super::coprocessor::Request) -> ::grpc::ClientUnaryReceiver<super::coprocessor::Response> {
        self.coprocessor_async_opt(req, ::grpc::CallOption::default())
    }

    pub fn raft_opt(&self, opt: ::grpc::CallOption) -> (::grpc::ClientCStreamSender<super::raft_serverpb::RaftMessage>, ::grpc::ClientCStreamReceiver<super::raft_serverpb::Done>) {
        self.client.client_streaming(&METHOD_TIKV_RAFT, opt)
    }

    pub fn raft(&self) -> (::grpc::ClientCStreamSender<super::raft_serverpb::RaftMessage>, ::grpc::ClientCStreamReceiver<super::raft_serverpb::Done>) {
        self.raft_opt(::grpc::CallOption::default())
    }

    pub fn snapshot_opt(&self, opt: ::grpc::CallOption) -> (::grpc::ClientCStreamSender<super::raft_serverpb::SnapshotChunk>, ::grpc::ClientCStreamReceiver<super::raft_serverpb::Done>) {
        self.client.client_streaming(&METHOD_TIKV_SNAPSHOT, opt)
    }

    pub fn snapshot(&self) -> (::grpc::ClientCStreamSender<super::raft_serverpb::SnapshotChunk>, ::grpc::ClientCStreamReceiver<super::raft_serverpb::Done>) {
        self.snapshot_opt(::grpc::CallOption::default())
    }

    pub fn mvcc_get_by_key_opt(&self, req: super::kvrpcpb::MvccGetByKeyRequest, opt: ::grpc::CallOption) -> ::grpc::Result<super::kvrpcpb::MvccGetByKeyResponse> {
        self.client.unary_call(&METHOD_TIKV_MVCC_GET_BY_KEY, req, opt)
    }

    pub fn mvcc_get_by_key(&self, req: super::kvrpcpb::MvccGetByKeyRequest) -> ::grpc::Result<super::kvrpcpb::MvccGetByKeyResponse> {
        self.mvcc_get_by_key_opt(req, ::grpc::CallOption::default())
    }

    pub fn mvcc_get_by_key_async_opt(&self, req: super::kvrpcpb::MvccGetByKeyRequest, opt: ::grpc::CallOption) -> ::grpc::ClientUnaryReceiver<super::kvrpcpb::MvccGetByKeyResponse> {
        self.client.unary_call_async(&METHOD_TIKV_MVCC_GET_BY_KEY, req, opt)
    }

    pub fn mvcc_get_by_key_async(&self, req: super::kvrpcpb::MvccGetByKeyRequest) -> ::grpc::ClientUnaryReceiver<super::kvrpcpb::MvccGetByKeyResponse> {
        self.mvcc_get_by_key_async_opt(req, ::grpc::CallOption::default())
    }

    pub fn mvcc_get_by_start_ts_opt(&self, req: super::kvrpcpb::MvccGetByStartTsRequest, opt: ::grpc::CallOption) -> ::grpc::Result<super::kvrpcpb::MvccGetByStartTsResponse> {
        self.client.unary_call(&METHOD_TIKV_MVCC_GET_BY_START_TS, req, opt)
    }

    pub fn mvcc_get_by_start_ts(&self, req: super::kvrpcpb::MvccGetByStartTsRequest) -> ::grpc::Result<super::kvrpcpb::MvccGetByStartTsResponse> {
        self.mvcc_get_by_start_ts_opt(req, ::grpc::CallOption::default())
    }

    pub fn mvcc_get_by_start_ts_async_opt(&self, req: super::kvrpcpb::MvccGetByStartTsRequest, opt: ::grpc::CallOption) -> ::grpc::ClientUnaryReceiver<super::kvrpcpb::MvccGetByStartTsResponse> {
        self.client.unary_call_async(&METHOD_TIKV_MVCC_GET_BY_START_TS, req, opt)
    }

    pub fn mvcc_get_by_start_ts_async(&self, req: super::kvrpcpb::MvccGetByStartTsRequest) -> ::grpc::ClientUnaryReceiver<super::kvrpcpb::MvccGetByStartTsResponse> {
        self.mvcc_get_by_start_ts_async_opt(req, ::grpc::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Tikv {
    fn kv_get(&self, ctx: ::grpc::RpcContext, req: super::kvrpcpb::GetRequest, sink: ::grpc::UnarySink<super::kvrpcpb::GetResponse>);
    fn kv_scan(&self, ctx: ::grpc::RpcContext, req: super::kvrpcpb::ScanRequest, sink: ::grpc::UnarySink<super::kvrpcpb::ScanResponse>);
    fn kv_prewrite(&self, ctx: ::grpc::RpcContext, req: super::kvrpcpb::PrewriteRequest, sink: ::grpc::UnarySink<super::kvrpcpb::PrewriteResponse>);
    fn kv_commit(&self, ctx: ::grpc::RpcContext, req: super::kvrpcpb::CommitRequest, sink: ::grpc::UnarySink<super::kvrpcpb::CommitResponse>);
    fn kv_import(&self, ctx: ::grpc::RpcContext, req: super::kvrpcpb::ImportRequest, sink: ::grpc::UnarySink<super::kvrpcpb::ImportResponse>);
    fn kv_cleanup(&self, ctx: ::grpc::RpcContext, req: super::kvrpcpb::CleanupRequest, sink: ::grpc::UnarySink<super::kvrpcpb::CleanupResponse>);
    fn kv_batch_get(&self, ctx: ::grpc::RpcContext, req: super::kvrpcpb::BatchGetRequest, sink: ::grpc::UnarySink<super::kvrpcpb::BatchGetResponse>);
    fn kv_batch_rollback(&self, ctx: ::grpc::RpcContext, req: super::kvrpcpb::BatchRollbackRequest, sink: ::grpc::UnarySink<super::kvrpcpb::BatchRollbackResponse>);
    fn kv_scan_lock(&self, ctx: ::grpc::RpcContext, req: super::kvrpcpb::ScanLockRequest, sink: ::grpc::UnarySink<super::kvrpcpb::ScanLockResponse>);
    fn kv_resolve_lock(&self, ctx: ::grpc::RpcContext, req: super::kvrpcpb::ResolveLockRequest, sink: ::grpc::UnarySink<super::kvrpcpb::ResolveLockResponse>);
    fn kv_gc(&self, ctx: ::grpc::RpcContext, req: super::kvrpcpb::GCRequest, sink: ::grpc::UnarySink<super::kvrpcpb::GCResponse>);
    fn raw_get(&self, ctx: ::grpc::RpcContext, req: super::kvrpcpb::RawGetRequest, sink: ::grpc::UnarySink<super::kvrpcpb::RawGetResponse>);
    fn raw_put(&self, ctx: ::grpc::RpcContext, req: super::kvrpcpb::RawPutRequest, sink: ::grpc::UnarySink<super::kvrpcpb::RawPutResponse>);
    fn raw_delete(&self, ctx: ::grpc::RpcContext, req: super::kvrpcpb::RawDeleteRequest, sink: ::grpc::UnarySink<super::kvrpcpb::RawDeleteResponse>);
    fn raw_scan(&self, ctx: ::grpc::RpcContext, req: super::kvrpcpb::RawScanRequest, sink: ::grpc::UnarySink<super::kvrpcpb::RawScanResponse>);
    fn coprocessor(&self, ctx: ::grpc::RpcContext, req: super::coprocessor::Request, sink: ::grpc::UnarySink<super::coprocessor::Response>);
    fn raft(&self, ctx: ::grpc::RpcContext, stream: ::grpc::RequestStream<super::raft_serverpb::RaftMessage>, sink: ::grpc::ClientStreamingSink<super::raft_serverpb::Done>);
    fn snapshot(&self, ctx: ::grpc::RpcContext, stream: ::grpc::RequestStream<super::raft_serverpb::SnapshotChunk>, sink: ::grpc::ClientStreamingSink<super::raft_serverpb::Done>);
    fn mvcc_get_by_key(&self, ctx: ::grpc::RpcContext, req: super::kvrpcpb::MvccGetByKeyRequest, sink: ::grpc::UnarySink<super::kvrpcpb::MvccGetByKeyResponse>);
    fn mvcc_get_by_start_ts(&self, ctx: ::grpc::RpcContext, req: super::kvrpcpb::MvccGetByStartTsRequest, sink: ::grpc::UnarySink<super::kvrpcpb::MvccGetByStartTsResponse>);
}

pub fn create_tikv<S: Tikv + Send + Clone + 'static>(s: S) -> ::grpc::Service {
    let mut builder = ::grpc::ServiceBuilder::new();
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_KV_GET, move |ctx, req, resp| {
        instance.kv_get(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_KV_SCAN, move |ctx, req, resp| {
        instance.kv_scan(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_KV_PREWRITE, move |ctx, req, resp| {
        instance.kv_prewrite(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_KV_COMMIT, move |ctx, req, resp| {
        instance.kv_commit(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_KV_IMPORT, move |ctx, req, resp| {
        instance.kv_import(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_KV_CLEANUP, move |ctx, req, resp| {
        instance.kv_cleanup(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_KV_BATCH_GET, move |ctx, req, resp| {
        instance.kv_batch_get(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_KV_BATCH_ROLLBACK, move |ctx, req, resp| {
        instance.kv_batch_rollback(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_KV_SCAN_LOCK, move |ctx, req, resp| {
        instance.kv_scan_lock(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_KV_RESOLVE_LOCK, move |ctx, req, resp| {
        instance.kv_resolve_lock(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_KV_GC, move |ctx, req, resp| {
        instance.kv_gc(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_RAW_GET, move |ctx, req, resp| {
        instance.raw_get(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_RAW_PUT, move |ctx, req, resp| {
        instance.raw_put(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_RAW_DELETE, move |ctx, req, resp| {
        instance.raw_delete(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_RAW_SCAN, move |ctx, req, resp| {
        instance.raw_scan(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_COPROCESSOR, move |ctx, req, resp| {
        instance.coprocessor(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_client_streaming_handler(&METHOD_TIKV_RAFT, move |ctx, req, resp| {
        instance.raft(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_client_streaming_handler(&METHOD_TIKV_SNAPSHOT, move |ctx, req, resp| {
        instance.snapshot(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_MVCC_GET_BY_KEY, move |ctx, req, resp| {
        instance.mvcc_get_by_key(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_MVCC_GET_BY_START_TS, move |ctx, req, resp| {
        instance.mvcc_get_by_start_ts(ctx, req, resp)
    });
    builder.build()
}
