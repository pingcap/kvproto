#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommandRequestHeader {
    #[prost(uint64, tag = "1")]
    pub region_id: u64,
    #[prost(uint64, tag = "2")]
    pub index: u64,
    #[prost(uint64, tag = "3")]
    pub term: u64,
    /// Flush in-memory data to disk.
    #[prost(bool, tag = "4")]
    pub sync_log: bool,
    /// Destroy the region.
    #[prost(bool, tag = "5")]
    pub destroy: bool,
    /// Additional information for the request.
    #[prost(bytes, tag = "6")]
    pub context: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommandRequest {
    #[prost(message, optional, tag = "1")]
    pub header: ::std::option::Option<CommandRequestHeader>,
    // We don't enclose normal requests and administrator request
    // at same time.
    /// kv put / delete
    #[prost(message, repeated, tag = "2")]
    pub requests: ::std::vec::Vec<super::raft_cmdpb::Request>,
    /// region metadata manipulation command.
    #[prost(message, optional, tag = "3")]
    pub admin_request: ::std::option::Option<super::raft_cmdpb::AdminRequest>,
    /// region metadata manipulation result.
    #[prost(message, optional, tag = "4")]
    pub admin_response: ::std::option::Option<super::raft_cmdpb::AdminResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommandRequestBatch {
    #[prost(message, repeated, tag = "1")]
    pub requests: ::std::vec::Vec<CommandRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommandResponseHeader {
    #[prost(uint64, tag = "1")]
    pub region_id: u64,
    /// Region is destroyed.
    #[prost(bool, tag = "2")]
    pub destroyed: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommandResponse {
    #[prost(message, optional, tag = "1")]
    pub header: ::std::option::Option<CommandResponseHeader>,
    #[prost(message, optional, tag = "2")]
    pub apply_state: ::std::option::Option<super::raft_serverpb::RaftApplyState>,
    #[prost(uint64, tag = "3")]
    pub applied_term: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommandResponseBatch {
    #[prost(message, repeated, tag = "1")]
    pub responses: ::std::vec::Vec<CommandResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnapshotState {
    #[prost(message, optional, tag = "1")]
    pub region: ::std::option::Option<super::metapb::Region>,
    #[prost(message, optional, tag = "2")]
    pub peer: ::std::option::Option<super::metapb::Peer>,
    #[prost(message, optional, tag = "3")]
    pub apply_state: ::std::option::Option<super::raft_serverpb::RaftApplyState>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnapshotData {
    #[prost(string, tag = "1")]
    pub cf: std::string::String,
    #[prost(uint32, tag = "2")]
    pub checksum: u32,
    #[prost(message, repeated, tag = "3")]
    pub data: ::std::vec::Vec<super::raft_serverpb::KeyValue>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnapshotRequest {
    #[prost(oneof = "snapshot_request::Chunk", tags = "1, 2")]
    pub chunk: ::std::option::Option<snapshot_request::Chunk>,
}
pub mod snapshot_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Chunk {
        /// The first message for snapshots.
        /// It contains the latest region information after applied snapshot.
        #[prost(message, tag = "1")]
        State(super::SnapshotState),
        /// Following messages are always data.
        #[prost(message, tag = "2")]
        Data(super::SnapshotData),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnapshotDone {}
const METHOD_ENGINE_APPLY_COMMAND_BATCH: ::grpcio::Method<
    CommandRequestBatch,
    CommandResponseBatch,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/enginepb.Engine/ApplyCommandBatch",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
};
const METHOD_ENGINE_APPLY_SNAPSHOT: ::grpcio::Method<SnapshotRequest, SnapshotDone> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::ClientStreaming,
        name: "/enginepb.Engine/ApplySnapshot",
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
pub struct EngineClient {
    client: ::grpcio::Client,
}
impl EngineClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        EngineClient {
            client: ::grpcio::Client::new(channel),
        }
    }
    pub fn apply_command_batch_opt(
        &self,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<(
        ::grpcio::ClientDuplexSender<CommandRequestBatch>,
        ::grpcio::ClientDuplexReceiver<CommandResponseBatch>,
    )> {
        self.client
            .duplex_streaming(&METHOD_ENGINE_APPLY_COMMAND_BATCH, opt)
    }
    pub fn apply_command_batch(
        &self,
    ) -> ::grpcio::Result<(
        ::grpcio::ClientDuplexSender<CommandRequestBatch>,
        ::grpcio::ClientDuplexReceiver<CommandResponseBatch>,
    )> {
        self.apply_command_batch_opt(::grpcio::CallOption::default())
    }
    pub fn apply_snapshot_opt(
        &self,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<(
        ::grpcio::ClientCStreamSender<SnapshotRequest>,
        ::grpcio::ClientCStreamReceiver<SnapshotDone>,
    )> {
        self.client
            .client_streaming(&METHOD_ENGINE_APPLY_SNAPSHOT, opt)
    }
    pub fn apply_snapshot(
        &self,
    ) -> ::grpcio::Result<(
        ::grpcio::ClientCStreamSender<SnapshotRequest>,
        ::grpcio::ClientCStreamReceiver<SnapshotDone>,
    )> {
        self.apply_snapshot_opt(::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F)
    where
        F: ::futures::Future<Item = (), Error = ()> + Send + 'static,
    {
        self.client.spawn(f)
    }
}
pub trait Engine {
    fn apply_command_batch(
        &mut self,
        ctx: ::grpcio::RpcContext,
        stream: ::grpcio::RequestStream<CommandRequestBatch>,
        sink: ::grpcio::DuplexSink<CommandResponseBatch>,
    );
    fn apply_snapshot(
        &mut self,
        ctx: ::grpcio::RpcContext,
        stream: ::grpcio::RequestStream<SnapshotRequest>,
        sink: ::grpcio::ClientStreamingSink<SnapshotDone>,
    );
}
pub fn create_engine<S: Engine + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder
        .add_duplex_streaming_handler(&METHOD_ENGINE_APPLY_COMMAND_BATCH, move |ctx, req, resp| {
            instance.apply_command_batch(ctx, req, resp)
        });
    let mut instance = s.clone();
    builder = builder
        .add_client_streaming_handler(&METHOD_ENGINE_APPLY_SNAPSHOT, move |ctx, req, resp| {
            instance.apply_snapshot(ctx, req, resp)
        });
    builder.build()
}
