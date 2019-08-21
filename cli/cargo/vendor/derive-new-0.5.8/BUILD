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
    name = "derive_new",
    crate_root = "src/lib.rs",
    crate_type = "proc-macro",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cli/cargo/vendor/proc-macro2-1.0.1:proc_macro2",
        "//cli/cargo/vendor/quote-1.0.2:quote",
        "//cli/cargo/vendor/syn-1.0.3:syn",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.5.8",
    crate_features = [
        "default",
        "std",
    ],
)

# Unsupported target "test" with type "test" omitted
