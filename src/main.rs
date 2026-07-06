mod cats;
mod cli;
mod lang;
mod render;
mod sounds;
mod state;

use clap::Parser;

use cli::Cli;
use state::{Action, State};

fn main() {
    let cli = Cli::parse();
    let action = cli.action();
    let language = lang::resolve(cli.lang.as_deref());
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();

    // Load -> roll over the day -> apply action -> persist (best effort).
    let path = state::state_file_path();
    let current = match &path {
        Some(p) => State::load(p),
        None => State::default(),
    };
    let next = current.advance(&today, action);

    if let Some(p) = &path {
        if let Err(e) = next.save(p) {
            eprintln!(
                "haru: could not save state ({e}). your cat is fine, but this visit won't be remembered."
            );
        }
    }

    let out = match action {
        Action::Summon => render::summon(&language),
        Action::Pat => render::pat(&language, &next),
        Action::Feed => render::feed(&language, &next),
        Action::Status => render::status(&language, &next),
    };
    println!("{out}");
}
