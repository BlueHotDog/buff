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
  "notice", # "MIT,Apache-2.0"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
)


alias(
  name = "rust_argon2",
  actual = ":argon2",
)

rust_library(
    name = "argon2",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cli/cargo/vendor/base64-0.10.1:base64",
        "//cli/cargo/vendor/blake2b_simd-0.5.7:blake2b_simd",
        "//cli/cargo/vendor/crossbeam-utils-0.6.6:crossbeam_utils",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.5.1",
    crate_features = [
    ],
)

# Unsupported target "integration_test" with type "test" omitted
