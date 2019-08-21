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


# Unsupported target "build-script-build" with type "custom-build" omitted
# Unsupported target "double_init_fail" with type "test" omitted
# Unsupported target "init_zero_threads" with type "test" omitted

rust_library(
    name = "rayon_core",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cli/cargo/vendor/crossbeam-deque-0.6.3:crossbeam_deque",
        "//cli/cargo/vendor/crossbeam-queue-0.1.2:crossbeam_queue",
        "//cli/cargo/vendor/crossbeam-utils-0.6.6:crossbeam_utils",
        "//cli/cargo/vendor/lazy_static-1.3.0:lazy_static",
        "//cli/cargo/vendor/num_cpus-1.10.1:num_cpus",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "1.5.0",
    crate_features = [
    ],
)

# Unsupported target "scope_join" with type "test" omitted
# Unsupported target "scoped_threadpool" with type "test" omitted
# Unsupported target "simple_panic" with type "test" omitted
# Unsupported target "stack_overflow_crash" with type "test" omitted
