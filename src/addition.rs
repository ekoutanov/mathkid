use std::fmt::{Display, Formatter};
use std::str::FromStr;
use tinyrand::{RandRange};
use crate::{Outcome, Question, Topic};

pub struct Addition {
    config: AdditionConfig
}

pub struct AdditionConfig {
    pub min_val: u32,
    pub max_val: u32
}

impl Addition {
    pub fn new(config: AdditionConfig) -> Self {
        Self { config }
    }
}

impl Topic for Addition {
    fn name(&self) -> String {
        String::from("addition")
    }

    fn ask(&self, rand: &mut Box<dyn RandRange<u32>>) -> Box<dyn Question> {
        let lhs = rand.next_range(self.config.min_val..self.config.max_val) as i32;
        let rhs = rand.next_range(self.config.min_val..self.config.max_val) as i32;
        Box::new(AdditionQuestion {
            lhs,
            rhs
        })
    }
}

pub struct AdditionQuestion {
    lhs: i32,
    rhs: i32
}

impl Display for AdditionQuestion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Can you add these two numbers for me.")?;
        write!(f, "{} + {} = ?", self.lhs, self.rhs)
    }
}

impl Question for AdditionQuestion {
    fn answer(&self, answer: &str) -> Outcome {
        match parse(answer) {
            Ok(answer) => {
                let expected = self.lhs + self.rhs;
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
    use super::*;

    pub fn addition_1() -> Box<dyn Topic> {
        Box::new(Addition::new(AdditionConfig {
            min_val: 0,
            max_val: 10
        }))
    }

    pub fn addition_2() -> Box<dyn Topic> {
        Box::new(Addition::new(AdditionConfig {
            min_val: 0,
            max_val: 9999
        }))
    }
}
