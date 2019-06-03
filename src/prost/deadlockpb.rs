#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WaitForEntriesRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WaitForEntriesResponse {
    #[prost(message, repeated, tag = "1")]
    pub entries: ::std::vec::Vec<WaitForEntry>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WaitForEntry {
    /// The transaction id that is waiting.
    #[prost(uint64, tag = "1")]
    pub txn: u64,
    /// The transaction id that is being waited for.
    #[prost(uint64, tag = "2")]
    pub wait_for_txn: u64,
    /// The hash value of the key is being waited for.
    #[prost(uint64, tag = "3")]
    pub key_hash: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeadlockRequest {
    #[prost(enumeration = "DeadlockRequestType", tag = "1")]
    pub tp: i32,
    #[prost(message, optional, tag = "2")]
    pub entry: ::std::option::Option<WaitForEntry>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeadlockResponse {
    /// The same entry sent by DeadlockRequest, identifies the sender.
    #[prost(message, optional, tag = "1")]
    pub entry: ::std::option::Option<WaitForEntry>,
    /// The key hash of the lock that is hold by the waiting transaction.
    #[prost(uint64, tag = "2")]
    pub deadlock_key_hash: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DeadlockRequestType {
    Detect = 0,
    /// CleanUpWaitFor cleans a single entry the transaction is waiting.
    CleanUpWaitFor = 1,
    /// CleanUp cleans all entries the transaction is waiting.
    CleanUp = 2,
}
const METHOD_DEADLOCK_GET_WAIT_FOR_ENTRIES: ::grpcio::Method<
    WaitForEntriesRequest,
    WaitForEntriesResponse,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/deadlockpb.Deadlock/GetWaitForEntries",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
};
const METHOD_DEADLOCK_DETECT: ::grpcio::Method<DeadlockRequest, DeadlockResponse> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::Duplex,
        name: "/deadlockpb.Deadlock/Detect",
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
pub struct DeadlockClient {
    client: ::grpcio::Client,
}
impl DeadlockClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        DeadlockClient {
            client: ::grpcio::Client::new(channel),
        }
    }
    pub fn get_wait_for_entries_opt(
        &self,
        req: &WaitForEntriesRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<WaitForEntriesResponse> {
        self.client
            .unary_call(&METHOD_DEADLOCK_GET_WAIT_FOR_ENTRIES, req, opt)
    }
    pub fn get_wait_for_entries(
        &self,
        req: &WaitForEntriesRequest,
    ) -> ::grpcio::Result<WaitForEntriesResponse> {
        self.get_wait_for_entries_opt(req, ::grpcio::CallOption::default())
    }
    pub fn get_wait_for_entries_async_opt(
        &self,
        req: &WaitForEntriesRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<WaitForEntriesResponse>> {
        self.client
            .unary_call_async(&METHOD_DEADLOCK_GET_WAIT_FOR_ENTRIES, req, opt)
    }
    pub fn get_wait_for_entries_async(
        &self,
        req: &WaitForEntriesRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<WaitForEntriesResponse>> {
        self.get_wait_for_entries_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn detect_opt(
        &self,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<(
        ::grpcio::ClientDuplexSender<DeadlockRequest>,
        ::grpcio::ClientDuplexReceiver<DeadlockResponse>,
    )> {
        self.client.duplex_streaming(&METHOD_DEADLOCK_DETECT, opt)
    }
    pub fn detect(
        &self,
    ) -> ::grpcio::Result<(
        ::grpcio::ClientDuplexSender<DeadlockRequest>,
        ::grpcio::ClientDuplexReceiver<DeadlockResponse>,
    )> {
        self.detect_opt(::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F)
    where
        F: ::futures::Future<Item = (), Error = ()> + Send + 'static,
    {
        self.client.spawn(f)
    }
}
pub trait Deadlock {
    fn get_wait_for_entries(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: WaitForEntriesRequest,
        sink: ::grpcio::UnarySink<WaitForEntriesResponse>,
    );
    fn detect(
        &mut self,
        ctx: ::grpcio::RpcContext,
        stream: ::grpcio::RequestStream<DeadlockRequest>,
        sink: ::grpcio::DuplexSink<DeadlockResponse>,
    );
}
pub fn create_deadlock<S: Deadlock + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_DEADLOCK_GET_WAIT_FOR_ENTRIES,
        move |ctx, req, resp| instance.get_wait_for_entries(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder
        .add_duplex_streaming_handler(&METHOD_DEADLOCK_DETECT, move |ctx, req, resp| {
            instance.detect(ctx, req, resp)
        });
    builder.build()
}
