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

const METHOD_CONFIG_CREATE: ::grpcio::Method<super::configpb::CreateRequest, super::configpb::CreateResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/configpb.Config/Create",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CONFIG_GET: ::grpcio::Method<super::configpb::GetRequest, super::configpb::GetResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/configpb.Config/Get",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CONFIG_UPDATE: ::grpcio::Method<super::configpb::UpdateRequest, super::configpb::UpdateResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/configpb.Config/Update",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CONFIG_DELETE: ::grpcio::Method<super::configpb::DeleteRequest, super::configpb::DeleteResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/configpb.Config/Delete",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct ConfigClient {
    client: ::grpcio::Client,
}

impl ConfigClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        ConfigClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn create_opt(&self, req: &super::configpb::CreateRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::configpb::CreateResponse> {
        self.client.unary_call(&METHOD_CONFIG_CREATE, req, opt)
    }

    pub fn create(&self, req: &super::configpb::CreateRequest) -> ::grpcio::Result<super::configpb::CreateResponse> {
        self.create_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_async_opt(&self, req: &super::configpb::CreateRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::configpb::CreateResponse>> {
        self.client.unary_call_async(&METHOD_CONFIG_CREATE, req, opt)
    }

    pub fn create_async(&self, req: &super::configpb::CreateRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::configpb::CreateResponse>> {
        self.create_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_opt(&self, req: &super::configpb::GetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::configpb::GetResponse> {
        self.client.unary_call(&METHOD_CONFIG_GET, req, opt)
    }

    pub fn get(&self, req: &super::configpb::GetRequest) -> ::grpcio::Result<super::configpb::GetResponse> {
        self.get_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_async_opt(&self, req: &super::configpb::GetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::configpb::GetResponse>> {
        self.client.unary_call_async(&METHOD_CONFIG_GET, req, opt)
    }

    pub fn get_async(&self, req: &super::configpb::GetRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::configpb::GetResponse>> {
        self.get_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_opt(&self, req: &super::configpb::UpdateRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::configpb::UpdateResponse> {
        self.client.unary_call(&METHOD_CONFIG_UPDATE, req, opt)
    }

    pub fn update(&self, req: &super::configpb::UpdateRequest) -> ::grpcio::Result<super::configpb::UpdateResponse> {
        self.update_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_async_opt(&self, req: &super::configpb::UpdateRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::configpb::UpdateResponse>> {
        self.client.unary_call_async(&METHOD_CONFIG_UPDATE, req, opt)
    }

    pub fn update_async(&self, req: &super::configpb::UpdateRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::configpb::UpdateResponse>> {
        self.update_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_opt(&self, req: &super::configpb::DeleteRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::configpb::DeleteResponse> {
        self.client.unary_call(&METHOD_CONFIG_DELETE, req, opt)
    }

    pub fn delete(&self, req: &super::configpb::DeleteRequest) -> ::grpcio::Result<super::configpb::DeleteResponse> {
        self.delete_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_async_opt(&self, req: &super::configpb::DeleteRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::configpb::DeleteResponse>> {
        self.client.unary_call_async(&METHOD_CONFIG_DELETE, req, opt)
    }

    pub fn delete_async(&self, req: &super::configpb::DeleteRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::configpb::DeleteResponse>> {
        self.delete_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Config {
    fn create(&mut self, ctx: ::grpcio::RpcContext, req: super::configpb::CreateRequest, sink: ::grpcio::UnarySink<super::configpb::CreateResponse>);
    fn get(&mut self, ctx: ::grpcio::RpcContext, req: super::configpb::GetRequest, sink: ::grpcio::UnarySink<super::configpb::GetResponse>);
    fn update(&mut self, ctx: ::grpcio::RpcContext, req: super::configpb::UpdateRequest, sink: ::grpcio::UnarySink<super::configpb::UpdateResponse>);
    fn delete(&mut self, ctx: ::grpcio::RpcContext, req: super::configpb::DeleteRequest, sink: ::grpcio::UnarySink<super::configpb::DeleteResponse>);
}

pub fn create_config<S: Config + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CONFIG_CREATE, move |ctx, req, resp| {
        instance.create(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CONFIG_GET, move |ctx, req, resp| {
        instance.get(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CONFIG_UPDATE, move |ctx, req, resp| {
        instance.update(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CONFIG_DELETE, move |ctx, req, resp| {
        instance.delete(ctx, req, resp)
    });
    builder.build()
}
