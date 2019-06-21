use std::collections::HashMap;
use serde_derive::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct PackageMetadata {
  package: Package,
  dependencies: HashMap<String, String>,
}

#[derive(Deserialize)]
struct Package {
  name: String,
  description: String,
  keywords: Vec<String>,
  homepage: String,
  repository_url: String,
}

pub fn load_from_path(path: &str) -> PackageMetadata {
  let toml_content = fs::read_to_string(path).unwrap();
  let package_metadata: PackageMetadata = toml::from_str(&toml_content).unwrap();
  return package_metadata;
}

#[test]
fn should_load_from_path() {
  let package_metadata = load_from_path("./tests/fixtures/test_artifact/buff.toml");
  assert_eq!(package_metadata.package.name, "test_package");
  assert_eq!(package_metadata.package.description, "test_package description");
  assert_eq!(package_metadata.package.homepage, "https://example.com");
  assert_eq!(package_metadata.package.repository_url, "https://repo.com");
  assert_eq!(package_metadata.package.keywords, ["awesome", "great"]);
  assert_eq!(package_metadata.dependencies.len(), 1);
  assert_eq!(package_metadata.dependencies.keys().last().unwrap(), "yummy_package");
  assert_eq!(package_metadata.dependencies.values().last().unwrap(), "1.0");
}