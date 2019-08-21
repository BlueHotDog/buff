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


# Unsupported target "greeter_client" with type "example" omitted
# Unsupported target "greeter_server" with type "example" omitted

rust_library(
    name = "grpcio",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cli/cargo/vendor/futures-0.1.28:futures",
        "//cli/cargo/vendor/grpcio-sys-0.4.4:grpcio_sys",
        "//cli/cargo/vendor/libc-0.2.62:libc",
        "//cli/cargo/vendor/log-0.4.8:log",
        "//cli/cargo/vendor/protobuf-2.8.0:protobuf",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.4.4",
    crate_features = [
        "default",
        "grpcio-sys",
        "protobuf",
        "protobuf-codec",
        "secure",
    ],
)

# Unsupported target "route_guide_client" with type "example" omitted
# Unsupported target "route_guide_server" with type "example" omitted
# Unsupported target "tests" with type "test" omitted
