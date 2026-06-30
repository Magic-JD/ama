use crate::cli::command::CreateArgs;
use crate::state::quiz::Quiz;

pub fn run(args: CreateArgs) {
    let quiz = Quiz::create(args);
    println!("Created quiz {}...", quiz.name);
}
