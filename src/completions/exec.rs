use super::App;
use pipe_trait::*;
use std::{
    fs::File,
    io::{stdout, Write},
};
use structopt::StructOpt;

impl App {
    /// Run the completion generator
    pub fn exec<Target: StructOpt>(self) {
        let App { bin, output, shell } = self;

        let mut buf: Box<dyn Write> = if let Some(file_name) = output {
            file_name.pipe(File::create).unwrap().pipe(Box::new)
        } else {
            Box::new(stdout())
        };

        Target::clap().gen_completions_to(bin, shell.to_clap(), &mut buf);
    }
}
