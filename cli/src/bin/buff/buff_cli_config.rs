use std::collections::HashMap;
use tempdir::TempDir;
use std::env;
use std::path::{Path, PathBuf};
use std::fs;
use serde::{Deserialize, Serialize};
use dirs::config_dir;

#[derive(Debug, Deserialize, Serialize)]
pub struct Registry {
  token: String,
}

impl PartialEq for Registry {
  fn eq(&self, other: &Self) -> bool {
    self.token == other.token
  }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BuffCliConfig {
  registries: HashMap<String, Registry>,
}

impl BuffCliConfig {
  pub fn new() -> Self {
    let path = get_config_path();
    let mut config = BuffCliConfig { registries: HashMap::new() };
    if path.exists() {
      let toml_content = fs::read_to_string(path).unwrap();
      config = toml::from_str(&toml_content).expect("Failed to parse toml");
    }
    config
  }

  pub fn add_registry(&mut self, url: &str, token: &str) {
    self.registries.insert(url.to_string(), Registry { token: token.to_string() });
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
    Ok(s) => Path::new(&s).join("config.toml"),
    _ => config_dir().unwrap().join("buff/config.toml")
  }
}

#[test]
fn should_new() {
  env::set_var("BUFF_HOME", "./tests/fixtures/buff_home");
  let config = BuffCliConfig::new();
  assert_eq!(config.registries.len(), 2);
  assert_eq!(config.registries["localhost:50051"], Registry { token: "token1".to_string() });
  assert_eq!(config.registries["localhost:50052"], Registry { token: "token2".to_string() });
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
  assert_eq!(config.registries[url], Registry { token: token.to_string() });
}