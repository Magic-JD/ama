use crate::cli::command::AddArgs;
use crate::state::quiz::Quiz;

pub fn run(args: AddArgs) {
    Quiz::add_round(args);
}
