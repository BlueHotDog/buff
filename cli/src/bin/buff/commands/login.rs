#[path = "../settings.rs"]
mod settings;
use settings::Settings;
#[path = "../buff_cli_config.rs"]
mod buff_cli_config;
use buff_cli_config::BuffCliConfig;

use std::sync::Arc;
use grpcio::{ChannelBuilder, EnvBuilder};
#[path = "../protobuffers/mod.rs"]
mod protobuffers;
use protobuffers::buff_grpc::{AuthServiceClient, AuthService};
use protobuffers::buff::{LoginRequest, LoginResponse};

pub fn execute(email: &str, password: &str) {
  let settings = Settings::new().unwrap();
  let registry_url = &settings.buff_grpc_server.url;
  let token = get_jwt_token(email, password, registry_url);
  let mut config = BuffCliConfig::new();
  config.add_registry(registry_url, &token);
  config.save();
}

fn get_jwt_token(email: &str, password: &str, grpc_server_url: &str) -> String {
  let env = Arc::new(EnvBuilder::new().build());
  let channel = ChannelBuilder::new(env).connect(grpc_server_url);
  let client = AuthServiceClient::new(channel);
  let mut req = LoginRequest::new();
  req.set_email(email.to_owned());
  req.set_password(password.to_owned());
  let reply: LoginResponse = client.login(&req).expect("Failed RPC call");
  return reply.get_token().to_string();
}