use crate::cli::command::{AddArgs, CreateArgs, EvalArgs};
use crate::database::quiz::{
    add_round_to_current_quiz, create_new_quiz, load_current_quiz, resolve_current_round,
};

pub struct Quiz {
    pub name: String,
    pub rounds: Vec<Round>,
}

pub struct Round {
    pub question: String,
    pub answer: String,
}

impl Quiz {
    pub fn load() -> Self {
        load_current_quiz().expect("There is no currently set quiz! Create one first!")
    }

    pub fn question(&self) -> &str {
        &self
            .rounds
            .first()
            .expect("There is no current question! Add some more!")
            .question
    }

    pub fn answer(&self) -> &str {
        &self
            .rounds
            .first()
            .expect("There are no current answers! Add some more!")
            .answer
    }

    pub fn create(args: CreateArgs) -> Self {
        create_new_quiz(args)
    }

    pub fn add_round(args: AddArgs) -> Self {
        add_round_to_current_quiz(args).expect("There is no currently set quiz! Create one first!")
    }

    pub fn evaluate(args: EvalArgs) -> Self {
        resolve_current_round(args).expect("There is no currently set quiz! Create one first!")
    }
}
