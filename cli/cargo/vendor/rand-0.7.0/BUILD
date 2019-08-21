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


# Unsupported target "generators" with type "bench" omitted
# Unsupported target "misc" with type "bench" omitted
# Unsupported target "monte-carlo" with type "example" omitted
# Unsupported target "monty-hall" with type "example" omitted

rust_library(
    name = "rand",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2018",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cli/cargo/vendor/getrandom-0.1.10:getrandom",
        "//cli/cargo/vendor/libc-0.2.62:libc",
        "//cli/cargo/vendor/rand_chacha-0.2.1:rand_chacha",
        "//cli/cargo/vendor/rand_core-0.5.0:rand_core",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.7.0",
    crate_features = [
        "alloc",
        "default",
        "getrandom",
        "getrandom_package",
        "rand_core",
        "std",
    ],
)

# Unsupported target "seq" with type "bench" omitted
