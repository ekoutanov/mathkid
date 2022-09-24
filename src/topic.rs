//! Definition of modules and questions.

pub mod addition;
pub mod subtraction;

use std::fmt::Display;
use tinyrand::{RandRange};

/// An unbounded stream of questions on a particular topic. Topics such as addition may be
/// taught in different grades, at different levels of difficulty. Modules are instantiations
/// of topics that take on a specific configuration.
pub trait Module {
    /// The name of this module's topic.
    fn topic_name(&self) -> String;

    /// Generates a question from this module.
    fn ask(&self, rand: &mut dyn RandRange<u32>) -> Box<dyn Question>;
}

/// A question.
pub trait Question: Display {
    /// Submits an answer to this question, assessing it to return an [`Outcome`].
    fn answer(&self, answer: &str) -> Outcome;
}

/// The outcome of answering a question.
#[derive(Debug, PartialEq, Eq)]
pub enum Outcome {
    Incorrect,
    Invalid(String),
    Correct
}

#[cfg(test)]
mod tests;