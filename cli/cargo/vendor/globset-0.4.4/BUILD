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
  "unencumbered", # "Unlicense"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
)


# Unsupported target "bench" with type "bench" omitted

rust_library(
    name = "globset",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cli/cargo/vendor/aho-corasick-0.7.6:aho_corasick",
        "//cli/cargo/vendor/bstr-0.2.7:bstr",
        "//cli/cargo/vendor/fnv-1.0.6:fnv",
        "//cli/cargo/vendor/log-0.4.8:log",
        "//cli/cargo/vendor/regex-1.2.1:regex",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.4.4",
    crate_features = [
    ],
)

