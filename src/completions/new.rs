use super::{App, Shell};
use pipe_trait::*;
use std::path::PathBuf;
use structopt::clap::{self, Arg};

impl App {
    pub fn new(name: &str, bin: &str) -> Self {
        let matches = clap::App::new("Generate shell completion script")
            .name(name)
            .arg(
                Arg::with_name("binary")
                    .long("bin")
                    .value_name("binary")
                    .help("Binary name")
                    .takes_value(true)
                    .default_value(bin),
            )
            .arg(
                Arg::with_name("output")
                    .long("output")
                    .short("o")
                    .value_name("file")
                    .help("Write to file if specify, write to stdout if not")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("shell")
                    .value_name("shell")
                    .help("Type of shell")
                    .takes_value(true)
                    .required(true)
                    .possible_value("bash")
                    .possible_value("fish")
                    .possible_value("zsh")
                    .possible_value("powershell")
                    .possible_value("elvish"),
            )
            .get_matches();

        App {
            bin: matches.value_of("bin").unwrap_or(bin).to_string(),
            output: matches.value_of("output").map(PathBuf::from),
            shell: matches
                .value_of("shell")
                .unwrap()
                .pipe(|value| unsafe { Shell::parse_from_str_unchecked(value) }),
        }
    }
}
