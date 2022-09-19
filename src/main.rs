use std::io;
use std::str::FromStr;
use tinyrand::{RandRange};

const USER: &str = "Anna";

fn main() {
    println!("Hi {}, I've got a few questions for you.", USER);

    for q in 1..=10 {
        ask_question(q);
    }

    println!("Congratulations, you've answered all my questions!")
}

fn ask_question(question_no: u32) {
    println!("Question {question_no}: Can you add these two numbers for me:");
    let n1 = tinyrand_std::thread_rand().next_range(1..9999u32);
    let n2 = tinyrand_std::thread_rand().next_range(1..9999u32);
    println!("  {n1} + {n2}");
    let expected = n1 + n2;
    loop {
        let answer = read_answer();
        let answer = answer.trim();
        let answer = match u32::from_str(answer) {
            Ok(num) => num,
            Err(_) => {
                println!("The answer '{answer}' does not appear to be a number; please try again.");
                continue;
            }
        };
        if answer != expected {
            println!("Your answer {answer} is incorrect. Have another go!");
            continue;
        }
        println!("Correct, {n1} + {n2} = {answer}");
        break;
    }
}

fn read_answer() -> String {
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("Error");
    buf
}
