use std::env;
use std::ffi;
use std::process;
use std::str;

use build::CargoBuild;

/// The current process' target triplet.
pub const CURRENT_TARGET: &str = include_str!(concat!(env!("OUT_DIR"), "/current_target.txt"));

lazy_static! {
    static ref CARBO_BIN: ffi::OsString = env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
}

/// Top-level command.
#[derive(Debug)]
pub struct Cargo {
    cmd: process::Command,
}

impl Cargo {
    /// Create a top-level command.
    pub fn new() -> Self {
        Self {
            cmd: process::Command::new(CARBO_BIN.as_os_str()),
        }
    }

    /// Manually pass an argument that is unsupported.
    ///
    /// Caution: Passing in a sub-command or `--` can throw off the API.
    pub fn arg<S: AsRef<ffi::OsStr>>(mut self, arg: S) -> Self {
        self.cmd.arg(arg);
        self
    }

    /// Run the `build` subcommand.
    pub fn build(self) -> CargoBuild {
        self.build_with("build")
    }

    /// Run a custom `build` subcommand.
    pub fn build_with<S: AsRef<ffi::OsStr>>(mut self, name: S) -> CargoBuild {
        self.cmd.arg(name).arg("--message-format=json");
        CargoBuild::with_command(self.cmd)
    }
}

impl Default for Cargo {
    fn default() -> Self {
        Self::new()
    }
}
