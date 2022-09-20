pub mod addition;

use std::fmt::Display;
use tinyrand::Rand;

pub trait Topic {
    type Question: Question;

    fn ask(&self, rand: &mut impl Rand) -> Self::Question;
}

pub trait Question: Display {
    fn answer(&self, answer: &str) -> Outcome;
}

#[derive(Debug)]
pub enum Outcome {
    Incorrect,
    Invalid(String),
    Correct
}