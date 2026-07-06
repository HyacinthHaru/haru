use clap::Parser;

use crate::state::Action;

/// A tiny terminal cat companion.
#[derive(Parser, Debug)]
#[command(name = "haru", version, about = "A tiny terminal cat companion.")]
pub struct Cli {
    /// Pat Haru (counts toward today's pats)
    #[arg(long)]
    pub pat: bool,

    /// Feed Haru (counts toward today's feeds)
    #[arg(long)]
    pub feed: bool,

    /// Show Haru's status
    #[arg(long)]
    pub status: bool,

    /// Force a language (e.g. `en`, `zh`); overrides auto-detection
    #[arg(long, value_name = "CODE")]
    pub lang: Option<String>,
}

impl Cli {
    /// Resolve the action. If several flags are given, priority is
    /// status > feed > pat, then a plain summon.
    pub fn action(&self) -> Action {
        if self.status {
            Action::Status
        } else if self.feed {
            Action::Feed
        } else if self.pat {
            Action::Pat
        } else {
            Action::Summon
        }
    }
}
