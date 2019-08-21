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


# Unsupported target "build-script-build" with type "custom-build" omitted

rust_library(
    name = "syn",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cli/cargo/vendor/proc-macro2-0.4.30:proc_macro2",
        "//cli/cargo/vendor/quote-0.6.13:quote",
        "//cli/cargo/vendor/unicode-xid-0.1.0:unicode_xid",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.15.44",
    crate_features = [
        "clone-impls",
        "default",
        "derive",
        "extra-traits",
        "parsing",
        "printing",
        "proc-macro",
        "proc-macro2",
        "quote",
        "visit",
    ],
)

# Unsupported target "test_asyncness" with type "test" omitted
# Unsupported target "test_attribute" with type "test" omitted
# Unsupported target "test_derive_input" with type "test" omitted
# Unsupported target "test_expr" with type "test" omitted
# Unsupported target "test_generics" with type "test" omitted
# Unsupported target "test_grouping" with type "test" omitted
# Unsupported target "test_ident" with type "test" omitted
# Unsupported target "test_lit" with type "test" omitted
# Unsupported target "test_meta" with type "test" omitted
# Unsupported target "test_parse_buffer" with type "test" omitted
# Unsupported target "test_pat" with type "test" omitted
# Unsupported target "test_precedence" with type "test" omitted
# Unsupported target "test_round_trip" with type "test" omitted
# Unsupported target "test_should_parse" with type "test" omitted
# Unsupported target "test_token_trees" with type "test" omitted
# Unsupported target "zzz_stable" with type "test" omitted
