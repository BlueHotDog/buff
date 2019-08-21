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


# Unsupported target "accuracy" with type "test" omitted
# Unsupported target "backtrace" with type "example" omitted

rust_library(
    name = "backtrace",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2018",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cli/cargo/vendor/backtrace-sys-0.1.31:backtrace_sys",
        "//cli/cargo/vendor/cfg-if-0.1.9:cfg_if",
        "//cli/cargo/vendor/libc-0.2.62:libc",
        "//cli/cargo/vendor/rustc-demangle-0.1.16:rustc_demangle",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.3.35",
    crate_features = [
        "backtrace-sys",
        "dbghelp",
        "default",
        "dladdr",
        "libbacktrace",
        "libunwind",
        "std",
    ],
)

# Unsupported target "benchmarks" with type "bench" omitted
# Unsupported target "long_fn_name" with type "test" omitted
# Unsupported target "raw" with type "example" omitted
# Unsupported target "skip_inner_frames" with type "test" omitted
# Unsupported target "smoke" with type "test" omitted
