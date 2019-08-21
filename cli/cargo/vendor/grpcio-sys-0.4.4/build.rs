// Copyright 2017 PingCAP, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate cc;
extern crate cmake;
extern crate pkg_config;

use std::env::VarError;
use std::path::Path;
use std::{env, fs, io};

use cc::Build;
use cmake::Config;
use pkg_config::{Config as PkgConfig, Library};

const GRPC_VERSION: &'static str = "1.17.2";

fn probe_library(library: &str, cargo_metadata: bool) -> Library {
    match PkgConfig::new()
        .atleast_version(GRPC_VERSION)
        .cargo_metadata(cargo_metadata)
        .probe(library)
    {
        Ok(lib) => lib,
        Err(e) => panic!("can't find library {} via pkg-config: {:?}", library, e),
    }
}

fn prepare_grpc() {
    let mut modules = vec![
        "grpc",
        "grpc/third_party/zlib",
        "grpc/third_party/cares/cares",
        "grpc/third_party/address_sorting",
    ];

    if cfg!(feature = "secure") {
        modules.push("grpc/third_party/boringssl");
    }

    for module in modules {
        if is_directory_empty(module).unwrap_or(true) {
            panic!(
                "Can't find module {}. You need to run `git submodule \
                 update --init --recursive` first to build the project.",
                module
            );
        }
    }
}

fn is_directory_empty<P: AsRef<Path>>(p: P) -> Result<bool, io::Error> {
    let mut entries = fs::read_dir(p)?;
    Ok(entries.next().is_none())
}

