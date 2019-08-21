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
  "restricted", # "MIT OR Apache-2.0"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
)


# Unsupported target "context" with type "example" omitted
# Unsupported target "display" with type "example" omitted
# Unsupported target "example" with type "example" omitted

rust_library(
    name = "exitfailure",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cli/cargo/vendor/failure-0.1.5:failure",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.5.1",
    crate_features = [
    ],
)

# Unsupported target "test" with type "test" omitted
