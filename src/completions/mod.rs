//! Resources to create a completion generator program.

use pipe_trait::*;
use std::{
    fs::File,
    io::{stdout, Write},
    path::PathBuf,
};
use structopt::StructOpt;

mod shell;

pub use shell::Shell;

/// Arguments of a completion generator program.
#[derive(Debug, StructOpt, Clone, Eq, PartialEq)]
#[structopt(rename_all = "kebab")]
pub struct Args {
    /// Binary name
    #[structopt(long, default_value = "pretty-exec")]
    pub bin: String,

    /// File to write to
    /// [default: stdout]
    #[structopt(long, short = "o")]
    pub output: Option<PathBuf>,

    /// Type of shell
    #[structopt(
        name = "shell",
        possible_values = &["bash", "fish", "zsh", "powershell", "elvish"],
        parse(from_str = Shell::parse_from_str_unchecked),
    )]
    pub shell: Shell,
}

impl Args {
    /// Run the completion generator
    pub fn exec<Target: StructOpt>(self) {
        let Args { bin, output, shell } = self;

        let mut buf: Box<dyn Write> = if let Some(file_name) = output {
            file_name.pipe(File::create).unwrap().pipe(Box::new)
        } else {
            Box::new(stdout())
        };

        Target::clap().gen_completions_to(bin, shell.to_clap(), &mut buf);
    }
}
