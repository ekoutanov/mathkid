use mathkid::addition::Addition;
use mathkid::{Outcome, Question, Topic};
use std::io;
use tinyrand_std::thread_rand;

const USER: &str = "Anna";

fn main() {
    println!("Hi {}, I've got a few questions for you.", USER);

    let mut rand = thread_rand();
    let topic = Addition::new();
    for question_no in 1..=10 {
        let question = topic.ask(&mut rand);
        ask_question(question_no, question);
    }

    println!("Congratulations, you've answered all my questions!")
}

fn ask_question(question_no: u32, question: impl Question) {
    println!("Question {question_no}:");
    println!("{question}");
    loop {
        let answer = read_answer();
        match answer {
            None => {
                println!("You've skipped the question.");
                return;
            },
            Some(answer) => {
                let answer = answer.trim();
                match question.answer(answer) {
                    Outcome::Incorrect => {
                        println!("Your answer isn't quite right. Try again!")
                    }
                    Outcome::Invalid(err) => {
                        println!("There was a problem with your answer: {err}")
                    }
                    Outcome::Correct => {
                        println!("That's the right answer. Great work!");
                        return;
                    }
                }
            }
        }
    }
}

fn read_answer() -> Option<String> {
    let mut buf = String::new();
    let bytes = io::stdin().read_line(&mut buf).unwrap();
    match bytes {
        0 => None,
        _ => Some(buf),
    }
}
