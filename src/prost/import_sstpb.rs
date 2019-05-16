#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwitchModeRequest {
    #[prost(enumeration = "SwitchMode", tag = "1")]
    pub mode: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwitchModeResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Range {
    #[prost(bytes, tag = "1")]
    pub start: std::vec::Vec<u8>,
    #[prost(bytes, tag = "2")]
    pub end: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SstMeta {
    #[prost(bytes, tag = "1")]
    pub uuid: std::vec::Vec<u8>,
    #[prost(message, optional, tag = "2")]
    pub range: ::std::option::Option<Range>,
    #[prost(uint32, tag = "3")]
    pub crc32: u32,
    #[prost(uint64, tag = "4")]
    pub length: u64,
    #[prost(string, tag = "5")]
    pub cf_name: std::string::String,
    #[prost(uint64, tag = "6")]
    pub region_id: u64,
    #[prost(message, optional, tag = "7")]
    pub region_epoch: ::std::option::Option<super::metapb::RegionEpoch>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadRequest {
    #[prost(oneof = "upload_request::Chunk", tags = "1, 2")]
    pub chunk: ::std::option::Option<upload_request::Chunk>,
}
pub mod upload_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Chunk {
        #[prost(message, tag = "1")]
        Meta(super::SstMeta),
        #[prost(bytes, tag = "2")]
        Data(std::vec::Vec<u8>),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IngestRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::std::option::Option<super::kvrpcpb::Context>,
    #[prost(message, optional, tag = "2")]
    pub sst: ::std::option::Option<SstMeta>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IngestResponse {
    #[prost(message, optional, tag = "1")]
    pub error: ::std::option::Option<super::errorpb::Error>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompactRequest {
    /// Compact files in the range and above the output level.
    /// Compact all files if the range is not specified.
    /// Compact all files to the bottommost level if the output level is -1.
    #[prost(message, optional, tag = "1")]
    pub range: ::std::option::Option<Range>,
    #[prost(int32, tag = "2")]
    pub output_level: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompactResponse {}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SwitchMode {
    Normal = 0,
    Import = 1,
}
const METHOD_IMPORT_SST_SWITCH_MODE: ::grpcio::Method<SwitchModeRequest, SwitchModeResponse> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::Unary,
        name: "/import_sstpb.ImportSST/SwitchMode",
        req_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pr_ser,
            de: ::grpcio::pr_de,
        },
        resp_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pr_ser,
            de: ::grpcio::pr_de,
        },
    };
const METHOD_IMPORT_SST_UPLOAD: ::grpcio::Method<UploadRequest, UploadResponse> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::ClientStreaming,
        name: "/import_sstpb.ImportSST/Upload",
        req_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pr_ser,
            de: ::grpcio::pr_de,
        },
        resp_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pr_ser,
            de: ::grpcio::pr_de,
        },
    };
const METHOD_IMPORT_SST_INGEST: ::grpcio::Method<IngestRequest, IngestResponse> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::Unary,
        name: "/import_sstpb.ImportSST/Ingest",
        req_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pr_ser,
            de: ::grpcio::pr_de,
        },
        resp_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pr_ser,
            de: ::grpcio::pr_de,
        },
    };
const METHOD_IMPORT_SST_COMPACT: ::grpcio::Method<CompactRequest, CompactResponse> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::Unary,
        name: "/import_sstpb.ImportSST/Compact",
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
pub struct ImportSstClient {
    client: ::grpcio::Client,
}
impl ImportSstClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        ImportSstClient {
            client: ::grpcio::Client::new(channel),
        }
    }
    pub fn switch_mode_opt(
        &self,
        req: &SwitchModeRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<SwitchModeResponse> {
        self.client
            .unary_call(&METHOD_IMPORT_SST_SWITCH_MODE, req, opt)
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
            .unary_call_async(&METHOD_IMPORT_SST_SWITCH_MODE, req, opt)
    }
    pub fn switch_mode_async(
        &self,
        req: &SwitchModeRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<SwitchModeResponse>> {
        self.switch_mode_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn upload_opt(
        &self,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<(
        ::grpcio::ClientCStreamSender<UploadRequest>,
        ::grpcio::ClientCStreamReceiver<UploadResponse>,
    )> {
        self.client.client_streaming(&METHOD_IMPORT_SST_UPLOAD, opt)
    }
    pub fn upload(
        &self,
    ) -> ::grpcio::Result<(
        ::grpcio::ClientCStreamSender<UploadRequest>,
        ::grpcio::ClientCStreamReceiver<UploadResponse>,
    )> {
        self.upload_opt(::grpcio::CallOption::default())
    }
    pub fn ingest_opt(
        &self,
        req: &IngestRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<IngestResponse> {
        self.client.unary_call(&METHOD_IMPORT_SST_INGEST, req, opt)
    }
    pub fn ingest(&self, req: &IngestRequest) -> ::grpcio::Result<IngestResponse> {
        self.ingest_opt(req, ::grpcio::CallOption::default())
    }
    pub fn ingest_async_opt(
        &self,
        req: &IngestRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<IngestResponse>> {
        self.client
            .unary_call_async(&METHOD_IMPORT_SST_INGEST, req, opt)
    }
    pub fn ingest_async(
        &self,
        req: &IngestRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<IngestResponse>> {
        self.ingest_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn compact_opt(
        &self,
        req: &CompactRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<CompactResponse> {
        self.client.unary_call(&METHOD_IMPORT_SST_COMPACT, req, opt)
    }
    pub fn compact(&self, req: &CompactRequest) -> ::grpcio::Result<CompactResponse> {
        self.compact_opt(req, ::grpcio::CallOption::default())
    }
    pub fn compact_async_opt(
        &self,
        req: &CompactRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<CompactResponse>> {
        self.client
            .unary_call_async(&METHOD_IMPORT_SST_COMPACT, req, opt)
    }
    pub fn compact_async(
        &self,
        req: &CompactRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<CompactResponse>> {
        self.compact_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F)
    where
        F: ::futures::Future<Item = (), Error = ()> + Send + 'static,
    {
        self.client.spawn(f)
    }
}
pub trait ImportSst {
    fn switch_mode(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: SwitchModeRequest,
        sink: ::grpcio::UnarySink<SwitchModeResponse>,
    );
    fn upload(
        &mut self,
        ctx: ::grpcio::RpcContext,
        stream: ::grpcio::RequestStream<UploadRequest>,
        sink: ::grpcio::ClientStreamingSink<UploadResponse>,
    );
    fn ingest(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: IngestRequest,
        sink: ::grpcio::UnarySink<IngestResponse>,
    );
    fn compact(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: CompactRequest,
        sink: ::grpcio::UnarySink<CompactResponse>,
    );
}
pub fn create_import_sst<S: ImportSst + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_IMPORT_SST_SWITCH_MODE, move |ctx, req, resp| {
        instance.switch_mode(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder
        .add_client_streaming_handler(&METHOD_IMPORT_SST_UPLOAD, move |ctx, req, resp| {
            instance.upload(ctx, req, resp)
        });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_IMPORT_SST_INGEST, move |ctx, req, resp| {
        instance.ingest(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_IMPORT_SST_COMPACT, move |ctx, req, resp| {
        instance.compact(ctx, req, resp)
    });
    builder.build()
}
