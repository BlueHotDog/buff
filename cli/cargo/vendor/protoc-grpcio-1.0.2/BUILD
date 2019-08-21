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
  "notice", # "Apache-2.0"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
)



rust_library(
    name = "protoc_grpcio",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cli/cargo/vendor/failure-0.1.5:failure",
        "//cli/cargo/vendor/grpcio-compiler-0.5.0-alpha.2:grpcio_compiler",
        "//cli/cargo/vendor/protobuf-2.8.0:protobuf",
        "//cli/cargo/vendor/protobuf-codegen-2.8.0:protobuf_codegen",
        "//cli/cargo/vendor/protoc-2.8.0:protoc",
        "//cli/cargo/vendor/tempfile-3.1.0:tempfile",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "1.0.2",
    crate_features = [
    ],
)

