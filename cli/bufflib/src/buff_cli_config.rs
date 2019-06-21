use dirs::config_dir;
use serde::{Deserialize, Serialize};

use std::env;

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use tempdir::TempDir;

#[derive(Debug, Deserialize, Serialize)]
pub struct RegistryConfig {
  token: String,
}

impl PartialEq for RegistryConfig {
  fn eq(&self, other: &Self) -> bool {
    self.token == other.token
  }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BuffCliConfig {
  pub preferred_registry: String,
  registries: HashMap<String, RegistryConfig>,
}

pub fn get_default_registry_url() -> String {
  match std::env::var("BUFF_REGISTRY_URL") {
    Ok(s) => s,
    _ => "localhost:50051".to_string(),
  }
}

impl BuffCliConfig {
  pub fn new() -> Self {
    let path = get_config_path();
    let mut config = BuffCliConfig {
      preferred_registry: get_default_registry_url(),
      registries: HashMap::new(),
    };
    if path.exists() {
      let toml_content = fs::read_to_string(path).unwrap();
      config = toml::from_str(&toml_content).expect("Failed to parse toml");
    }
    config
  }

  pub fn add_registry(&mut self, url: &str, token: &str) {
    self.registries.insert(
      url.to_string(),
      RegistryConfig {
        token: token.to_string(),
      },
    );
  }

  pub fn save(&self) {
    let config_path = get_config_path();
    //note(itay): Annoyingly, this is how we extract the dir from a path
    //that might end with a filename.
    let config_dir = config_path.with_file_name("");
    if !config_dir.exists() {
      std::fs::create_dir(config_dir).expect("Failed to create config dir");
    }
    let toml_content = toml::to_string(self).unwrap();
    fs::write(config_path, toml_content).expect("Failed to save config");
  }
}

fn get_config_path() -> PathBuf {
  match env::var("BUFF_HOME") {
    Ok(s) => std::env::current_dir()
      .unwrap()
      .join(&s)
      .join("config.toml"),
    _ => config_dir().unwrap().join("buff/config.toml"),
  }
}

#[test]
fn should_new() {
  env::set_var("BUFF_HOME", "../tests/fixtures/buff_home");
  let config = BuffCliConfig::new();
  assert_eq!(config.preferred_registry, "localhost:50051");
  assert_eq!(config.registries.len(), 2);
  assert_eq!(
    config.registries["localhost:50051"],
    RegistryConfig {
      token: "token1".to_string()
    }
  );
  assert_eq!(
    config.registries["localhost:50052"],
    RegistryConfig {
      token: "token2".to_string()
    }
  );
}

#[test]
fn should_save() {
  let tmp_dir = TempDir::new("buff_test").unwrap();
  let url = "localhost:50051";
  let token = "newtoken";
  env::set_var("BUFF_HOME", tmp_dir.path().join("buff"));
  let mut config = BuffCliConfig::new();
  config.add_registry(url, token);
  config.save();
  config = BuffCliConfig::new();
  assert_eq!(config.registries.len(), 1);
  assert_eq!(
    config.registries[url],
    RegistryConfig {
      token: token.to_string()
    }
  );
}
