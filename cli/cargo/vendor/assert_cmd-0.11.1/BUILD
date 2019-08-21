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


# Unsupported target "assert" with type "test" omitted

rust_library(
    name = "assert_cmd",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cli/cargo/vendor/escargot-0.4.0:escargot",
        "//cli/cargo/vendor/predicates-1.0.1:predicates",
        "//cli/cargo/vendor/predicates-core-1.0.0:predicates_core",
        "//cli/cargo/vendor/predicates-tree-1.0.0:predicates_tree",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.11.1",
    crate_features = [
    ],
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
        ":assert_cmd",
        "//cli/cargo/vendor/escargot-0.4.0:escargot",
        "//cli/cargo/vendor/predicates-1.0.1:predicates",
        "//cli/cargo/vendor/predicates-core-1.0.0:predicates_core",
        "//cli/cargo/vendor/predicates-tree-1.0.0:predicates_tree",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.11.1",
    crate_features = [
    ],
)

# Unsupported target "cargo" with type "test" omitted
# Unsupported target "docmatic" with type "test" omitted
# Unsupported target "example_fixture" with type "example" omitted
# Unsupported target "examples" with type "test" omitted
