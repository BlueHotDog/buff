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


# Unsupported target "async-reader" with type "test" omitted
# Unsupported target "deflatedecoder-bufread" with type "example" omitted
# Unsupported target "deflatedecoder-read" with type "example" omitted
# Unsupported target "deflatedecoder-write" with type "example" omitted
# Unsupported target "deflateencoder-bufread" with type "example" omitted
# Unsupported target "deflateencoder-read" with type "example" omitted
# Unsupported target "deflateencoder-write" with type "example" omitted
# Unsupported target "early-flush" with type "test" omitted
# Unsupported target "empty-read" with type "test" omitted

rust_library(
    name = "flate2",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cli/cargo/vendor/crc32fast-1.2.0:crc32fast",
        "//cli/cargo/vendor/libc-0.2.62:libc",
        "//cli/cargo/vendor/libz-sys-1.0.25:libz_sys",
        "//cli/cargo/vendor/miniz_oxide-0.3.2:miniz_oxide",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "1.0.11",
    crate_features = [
        "libz-sys",
        "zlib",
    ],
)

# Unsupported target "gunzip" with type "test" omitted
# Unsupported target "gzbuilder" with type "example" omitted
# Unsupported target "gzdecoder-bufread" with type "example" omitted
# Unsupported target "gzdecoder-read" with type "example" omitted
# Unsupported target "gzdecoder-write" with type "example" omitted
# Unsupported target "gzencoder-bufread" with type "example" omitted
# Unsupported target "gzencoder-read" with type "example" omitted
# Unsupported target "gzencoder-write" with type "example" omitted
# Unsupported target "gzmultidecoder-bufread" with type "example" omitted
# Unsupported target "gzmultidecoder-read" with type "example" omitted
# Unsupported target "tokio" with type "test" omitted
# Unsupported target "zero-write" with type "test" omitted
# Unsupported target "zlibdecoder-bufread" with type "example" omitted
# Unsupported target "zlibdecoder-read" with type "example" omitted
# Unsupported target "zlibdecoder-write" with type "example" omitted
# Unsupported target "zlibencoder-bufread" with type "example" omitted
# Unsupported target "zlibencoder-read" with type "example" omitted
# Unsupported target "zlibencoder-write" with type "example" omitted
