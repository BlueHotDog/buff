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


rust_binary(
    # Prefix bin name to disambiguate from (probable) collision with lib name
    # N.B.: The exact form of this is subject to change.
    name = "cargo_bin_bin_fixture",
    crate_root = "src/bin/bin_fixture.rs",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        # Binaries get an implicit dependency on their crate's lib
        ":escargot",
        "//cli/cargo/vendor/lazy_static-1.3.0:lazy_static",
        "//cli/cargo/vendor/log-0.4.8:log",
        "//cli/cargo/vendor/serde-1.0.99:serde",
        "//cli/cargo/vendor/serde_json-1.0.40:serde_json",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.4.0",
    crate_features = [
    ],
)

# Unsupported target "build" with type "test" omitted
# Unsupported target "build-script-build" with type "custom-build" omitted

rust_library(
    name = "escargot",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cli/cargo/vendor/lazy_static-1.3.0:lazy_static",
        "//cli/cargo/vendor/log-0.4.8:log",
        "//cli/cargo/vendor/serde-1.0.99:serde",
        "//cli/cargo/vendor/serde_json-1.0.40:serde_json",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.4.0",
    crate_features = [
    ],
)

# Unsupported target "example_fixture" with type "example" omitted
# Unsupported target "run" with type "test" omitted
