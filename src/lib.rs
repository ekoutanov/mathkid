pub mod addition;
pub mod syllabus;

use std::fmt::Display;
use tinyrand::{RandRange};
use serde::{Serialize, Deserialize};
use unidecode::unidecode;

pub trait Topic {
    fn name(&self) -> String;

    fn ask(&self, rand: &mut Box<dyn RandRange<u32>>) -> Box<dyn Question>;
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub first_name: String,
    pub course: String,
}

impl Profile {
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|err| err.to_string())
    }

    pub fn from_json(json: &str) -> Result<Profile, String> {
        serde_json::from_str(json).map_err(|err| err.to_string())
    }

    pub fn sanitised_first_name(&self) -> String {
        let transliterated = unidecode(&self.first_name);
        transliterated
            .to_ascii_lowercase()
            .chars()
            .filter(|ch| ch.is_ascii_alphabetic())
            .fold(String::new(), |acc, ch| acc + &ch.to_string())
    }
}