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

rust_binary(
    name = "syn_build_script",
    srcs = glob(["**/*.rs"]),
    crate_root = "build.rs",
    edition = "2018",
    deps = [
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    crate_features = [
      "clone-impls",
      "default",
      "derive",
      "parsing",
      "printing",
      "proc-macro",
      "proc-macro2",
      "quote",
      "visit",
    ],
    data = glob(["*"]),
    version = "1.0.3",
    visibility = ["//visibility:private"],
)

genrule(
    name = "syn_build_script_executor",
    srcs = glob(["*", "**/*.rs"]),
    outs = ["syn_out_dir_outputs.tar.gz"],
    tools = [
      ":syn_build_script",
    ],
    tags = ["no-sandbox"],
    cmd = "mkdir -p $$(dirname $@)/syn_out_dir_outputs/;"
        + " (export CARGO_MANIFEST_DIR=\"$$PWD/$$(dirname $(location :Cargo.toml))\";"
        # TODO(acmcarther): This needs to be revisited as part of the cross compilation story.
        #                   See also: https://github.com/google/cargo-raze/pull/54
        + " export TARGET='x86_64-unknown-linux-gnu';"
        + " export RUST_BACKTRACE=1;"
        + " export CARGO_FEATURE_CLONE_IMPLS=1;"
        + " export CARGO_FEATURE_DEFAULT=1;"
        + " export CARGO_FEATURE_DERIVE=1;"
        + " export CARGO_FEATURE_PARSING=1;"
        + " export CARGO_FEATURE_PRINTING=1;"
        + " export CARGO_FEATURE_PROC_MACRO=1;"
        + " export CARGO_FEATURE_PROC_MACRO2=1;"
        + " export CARGO_FEATURE_QUOTE=1;"
        + " export CARGO_FEATURE_VISIT=1;"
        + " export OUT_DIR=$$PWD/$$(dirname $@)/syn_out_dir_outputs;"
        + " export BINARY_PATH=\"$$PWD/$(location :syn_build_script)\";"
        + " export OUT_TAR=$$PWD/$@;"
        + " cd $$(dirname $(location :Cargo.toml)) && $$BINARY_PATH && tar -czf $$OUT_TAR -C $$OUT_DIR .)"
)

# Unsupported target "file" with type "bench" omitted
# Unsupported target "rust" with type "bench" omitted

rust_library(
    name = "syn",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2018",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cli/cargo/vendor/proc-macro2-1.0.1:proc_macro2",
        "//cli/cargo/vendor/quote-1.0.2:quote",
        "//cli/cargo/vendor/unicode-xid-0.2.0:unicode_xid",
    ],
    rustc_flags = [
        "--cap-lints=allow",
        "--cfg=ossl102",
        "-Clink-args=-lstdc++",
    ],
    out_dir_tar = ":syn_build_script_executor",
    version = "1.0.3",
    crate_features = [
        "clone-impls",
        "default",
        "derive",
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
# Unsupported target "test_size" with type "test" omitted
# Unsupported target "test_token_trees" with type "test" omitted
# Unsupported target "zzz_stable" with type "test" omitted
