#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwitchModeRequest {
    #[prost(string, tag = "1")]
    pub pd_addr: std::string::String,
    #[prost(message, optional, tag = "2")]
    pub request: ::std::option::Option<super::import_sstpb::SwitchModeRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwitchModeResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenEngineRequest {
    #[prost(bytes, tag = "1")]
    pub uuid: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenEngineResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteHead {
    #[prost(bytes, tag = "1")]
    pub uuid: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Mutation {
    #[prost(enumeration = "mutation::Op", tag = "1")]
    pub op: i32,
    #[prost(bytes, tag = "2")]
    pub key: std::vec::Vec<u8>,
    #[prost(bytes, tag = "3")]
    pub value: std::vec::Vec<u8>,
}
pub mod mutation {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Op {
        Put = 0,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteBatch {
    #[prost(uint64, tag = "1")]
    pub commit_ts: u64,
    #[prost(message, repeated, tag = "2")]
    pub mutations: ::std::vec::Vec<Mutation>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteEngineRequest {
    #[prost(oneof = "write_engine_request::Chunk", tags = "1, 2")]
    pub chunk: ::std::option::Option<write_engine_request::Chunk>,
}
pub mod write_engine_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Chunk {
        #[prost(message, tag = "1")]
        Head(super::WriteHead),
        #[prost(message, tag = "2")]
        Batch(super::WriteBatch),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteEngineResponse {
    #[prost(message, optional, tag = "1")]
    pub error: ::std::option::Option<Error>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloseEngineRequest {
    #[prost(bytes, tag = "1")]
    pub uuid: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloseEngineResponse {
    #[prost(message, optional, tag = "1")]
    pub error: ::std::option::Option<Error>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportEngineRequest {
    #[prost(bytes, tag = "1")]
    pub uuid: std::vec::Vec<u8>,
    #[prost(string, tag = "2")]
    pub pd_addr: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportEngineResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CleanupEngineRequest {
    #[prost(bytes, tag = "1")]
    pub uuid: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CleanupEngineResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompactClusterRequest {
    #[prost(string, tag = "1")]
    pub pd_addr: std::string::String,
    #[prost(message, optional, tag = "2")]
    pub request: ::std::option::Option<super::import_sstpb::CompactRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompactClusterResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Error {
    /// This can happen if the client hasn't opened the engine, or the server
    /// restarts while the client is writing or closing. An unclosed engine will
    /// be removed on server restart, so the client should not continue but
    /// restart the previous job in that case.
    #[prost(message, optional, tag = "1")]
    pub engine_not_found: ::std::option::Option<error::EngineNotFound>,
}
pub mod error {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EngineNotFound {
        #[prost(bytes, tag = "1")]
        pub uuid: std::vec::Vec<u8>,
    }
}
const METHOD_IMPORT_KV_SWITCH_MODE: ::grpcio::Method<SwitchModeRequest, SwitchModeResponse> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::Unary,
        name: "/import_kvpb.ImportKV/SwitchMode",
        req_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pr_ser,
            de: ::grpcio::pr_de,
        },
        resp_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pr_ser,
            de: ::grpcio::pr_de,
        },
    };
const METHOD_IMPORT_KV_OPEN_ENGINE: ::grpcio::Method<OpenEngineRequest, OpenEngineResponse> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::Unary,
        name: "/import_kvpb.ImportKV/OpenEngine",
        req_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pr_ser,
            de: ::grpcio::pr_de,
        },
        resp_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pr_ser,
            de: ::grpcio::pr_de,
        },
    };
const METHOD_IMPORT_KV_WRITE_ENGINE: ::grpcio::Method<WriteEngineRequest, WriteEngineResponse> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::ClientStreaming,
        name: "/import_kvpb.ImportKV/WriteEngine",
        req_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pr_ser,
            de: ::grpcio::pr_de,
        },
        resp_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pr_ser,
            de: ::grpcio::pr_de,
        },
    };
const METHOD_IMPORT_KV_CLOSE_ENGINE: ::grpcio::Method<CloseEngineRequest, CloseEngineResponse> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::Unary,
        name: "/import_kvpb.ImportKV/CloseEngine",
        req_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pr_ser,
            de: ::grpcio::pr_de,
        },
        resp_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pr_ser,
            de: ::grpcio::pr_de,
        },
    };
