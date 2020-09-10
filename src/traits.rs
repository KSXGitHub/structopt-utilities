use super::completions;
use std::{env::args, ffi::OsString, process::exit};
use structopt::{clap, StructOpt};

/// Utility trait for all `StructOpt` types.
pub trait StructOptUtils: StructOpt + Sized {
    /// If the flags are invalid or unknown, print error and exit with code 1.
    /// If `--help`, print help message and exit with code 0.
    /// If `--version`, print version and exit with code 0.
    /// Otherwise, return parsing result.
    fn strict_from_iter(input: impl IntoIterator<Item = impl Into<OsString> + Clone>) -> Self {
        match Self::from_iter_safe(input) as Result<Self, clap::Error> {
            Ok(value) => value,
            Err(clap::Error { kind, message, .. }) => match kind {
                clap::ErrorKind::HelpDisplayed | clap::ErrorKind::VersionDisplayed => {
                    println!("{}", message);
                    exit(0);
                }
                _ => {
                    eprintln!("{}", message);
                    exit(1);
                }
            },
        }
    }

    /// Apply `parse_strict` on main CLI arguments.
    fn strict_from_args() -> Self {
        Self::strict_from_iter(args())
    }

    /// Run completion generator program.
    fn run_completion_generator(name: &str, bin: &str) {
        completions::App::new(name, bin).exec::<Self>()
    }
}

impl<Args: StructOpt> StructOptUtils for Args {}
