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

const METHOD_BACKUP_BACKUP: ::grpcio::Method<super::backup::BackupRequest, super::backup::BackupResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/backup.Backup/backup",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct BackupClient {
    client: ::grpcio::Client,
}

impl BackupClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        BackupClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn backup_opt(&self, req: &super::backup::BackupRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::backup::BackupResponse>> {
        self.client.server_streaming(&METHOD_BACKUP_BACKUP, req, opt)
    }

    pub fn backup(&self, req: &super::backup::BackupRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::backup::BackupResponse>> {
        self.backup_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Backup {
    fn backup(&mut self, ctx: ::grpcio::RpcContext, req: super::backup::BackupRequest, sink: ::grpcio::ServerStreamingSink<super::backup::BackupResponse>);
}

pub fn create_backup<S: Backup + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_BACKUP_BACKUP, move |ctx, req, resp| {
        instance.backup(ctx, req, resp)
    });
    builder.build()
}
