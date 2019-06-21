use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Deserialize)]
pub struct PackageMetadata {
  package: Package,
}

#[derive(Deserialize)]
struct Package {
  name: String,
  description: String,
  keywords: Vec<String>,
  homepage: String,
  repository_url: String,
}

impl PackageMetadata {
  pub fn new(path: &str) -> Self {
    let toml_content = fs::read_to_string(path).unwrap();
    let package_metadata: PackageMetadata = toml::from_str(&toml_content).unwrap();
    return package_metadata;
  }
}

#[test]
fn should_new() {
  let path = std::env::current_dir()
    .unwrap()
    .join("../tests/fixtures/test_artifact/buff.toml")
    .to_str()
    .unwrap()
    .to_owned();
  let package_metadata = PackageMetadata::new(&path);
  assert_eq!(package_metadata.package.name, "test_package");
  assert_eq!(
    package_metadata.package.description,
    "test_package description"
  );
  assert_eq!(package_metadata.package.homepage, "https://example.com");
  assert_eq!(package_metadata.package.repository_url, "https://repo.com");
  assert_eq!(package_metadata.package.keywords, ["awesome", "great"]);
}
