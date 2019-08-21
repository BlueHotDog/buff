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
    name = "synstructure",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cli/cargo/vendor/proc-macro2-0.4.30:proc_macro2",
        "//cli/cargo/vendor/quote-0.6.13:quote",
        "//cli/cargo/vendor/syn-0.15.44:syn",
        "//cli/cargo/vendor/unicode-xid-0.1.0:unicode_xid",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.10.2",
    crate_features = [
    ],
)

