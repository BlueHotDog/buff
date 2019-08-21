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



rust_library(
    name = "blake2b_simd",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2018",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cli/cargo/vendor/arrayref-0.3.5:arrayref",
        "//cli/cargo/vendor/arrayvec-0.4.11:arrayvec",
        "//cli/cargo/vendor/constant_time_eq-0.1.4:constant_time_eq",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.5.7",
    crate_features = [
        "default",
        "std",
    ],
)

