use crate::cli::command::EvalArgs;
use crate::state::quiz::Quiz;

pub fn run(args: EvalArgs) {
    Quiz::evaluate(args);
}
