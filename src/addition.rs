use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use tinyrand::{RandRange};
use crate::{Outcome, Question, Topic};

pub struct Addition;

impl Addition {
    pub fn new() -> Self {
        Self {}
    }
}

impl Topic for Addition {
    fn ask(&self, rand: &RefCell<Box<dyn RandRange<u32>>>) -> Box<dyn Question> {
        let mut rand = rand.borrow_mut();
        let lhs = rand.next_range(1..9999u32) as i32;
        let rhs = rand.next_range(1..9999u32) as i32;
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