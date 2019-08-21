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


# Unsupported target "custom_default_format" with type "example" omitted
# Unsupported target "custom_format" with type "example" omitted
# Unsupported target "custom_logger" with type "example" omitted
# Unsupported target "default" with type "example" omitted
# Unsupported target "direct_logger" with type "example" omitted

rust_library(
    name = "env_logger",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cli/cargo/vendor/atty-0.2.13:atty",
        "//cli/cargo/vendor/humantime-1.2.0:humantime",
        "//cli/cargo/vendor/log-0.4.8:log",
        "//cli/cargo/vendor/regex-1.2.1:regex",
        "//cli/cargo/vendor/termcolor-1.0.5:termcolor",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.5.13",
    crate_features = [
        "default",
        "regex",
    ],
)

# Unsupported target "log-in-log" with type "test" omitted
# Unsupported target "regexp_filter" with type "test" omitted
