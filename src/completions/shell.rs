use core::hint::unreachable_unchecked;
use structopt::clap::{self, Shell::*};

/// Shell wrapper type with additional traits.
#[derive(Debug, Copy, Clone)]
pub struct Shell(pub clap::Shell);

impl Shell {
    /// Convert a string to `Shell`
    ///
    /// ### Safety
    ///
    /// It is safe when `text` is always one of `["bash", "fish", "zsh", "powershell", "elvish"]`.
    /// Otherwise, it returns `unreachable_unchecked()`.
    pub unsafe fn parse_from_str_unchecked(text: &str) -> Self {
        Shell(match text {
            "bash" => Bash,
            "fish" => Fish,
            "zsh" => Zsh,
            "powershell" => PowerShell,
            "elvish" => Elvish,
            _ => unreachable_unchecked(),
        })
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
