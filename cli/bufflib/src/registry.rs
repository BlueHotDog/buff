use crate::artifact;
use crate::buff_cli_config::BuffCliConfig;
use crate::package_metadata::PackageMetadata;
use crate::protobuffers::buff::PublishRequest;
use crate::protobuffers::buff::{LoginRequest, LoginResponse};
use crate::protobuffers::buff_grpc::AuthServiceClient;
use crate::protobuffers::buff_grpc::RegistryServiceClient;
use grpcio::{Channel, ChannelBuilder, EnvBuilder};
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub fn login(email: &str, password: &str) {
  let mut config = BuffCliConfig::new();
  // trace!(
  //   "Sending LoginRequest to AuthService url: {}",
  //   config.preferred_registry
  // );
  let registry_url = config.preferred_registry.clone();
  let channel = get_channel(registry_url.as_str());
  let client = AuthServiceClient::new(channel);
  let mut req = LoginRequest::new();
  req.set_email(email.to_owned());
  req.set_password(password.to_owned());
  let reply: LoginResponse = client.login(&req).expect("Failed RPC call");
  let token = reply.get_token().to_string();
  // trace!("Got JWT token from AuthService {}", token);
  // trace!("Saving token to registry in local user config...");
  config.add_registry(registry_url.as_str(), &token);
  config.save();
  // trace!("Successfully saved JWT token to local user config");
}

pub fn publish() {
  let config = BuffCliConfig::new();
  let registry_url = config.preferred_registry.clone();
  let channel = get_channel(registry_url.as_str());
  let client = RegistryServiceClient::new(channel);
  let metadata = PackageMetadata::new(&get_buff_toml_path().to_str().unwrap());
  let artifact_bytes = artifact::get_artifact_bytes(get_target_path().to_str().unwrap());
  artifact::save_artifact_to_path(get_target_path().to_str().unwrap(), "/tmp/abc");
  let mut req = PublishRequest::new();
  req.set_artifact(artifact_bytes);
  client.publish(&req).expect("Failed gRPC publish call");
}

fn get_channel(grpc_server_url: &str) -> Channel {
  let env = Arc::new(EnvBuilder::new().build());
  return ChannelBuilder::new(env).connect(grpc_server_url);
}

fn get_buff_toml_path() -> PathBuf {
  let target_path = get_target_path();
  target_path.join("buff.toml")
}

fn get_target_path() -> PathBuf {
  match std::env::var("BUFF_TARGET_PATH") {
    Ok(s) => Path::new(&s).to_path_buf(),
    _ => std::env::current_dir().unwrap(),
  }
}
