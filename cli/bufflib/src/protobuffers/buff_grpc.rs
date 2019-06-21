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

const METHOD_AUTH_SERVICE_LOGIN: ::grpcio::Method<super::buff::LoginRequest, super::buff::LoginResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/buff_server_grpc.AuthService/Login",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct AuthServiceClient {
    client: ::grpcio::Client,
}

impl AuthServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        AuthServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn login_opt(&self, req: &super::buff::LoginRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::buff::LoginResponse> {
        self.client.unary_call(&METHOD_AUTH_SERVICE_LOGIN, req, opt)
    }

    pub fn login(&self, req: &super::buff::LoginRequest) -> ::grpcio::Result<super::buff::LoginResponse> {
        self.login_opt(req, ::grpcio::CallOption::default())
    }

    pub fn login_async_opt(&self, req: &super::buff::LoginRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::buff::LoginResponse>> {
        self.client.unary_call_async(&METHOD_AUTH_SERVICE_LOGIN, req, opt)
    }

    pub fn login_async(&self, req: &super::buff::LoginRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::buff::LoginResponse>> {
        self.login_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait AuthService {
    fn login(&mut self, ctx: ::grpcio::RpcContext, req: super::buff::LoginRequest, sink: ::grpcio::UnarySink<super::buff::LoginResponse>);
}

pub fn create_auth_service<S: AuthService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_AUTH_SERVICE_LOGIN, move |ctx, req, resp| {
        instance.login(ctx, req, resp)
    });
    builder.build()
}

const METHOD_REGISTRY_SERVICE_PUBLISH: ::grpcio::Method<super::buff::PublishRequest, super::buff::PublishResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/buff_server_grpc.RegistryService/Publish",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct RegistryServiceClient {
    client: ::grpcio::Client,
}

impl RegistryServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        RegistryServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn publish_opt(&self, req: &super::buff::PublishRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::buff::PublishResponse> {
        self.client.unary_call(&METHOD_REGISTRY_SERVICE_PUBLISH, req, opt)
    }

    pub fn publish(&self, req: &super::buff::PublishRequest) -> ::grpcio::Result<super::buff::PublishResponse> {
        self.publish_opt(req, ::grpcio::CallOption::default())
    }

    pub fn publish_async_opt(&self, req: &super::buff::PublishRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::buff::PublishResponse>> {
        self.client.unary_call_async(&METHOD_REGISTRY_SERVICE_PUBLISH, req, opt)
    }

    pub fn publish_async(&self, req: &super::buff::PublishRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::buff::PublishResponse>> {
        self.publish_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait RegistryService {
    fn publish(&mut self, ctx: ::grpcio::RpcContext, req: super::buff::PublishRequest, sink: ::grpcio::UnarySink<super::buff::PublishResponse>);
}

pub fn create_registry_service<S: RegistryService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_REGISTRY_SERVICE_PUBLISH, move |ctx, req, resp| {
        instance.publish(ctx, req, resp)
    });
    builder.build()
}
