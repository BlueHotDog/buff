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



rust_library(
    name = "quicli",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2018",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cli/cargo/vendor/clap-verbosity-flag-0.2.0:clap_verbosity_flag",
        "//cli/cargo/vendor/exitfailure-0.5.1:exitfailure",
        "//cli/cargo/vendor/failure-0.1.5:failure",
        "//cli/cargo/vendor/failure_derive-0.1.5:failure_derive",
        "//cli/cargo/vendor/globwalk-0.3.1:globwalk",
        "//cli/cargo/vendor/log-0.4.8:log",
        "//cli/cargo/vendor/rayon-1.1.0:rayon",
        "//cli/cargo/vendor/remove_dir_all-0.5.2:remove_dir_all",
        "//cli/cargo/vendor/serde-1.0.99:serde",
        "//cli/cargo/vendor/serde_derive-1.0.99:serde_derive",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.4.0",
    crate_features = [
        "default",
        "full-throttle",
        "globwalk",
        "rayon",
        "remove_dir_all",
        "serde",
        "serde_derive",
    ],
)

