use crate::state::quiz::Quiz;

pub fn run() {
    let quiz = Quiz::load();
    println!("Question: {}", quiz.question())
}
