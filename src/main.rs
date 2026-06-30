mod actions;
mod cli;
mod configuration;
mod database;
mod state;
mod utils;

use crate::cli::command::{Cli, Command};
use crate::configuration::config::Config;
use clap::Parser;

fn main() {
    let args = Cli::parse();
    Config::init(args.config);

    match args.command {
        None => {} // We will use this to launch the tui eventually
        Some(Command::GenerateConfig) => actions::generate_config::run(),
        Some(Command::Create(args)) => actions::create::run(args),
        Some(Command::Add(args)) => actions::add::run(args),
        Some(Command::Ask) => actions::ask::run(),
        Some(Command::Answer) => actions::answer::run(),
        Some(Command::Eval(args)) => actions::eval::run(args),
    }
}
