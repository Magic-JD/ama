mod actions;
mod cli;
mod configuration;
mod utils;

use crate::cli::command::Cli;
use crate::configuration::config::Config;
use clap::Parser;

fn main() {
    let args = Cli::parse();
    Config::init(args.config);

    if args.task.generate_config {
        actions::generate_config::run();
        return;
    }
}
