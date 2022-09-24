//! Definition of topics and questions.

use std::fmt::Display;
use tinyrand::{RandRange};

/// An unbounded stream of questions on a particular topic.
pub trait Topic {
    /// The name of this topic.
    fn name(&self) -> String;

    /// Generates a question on this topic.
    fn ask(&self, rand: &mut Box<dyn RandRange<u32>>) -> Box<dyn Question>;
}

/// A question.
pub trait Question: Display {
    /// Submits an answer to this question, assessing it to return an [`Outcome`].
    fn answer(&self, answer: &str) -> Outcome;
}

/// The outcome of answering a question.
#[derive(Debug)]
pub enum Outcome {
    Incorrect,
    Invalid(String),
    Correct
}

#[cfg(test)]
mod tests;