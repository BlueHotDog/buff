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



rust_library(
    name = "crossbeam_epoch",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cli/cargo/vendor/arrayvec-0.4.11:arrayvec",
        "//cli/cargo/vendor/cfg-if-0.1.9:cfg_if",
        "//cli/cargo/vendor/crossbeam-utils-0.6.6:crossbeam_utils",
        "//cli/cargo/vendor/lazy_static-1.3.0:lazy_static",
        "//cli/cargo/vendor/memoffset-0.5.1:memoffset",
        "//cli/cargo/vendor/scopeguard-1.0.0:scopeguard",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.7.2",
    crate_features = [
        "crossbeam-utils",
        "default",
        "lazy_static",
        "std",
    ],
)

# Unsupported target "defer" with type "bench" omitted
# Unsupported target "flush" with type "bench" omitted
# Unsupported target "pin" with type "bench" omitted
# Unsupported target "sanitize" with type "example" omitted
# Unsupported target "treiber_stack" with type "example" omitted