fn build_grpc(cc: &mut Build, library: &str) {
    prepare_grpc();

    let dst = {
        let mut config = Config::new("grpc");
        if !cfg!(feature = "secure") {
            // boringssl's configuration is still included, but targets
            // will never be built, hence specify a fake go to get rid of
            // the unnecessary dependency.
            config.define("GO_EXECUTABLE", "fake-go-nonexist");
        }
        if get_env("CARGO_CFG_TARGET_OS").map_or(false, |s| s == "macos") {
            config.cxxflag("-stdlib=libc++");
        }
        if env::var("CARGO_CFG_TARGET_ENV").unwrap_or("".to_owned()) == "musl" {
            config.define("CMAKE_CXX_COMPILER", "g++");
        }

        // Cross-compile support for iOS
        match env::var("TARGET").unwrap_or("".to_owned()).as_str() {
            "aarch64-apple-ios" => {
                config
                    .define("CMAKE_OSX_SYSROOT", "iphoneos")
                    .define("CMAKE_OSX_ARCHITECTURES", "arm64");
            }
            "armv7-apple-ios" => {
                config
                    .define("CMAKE_OSX_SYSROOT", "iphoneos")
                    .define("CMAKE_OSX_ARCHITECTURES", "armv7");
            }
            "armv7s-apple-ios" => {
                config
                    .define("CMAKE_OSX_SYSROOT", "iphoneos")
                    .define("CMAKE_OSX_ARCHITECTURES", "armv7s");
            }
            "i386-apple-ios" => {
                config
                    .define("CMAKE_OSX_SYSROOT", "iphonesimulator")
                    .define("CMAKE_OSX_ARCHITECTURES", "i386");
            }
            "x86_64-apple-ios" => {
                config
                    .define("CMAKE_OSX_SYSROOT", "iphonesimulator")
                    .define("CMAKE_OSX_ARCHITECTURES", "x86_64");
            }
            _ => {}
        };

        // Allow overriding of the target passed to cmake
        // (needed for Android crosscompile)
        match env::var("CMAKE_TARGET_OVERRIDE") {
            Ok(val) => {
                config.target(&val);
            }
            Err(_) => {}
        };

        // We don't need to generate install targets.
        config.define("gRPC_INSTALL", "false");
        // We don't need to build csharp target.
        config.define("gRPC_BUILD_CSHARP_EXT", "false");
        // We don't need to build codegen target.
        config.define("gRPC_BUILD_CODEGEN", "false");
        // We don't need to build benchmarks.
        config.define("gRPC_BENCHMARK_PROVIDER", "none");
        if cfg!(feature = "openssl") {
            config.define("gRPC_SSL_PROVIDER", "package");
            config.define("EMBED_OPENSSL", "false");
            // Problem is: Ubuntu Trusty shipped with openssl 1.0.1f. Which doesn't
            // support alpn. And Google's gRPC checks for support of ALPN in plane
            // old Makefile, but not in CMake.
            config.cxxflag("-DTSI_OPENSSL_ALPN_SUPPORT=0");
        }
        config.build_target(library).uses_cxx11().build()
    };

    let mut zlib = "z";
    let build_dir = format!("{}/build", dst.display());
    let third_party = vec![
        "cares/cares/lib",
        "zlib",
        "boringssl/ssl",
        "boringssl/crypto",
    ];
    if get_env("CARGO_CFG_TARGET_OS").map_or(false, |s| s == "windows") {
        let profile = match &*env::var("PROFILE").unwrap_or("debug".to_owned()) {
            "bench" | "release" => {
                zlib = "zlibstatic";
                "Release"
            }
            _ => {
                zlib = "zlibstaticd";
                "Debug"
            }
        };
        println!("cargo:rustc-link-search=native={}/{}", build_dir, profile);
        for path in third_party {
            println!(
                "cargo:rustc-link-search=native={}/third_party/{}/{}",
                build_dir, path, profile
            );
        }
    } else {
        println!("cargo:rustc-link-search=native={}", build_dir);
        for path in third_party {
            println!(
                "cargo:rustc-link-search=native={}/third_party/{}",
                build_dir, path,
            );
        }
    }

    println!("cargo:rustc-link-lib=static={}", zlib);
    println!("cargo:rustc-link-lib=static=cares");
    println!("cargo:rustc-link-lib=static=gpr");
    println!("cargo:rustc-link-lib=static=address_sorting");
    println!("cargo:rustc-link-lib=static={}", library);

    if cfg!(feature = "secure") {
        if cfg!(feature = "openssl") {
            println!("cargo:rustc-link-lib=ssl");
            println!("cargo:rustc-link-lib=crypto");
        } else {
            println!("cargo:rustc-link-lib=static=ssl");
            println!("cargo:rustc-link-lib=static=crypto");
        }
    }

    cc.include("grpc/include");
}

fn get_env(name: &str) -> Option<String> {
    println!("cargo:rerun-if-env-changed={}", name);
    match env::var(name) {
        Ok(s) => Some(s),
        Err(VarError::NotPresent) => None,
        Err(VarError::NotUnicode(s)) => {
            panic!("unrecognize env var of {}: {:?}", name, s.to_string_lossy());
        }
    }
}

fn main() {
    let mut cc = Build::new();

    println!("cargo:rerun-if-changed=grpc_wrap.cc");
    println!("cargo:rerun-if-changed=grpc");

    let library = if cfg!(feature = "secure") {
        cc.define("GRPC_SYS_SECURE", None);
        "grpc"
    } else {
        "grpc_unsecure"
    };

    if get_env("GRPCIO_SYS_USE_PKG_CONFIG").map_or(false, |s| s == "1") {
        // Print cargo metadata.
        let lib_core = probe_library(library, true);
        for inc_path in lib_core.include_paths {
            cc.include(inc_path);
        }
    } else {
        build_grpc(&mut cc, library);
    }

    cc.cpp(true);
    if !cfg!(target_env = "msvc") {
        cc.flag("-std=c++11");
    }
    cc.file("grpc_wrap.cc");

    if get_env("CARGO_CFG_TARGET_OS").map_or(false, |s| s == "windows") {
        // At lease win7
        cc.define("_WIN32_WINNT", Some("0x0700"));
    }

    cc.warnings_into_errors(true);
    cc.compile("libgrpc_wrap.a");
}
