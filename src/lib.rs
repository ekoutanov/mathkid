//! Definition of topics and questions.

pub mod addition;
pub mod subtraction;
pub mod syllabus;

use std::fmt::Display;
use tinyrand::{RandRange};
use serde::{Serialize, Deserialize};
use unidecode::unidecode;

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

/// A student's profile.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Profile {
    pub first_name: String,
    pub course: String,
}

impl Profile {
    /// Converts the profile to its JSON representation.
    ///
    /// # Errors
    /// If the object could not be serialized to JSON.
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|err| err.to_string())
    }

    /// Loads a profile from its JSON representation.
    ///
    /// # Errors
    /// If the object could not be deserialized.
    pub fn from_json(json: &str) -> Result<Profile, String> {
        serde_json::from_str(json).map_err(|err| err.to_string())
    }

    /// Obtains a sanitised 'slug' from the `first_name` field of the profile.
    pub fn sanitised_first_name(&self) -> String {
        sanitise(&self.first_name)
    }
}

/// Obtains a 'slug' from the given string, comprising transliterated alphabetic ASCII characters.
fn sanitise(s: &str) -> String {
    let transliterated = unidecode(s);
    transliterated
        .to_ascii_lowercase()
        .chars()
        .filter(char::is_ascii_alphabetic)
        .fold(String::new(), |acc, ch| acc + &ch.to_string())
}

#[cfg(test)]
mod tests;