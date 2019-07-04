// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

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

const METHOD_DEADLOCK_GET_WAIT_FOR_ENTRIES: ::grpcio::Method<super::deadlock::WaitForEntriesRequest, super::deadlock::WaitForEntriesResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/deadlockpb.Deadlock/GetWaitForEntries",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_DEADLOCK_DETECT: ::grpcio::Method<super::deadlock::DeadlockRequest, super::deadlock::DeadlockResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/deadlockpb.Deadlock/Detect",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
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

    pub fn get_wait_for_entries_opt(&self, req: &super::deadlock::WaitForEntriesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::deadlock::WaitForEntriesResponse> {
        self.client.unary_call(&METHOD_DEADLOCK_GET_WAIT_FOR_ENTRIES, req, opt)
    }

    pub fn get_wait_for_entries(&self, req: &super::deadlock::WaitForEntriesRequest) -> ::grpcio::Result<super::deadlock::WaitForEntriesResponse> {
        self.get_wait_for_entries_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_wait_for_entries_async_opt(&self, req: &super::deadlock::WaitForEntriesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::deadlock::WaitForEntriesResponse>> {
        self.client.unary_call_async(&METHOD_DEADLOCK_GET_WAIT_FOR_ENTRIES, req, opt)
    }

    pub fn get_wait_for_entries_async(&self, req: &super::deadlock::WaitForEntriesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::deadlock::WaitForEntriesResponse>> {
        self.get_wait_for_entries_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn detect_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::deadlock::DeadlockRequest>, ::grpcio::ClientDuplexReceiver<super::deadlock::DeadlockResponse>)> {
        self.client.duplex_streaming(&METHOD_DEADLOCK_DETECT, opt)
    }

    pub fn detect(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::deadlock::DeadlockRequest>, ::grpcio::ClientDuplexReceiver<super::deadlock::DeadlockResponse>)> {
        self.detect_opt(::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Deadlock {
    fn get_wait_for_entries(&mut self, ctx: ::grpcio::RpcContext, req: super::deadlock::WaitForEntriesRequest, sink: ::grpcio::UnarySink<super::deadlock::WaitForEntriesResponse>);
    fn detect(&mut self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::deadlock::DeadlockRequest>, sink: ::grpcio::DuplexSink<super::deadlock::DeadlockResponse>);
}

pub fn create_deadlock<S: Deadlock + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_DEADLOCK_GET_WAIT_FOR_ENTRIES, move |ctx, req, resp| {
        instance.get_wait_for_entries(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_duplex_streaming_handler(&METHOD_DEADLOCK_DETECT, move |ctx, req, resp| {
        instance.detect(ctx, req, resp)
    });
    builder.build()
}
