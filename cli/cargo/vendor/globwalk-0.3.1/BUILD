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


# Unsupported target "docs" with type "test" omitted

rust_library(
    name = "globwalk",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cli/cargo/vendor/ignore-0.4.10:ignore",
        "//cli/cargo/vendor/walkdir-2.2.9:walkdir",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.3.1",
    crate_features = [
    ],
)

