use pipe_trait::*;
use structopt::clap::{self, Shell::*};

/// Shell wrapper type with additional traits.
#[derive(Debug, Copy, Clone)]
pub struct Shell(pub clap::Shell);

impl Shell {
    /// Convert a string to `Shell`
    pub fn parse_from_str(text: &str) -> Result<Self, &str> {
        match text {
            "bash" => Bash,
            "fish" => Fish,
            "zsh" => Zsh,
            "powershell" => PowerShell,
            "elvish" => Elvish,
            _ => return Err(text),
        }
        .pipe(Shell)
        .pipe(Ok)
    }

    /// Extract `clap::Shell`.
    pub fn to_clap(self) -> clap::Shell {
        self.0
    }

    /// Wrap `clap::Shell` into `Shell`.
    pub fn from_clap(clap: clap::Shell) -> Self {
        Shell(clap)
    }
}

impl PartialEq for Shell {
    fn eq(&self, other: &Self) -> bool {
        match (self.0, other.0) {
            (Bash, Bash)
            | (Fish, Fish)
            | (Zsh, Zsh)
            | (PowerShell, PowerShell)
            | (Elvish, Elvish) => true,
            _ => false,
        }
    }
}

impl Eq for Shell {}
