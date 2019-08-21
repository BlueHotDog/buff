package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library"
)


rust_library(
    name = "buff_lib",
    srcs = glob(["src/**/*.rs"]),
     deps = [
      "//cli/cargo:flate2",
      "//cli/cargo:toml"
    ]
)
