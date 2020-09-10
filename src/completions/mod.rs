//! Resources to create a completion generator program.

mod exec;
mod new;
mod shell;

pub use shell::Shell;

use std::path::PathBuf;

/// Arguments of a completion generator program.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct App {
    /// Binary name.
    pub bin: String,

    /// File to write to.
    /// `None` translates to stdout.
    pub output: Option<PathBuf>,

    /// Type of shell.
    pub shell: Shell,
}
