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
    name = "clap",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cli/cargo/vendor/ansi_term-0.11.0:ansi_term",
        "//cli/cargo/vendor/atty-0.2.13:atty",
        "//cli/cargo/vendor/bitflags-1.1.0:bitflags",
        "//cli/cargo/vendor/strsim-0.8.0:strsim",
        "//cli/cargo/vendor/textwrap-0.11.0:textwrap",
        "//cli/cargo/vendor/unicode-width-0.1.6:unicode_width",
        "//cli/cargo/vendor/vec_map-0.8.1:vec_map",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "2.33.0",
    crate_features = [
        "ansi_term",
        "atty",
        "color",
        "default",
        "strsim",
        "suggestions",
        "vec_map",
    ],
)

