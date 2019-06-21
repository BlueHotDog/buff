use std::env;
use config::{ConfigError, Config, File, Environment};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BuffGrpcServer {
  pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
  pub buff_grpc_server: BuffGrpcServer,
}

impl Settings {
  pub fn new() -> Result<Self, ConfigError> {
    let mut s = Config::new();

    // Start off by merging in the "default" configuration file
    s.merge(File::with_name("config/default"))?;

    // Add in the current environment file
    // Default to 'development' env
    // Note that this file is _optional_
    let env = env::var("RUN_MODE").unwrap_or("development".into());
    s.merge(File::with_name(&format!("config/{}", env)).required(false))?;

    // Add in a local configuration file
    // This file shouldn't be checked in to git
    s.merge(File::with_name("config/local").required(false))?;

    // Add in settings from the environment (with a prefix of APP)
    // Eg.. `BUFF_DEBUG=1 ./target/buff` would set the `debug` key
    s.merge(Environment::with_prefix("buff"))?;

    // You can deserialize (and thus freeze) the entire configuration as
    s.try_into()
  }
}