const METHOD_IMPORT_KV_IMPORT_ENGINE: ::grpcio::Method<ImportEngineRequest, ImportEngineResponse> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::Unary,
        name: "/import_kvpb.ImportKV/ImportEngine",
        req_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pr_ser,
            de: ::grpcio::pr_de,
        },
        resp_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pr_ser,
            de: ::grpcio::pr_de,
        },
    };
const METHOD_IMPORT_KV_CLEANUP_ENGINE: ::grpcio::Method<
    CleanupEngineRequest,
    CleanupEngineResponse,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/import_kvpb.ImportKV/CleanupEngine",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
};
const METHOD_IMPORT_KV_COMPACT_CLUSTER: ::grpcio::Method<
    CompactClusterRequest,
    CompactClusterResponse,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/import_kvpb.ImportKV/CompactCluster",
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
pub struct ImportKvClient {
    client: ::grpcio::Client,
}
impl ImportKvClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        ImportKvClient {
            client: ::grpcio::Client::new(channel),
        }
    }
    pub fn switch_mode_opt(
        &self,
        req: &SwitchModeRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<SwitchModeResponse> {
        self.client
            .unary_call(&METHOD_IMPORT_KV_SWITCH_MODE, req, opt)
    }
    pub fn switch_mode(&self, req: &SwitchModeRequest) -> ::grpcio::Result<SwitchModeResponse> {
        self.switch_mode_opt(req, ::grpcio::CallOption::default())
    }
    pub fn switch_mode_async_opt(
        &self,
        req: &SwitchModeRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<SwitchModeResponse>> {
        self.client
            .unary_call_async(&METHOD_IMPORT_KV_SWITCH_MODE, req, opt)
    }
    pub fn switch_mode_async(
        &self,
        req: &SwitchModeRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<SwitchModeResponse>> {
        self.switch_mode_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn open_engine_opt(
        &self,
        req: &OpenEngineRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<OpenEngineResponse> {
        self.client
            .unary_call(&METHOD_IMPORT_KV_OPEN_ENGINE, req, opt)
    }
    pub fn open_engine(&self, req: &OpenEngineRequest) -> ::grpcio::Result<OpenEngineResponse> {
        self.open_engine_opt(req, ::grpcio::CallOption::default())
    }
    pub fn open_engine_async_opt(
        &self,
        req: &OpenEngineRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<OpenEngineResponse>> {
        self.client
            .unary_call_async(&METHOD_IMPORT_KV_OPEN_ENGINE, req, opt)
    }
    pub fn open_engine_async(
        &self,
        req: &OpenEngineRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<OpenEngineResponse>> {
        self.open_engine_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn write_engine_opt(
        &self,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<(
        ::grpcio::ClientCStreamSender<WriteEngineRequest>,
        ::grpcio::ClientCStreamReceiver<WriteEngineResponse>,
    )> {
        self.client
            .client_streaming(&METHOD_IMPORT_KV_WRITE_ENGINE, opt)
    }
    pub fn write_engine(
        &self,
    ) -> ::grpcio::Result<(
        ::grpcio::ClientCStreamSender<WriteEngineRequest>,
        ::grpcio::ClientCStreamReceiver<WriteEngineResponse>,
    )> {
        self.write_engine_opt(::grpcio::CallOption::default())
    }
    pub fn close_engine_opt(
        &self,
        req: &CloseEngineRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<CloseEngineResponse> {
        self.client
            .unary_call(&METHOD_IMPORT_KV_CLOSE_ENGINE, req, opt)
    }
    pub fn close_engine(&self, req: &CloseEngineRequest) -> ::grpcio::Result<CloseEngineResponse> {
        self.close_engine_opt(req, ::grpcio::CallOption::default())
    }
    pub fn close_engine_async_opt(
        &self,
        req: &CloseEngineRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<CloseEngineResponse>> {
        self.client
            .unary_call_async(&METHOD_IMPORT_KV_CLOSE_ENGINE, req, opt)
    }
    pub fn close_engine_async(
        &self,
        req: &CloseEngineRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<CloseEngineResponse>> {
        self.close_engine_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn import_engine_opt(
        &self,
        req: &ImportEngineRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<ImportEngineResponse> {
        self.client
            .unary_call(&METHOD_IMPORT_KV_IMPORT_ENGINE, req, opt)
    }
    pub fn import_engine(
        &self,
        req: &ImportEngineRequest,
    ) -> ::grpcio::Result<ImportEngineResponse> {
        self.import_engine_opt(req, ::grpcio::CallOption::default())
    }
    pub fn import_engine_async_opt(
        &self,
        req: &ImportEngineRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<ImportEngineResponse>> {
        self.client
            .unary_call_async(&METHOD_IMPORT_KV_IMPORT_ENGINE, req, opt)
    }
    pub fn import_engine_async(
        &self,
        req: &ImportEngineRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<ImportEngineResponse>> {
        self.import_engine_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn cleanup_engine_opt(
        &self,
        req: &CleanupEngineRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<CleanupEngineResponse> {
        self.client
            .unary_call(&METHOD_IMPORT_KV_CLEANUP_ENGINE, req, opt)
    }
    pub fn cleanup_engine(
        &self,
        req: &CleanupEngineRequest,
    ) -> ::grpcio::Result<CleanupEngineResponse> {
        self.cleanup_engine_opt(req, ::grpcio::CallOption::default())
    }
    pub fn cleanup_engine_async_opt(
        &self,
        req: &CleanupEngineRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<CleanupEngineResponse>> {
        self.client
            .unary_call_async(&METHOD_IMPORT_KV_CLEANUP_ENGINE, req, opt)
    }
    pub fn cleanup_engine_async(
        &self,
        req: &CleanupEngineRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<CleanupEngineResponse>> {
        self.cleanup_engine_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn compact_cluster_opt(
        &self,
        req: &CompactClusterRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<CompactClusterResponse> {
        self.client
            .unary_call(&METHOD_IMPORT_KV_COMPACT_CLUSTER, req, opt)
    }
    pub fn compact_cluster(
        &self,
        req: &CompactClusterRequest,
    ) -> ::grpcio::Result<CompactClusterResponse> {
        self.compact_cluster_opt(req, ::grpcio::CallOption::default())
    }
    pub fn compact_cluster_async_opt(
        &self,
        req: &CompactClusterRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<CompactClusterResponse>> {
        self.client
            .unary_call_async(&METHOD_IMPORT_KV_COMPACT_CLUSTER, req, opt)
    }
    pub fn compact_cluster_async(
        &self,
        req: &CompactClusterRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<CompactClusterResponse>> {
        self.compact_cluster_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F)
    where
        F: ::futures::Future<Item = (), Error = ()> + Send + 'static,
    {
        self.client.spawn(f)
    }
}
pub trait ImportKv {
    fn switch_mode(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: SwitchModeRequest,
        sink: ::grpcio::UnarySink<SwitchModeResponse>,
    );
    fn open_engine(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: OpenEngineRequest,
        sink: ::grpcio::UnarySink<OpenEngineResponse>,
    );
    fn write_engine(
        &mut self,
        ctx: ::grpcio::RpcContext,
        stream: ::grpcio::RequestStream<WriteEngineRequest>,
        sink: ::grpcio::ClientStreamingSink<WriteEngineResponse>,
    );
    fn close_engine(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: CloseEngineRequest,
        sink: ::grpcio::UnarySink<CloseEngineResponse>,
    );
    fn import_engine(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: ImportEngineRequest,
        sink: ::grpcio::UnarySink<ImportEngineResponse>,
    );
    fn cleanup_engine(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: CleanupEngineRequest,
        sink: ::grpcio::UnarySink<CleanupEngineResponse>,
    );
    fn compact_cluster(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: CompactClusterRequest,
        sink: ::grpcio::UnarySink<CompactClusterResponse>,
    );
}
pub fn create_import_kv<S: ImportKv + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_IMPORT_KV_SWITCH_MODE, move |ctx, req, resp| {
        instance.switch_mode(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_IMPORT_KV_OPEN_ENGINE, move |ctx, req, resp| {
        instance.open_engine(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder
        .add_client_streaming_handler(&METHOD_IMPORT_KV_WRITE_ENGINE, move |ctx, req, resp| {
            instance.write_engine(ctx, req, resp)
        });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_IMPORT_KV_CLOSE_ENGINE, move |ctx, req, resp| {
        instance.close_engine(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_IMPORT_KV_IMPORT_ENGINE, move |ctx, req, resp| {
        instance.import_engine(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_IMPORT_KV_CLEANUP_ENGINE, move |ctx, req, resp| {
        instance.cleanup_engine(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder
        .add_unary_handler(&METHOD_IMPORT_KV_COMPACT_CLUSTER, move |ctx, req, resp| {
            instance.compact_cluster(ctx, req, resp)
        });
    builder.build()
}
