"""
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = [
  # Public for visibility by "@raze__crate__version//" targets.
  #
  # Prefer access through "//cli/cargo", which limits external
  # visibility to explicit Cargo.toml dependencies.
  "//visibility:public",
])

licenses([
  "notice", # "MIT"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
)


# Unsupported target "bash" with type "example" omitted
# Unsupported target "bash_read" with type "example" omitted
# Unsupported target "ftp" with type "example" omitted

rust_library(
    name = "rexpect",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cli/cargo/vendor/error-chain-0.11.0:error_chain",
        "//cli/cargo/vendor/nix-0.9.0:nix",
        "//cli/cargo/vendor/regex-0.2.11:regex",
        "//cli/cargo/vendor/tempfile-2.2.0:tempfile",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.3.0",
    crate_features = [
    ],
)

