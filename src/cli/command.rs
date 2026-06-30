use clap::{ArgAction, Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct ConfigArgs {
    // No args for now
}

#[derive(Debug, Parser)]
pub struct CreateArgs {
    pub quiz_name: String,
}

#[derive(Debug, Parser)]
pub struct AddArgs {
    pub question: String,
    pub answer: String,
}

#[derive(Debug, Parser)]
pub struct EvalArgs {
    #[arg(action = ArgAction::Set)]
    pub pass: bool,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    GenerateConfig,
    Create(CreateArgs),
    Add(AddArgs),
    Ask,
    Answer,
    Eval(EvalArgs),
}

#[derive(Debug, Parser)]
#[command(name = "ama")]
#[command(version = env!("CARGO_PKG_VERSION"), author = "Joe")]
pub struct Cli {
    #[command(flatten)]
    pub config: ConfigArgs,

    #[command(subcommand)]
    pub command: Option<Command>,
}
