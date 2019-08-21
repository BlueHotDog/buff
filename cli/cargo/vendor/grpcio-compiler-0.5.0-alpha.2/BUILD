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


rust_binary(
    # Prefix bin name to disambiguate from (probable) collision with lib name
    # N.B.: The exact form of this is subject to change.
    name = "cargo_bin_grpc_rust_plugin",
    crate_root = "src/bin/grpc_rust_plugin.rs",
    edition = "2018",
    srcs = glob(["**/*.rs"]),
    deps = [
        # Binaries get an implicit dependency on their crate's lib
        ":grpcio_compiler",
        "//cli/cargo/vendor/derive-new-0.5.8:derive_new",
        "//cli/cargo/vendor/protobuf-2.8.0:protobuf",
        "//cli/cargo/vendor/protobuf-codegen-2.8.0:protobuf_codegen",
        "//cli/cargo/vendor/tempfile-3.1.0:tempfile",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.5.0-alpha.2",
    crate_features = [
        "default",
        "protobuf-codec",
        "protobuf-codegen",
    ],
)


rust_library(
    name = "grpcio_compiler",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2018",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cli/cargo/vendor/derive-new-0.5.8:derive_new",
        "//cli/cargo/vendor/protobuf-2.8.0:protobuf",
        "//cli/cargo/vendor/protobuf-codegen-2.8.0:protobuf_codegen",
        "//cli/cargo/vendor/tempfile-3.1.0:tempfile",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.5.0-alpha.2",
    crate_features = [
        "default",
        "protobuf-codec",
        "protobuf-codegen",
    ],
)

