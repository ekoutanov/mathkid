use std::fmt::{Display, Formatter};
use std::str::FromStr;
use tinyrand::{RandRange};
use crate::topic::{Outcome, Question, Topic};

pub struct Subtraction {
    config: Config
}

pub struct Config {
    pub min_val: u32,
    pub max_val: u32
}

impl Subtraction {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

impl Topic for Subtraction {
    fn name(&self) -> String {
        String::from("subtraction")
    }

    fn ask(&self, rand: &mut Box<dyn RandRange<u32>>) -> Box<dyn Question> {
        let lhs = rand.next_range(self.config.min_val..self.config.max_val);
        let rhs = if lhs == 0 { 0 } else { rand.next_range(0..lhs) };
        Box::new(QuestionBody {
            lhs: lhs.try_into().unwrap(),
            rhs: rhs.try_into().unwrap()
        })
    }
}

pub struct QuestionBody {
    lhs: i32,
    rhs: i32
}

impl Display for QuestionBody {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Can you subtract these two numbers for me.")?;
        write!(f, "{} – {} = ?", self.lhs, self.rhs)
    }
}

impl Question for QuestionBody {
    fn answer(&self, answer: &str) -> Outcome {
        match parse(answer) {
            Ok(answer) => {
                let expected = self.lhs - self.rhs;
                if answer == expected {
                    Outcome::Correct
                } else {
                    Outcome::Incorrect
                }
            }
            Err(err) => Outcome::Invalid(err)
        }
    }
}

fn parse(answer: &str) -> Result<i32, String> {
    i32::from_str(answer).map_err(|_| format!("'{answer}' does not appear to be a valid number"))
}

pub mod presets {
    use super::{Subtraction, Config, Topic};

    pub fn subtraction_1() -> Box<dyn Topic> {
        Box::new(Subtraction::new(Config {
            min_val: 0,
            max_val: 10
        }))
    }

    pub fn subtraction_2() -> Box<dyn Topic> {
        Box::new(Subtraction::new(Config {
            min_val: 0,
            max_val: 9999
        }))
    }
}
