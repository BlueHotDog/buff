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


# Unsupported target "gitignore_matched_path_or_any_parents_tests" with type "test" omitted

rust_library(
    name = "ignore",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cli/cargo/vendor/crossbeam-channel-0.3.9:crossbeam_channel",
        "//cli/cargo/vendor/globset-0.4.4:globset",
        "//cli/cargo/vendor/lazy_static-1.3.0:lazy_static",
        "//cli/cargo/vendor/log-0.4.8:log",
        "//cli/cargo/vendor/memchr-2.2.1:memchr",
        "//cli/cargo/vendor/regex-1.2.1:regex",
        "//cli/cargo/vendor/same-file-1.0.5:same_file",
        "//cli/cargo/vendor/thread_local-0.3.6:thread_local",
        "//cli/cargo/vendor/walkdir-2.2.9:walkdir",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.4.10",
    crate_features = [
    ],
)

# Unsupported target "walk" with type "example" omitted
