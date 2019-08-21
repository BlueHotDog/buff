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
  "notice", # "Apache-2.0,MIT"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
)


# Unsupported target "clones" with type "test" omitted
# Unsupported target "cpu_monitor" with type "example" omitted
# Unsupported target "debug" with type "test" omitted
# Unsupported target "intersperse" with type "test" omitted
# Unsupported target "iter_panic" with type "test" omitted
# Unsupported target "named-threads" with type "test" omitted
# Unsupported target "octillion" with type "test" omitted
# Unsupported target "producer_split_at" with type "test" omitted

rust_library(
    name = "rayon",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cli/cargo/vendor/crossbeam-deque-0.6.3:crossbeam_deque",
        "//cli/cargo/vendor/either-1.5.2:either",
        "//cli/cargo/vendor/rayon-core-1.5.0:rayon_core",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "1.1.0",
    crate_features = [
    ],
)

# Unsupported target "sort-panic-safe" with type "test" omitted
# Unsupported target "str" with type "test" omitted
