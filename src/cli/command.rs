use clap::Parser;

#[derive(Debug, Parser)]
pub struct ConfigArgs {
    // No args for now
}

#[derive(Debug, Parser)]
pub struct TaskArgs {
    #[arg(long, help = "Generate a default configuration file")]
    pub generate_config: bool,
}

#[derive(Debug, Parser)]
#[command(name = "ama")]
#[command(version = env!("CARGO_PKG_VERSION"), author = "Joe")]
pub struct Cli {
    #[command(flatten)]
    pub config: ConfigArgs,

    #[command(flatten)]
    pub task: TaskArgs,
}
