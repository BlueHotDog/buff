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
  "restricted", # "MIT OR Apache-2.0"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
)



rust_library(
    name = "clap_verbosity_flag",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cli/cargo/vendor/env_logger-0.5.13:env_logger",
        "//cli/cargo/vendor/failure-0.1.5:failure",
        "//cli/cargo/vendor/log-0.4.8:log",
        "//cli/cargo/vendor/structopt-0.2.18:structopt",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.2.0",
    crate_features = [
    ],
)

