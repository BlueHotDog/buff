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


# Unsupported target "smoke" with type "test" omitted

rust_library(
    name = "tempdir",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cli/cargo/vendor/rand-0.4.6:rand",
        "//cli/cargo/vendor/remove_dir_all-0.5.2:remove_dir_all",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.3.7",
    crate_features = [
    ],
)

