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


# Unsupported target "all" with type "test" omitted
# Unsupported target "entry" with type "test" omitted
# Unsupported target "extract_file" with type "example" omitted
# Unsupported target "list" with type "example" omitted
# Unsupported target "raw_list" with type "example" omitted

rust_library(
    name = "tar",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2018",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cli/cargo/vendor/filetime-0.2.7:filetime",
        "//cli/cargo/vendor/libc-0.2.62:libc",
        "//cli/cargo/vendor/xattr-0.2.2:xattr",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.4.26",
    crate_features = [
        "default",
        "xattr",
    ],
)

# Unsupported target "write" with type "example" omitted
