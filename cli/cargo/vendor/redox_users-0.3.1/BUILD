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
    name = "redox_users",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cli/cargo/vendor/failure-0.1.5:failure",
        "//cli/cargo/vendor/rand_os-0.1.3:rand_os",
        "//cli/cargo/vendor/redox_syscall-0.1.56:redox_syscall",
        "//cli/cargo/vendor/rust-argon2-0.5.1:rust_argon2",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.3.1",
    crate_features = [
    ],
)

