//! Questions on subtraction.

use crate::topic::{Outcome, Question, Module};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use tinyrand::RandRange;

/// The subtraction module.
pub struct Subtraction {
    config: Config,
}

/// Configuration for [`Subtraction`].
pub struct Config {
    /// The smallest number that will be asked.
    pub min_val: u32,

    /// The largest number that will be asked.
    pub max_val: u32,
}

impl Config {
    /// Validates the given config.
    ///
    /// # Errors
    /// If the config is invalid.
    pub fn validate(&self) -> Result<(), String> {
        const MAX_MAX_VAL: u32 = u32::MAX << 1;
        if self.min_val >= self.max_val {
            return Err("min_val must be less than max_val".into());
        }
        if self.max_val > MAX_MAX_VAL {
            return Err(format!("max_val cannot exceed {MAX_MAX_VAL}"));
        }
        Ok(())
    }
}

impl TryFrom<Config> for Subtraction {
    type Error = String;

    fn try_from(config: Config) -> Result<Self, Self::Error> {
        config.validate()?;
        Ok(Self { config })
    }
}

impl Module for Subtraction {
    fn topic_name(&self) -> String {
        String::from("subtraction")
    }

    fn ask(&self, rand: &mut dyn RandRange<u32>) -> Box<dyn Question> {
        let lhs = rand.next_range(self.config.min_val..self.config.max_val);
        let rhs = if lhs == 0 { 0 } else { rand.next_range(0..lhs) };
        Box::new(Difference { lhs, rhs })
    }
}

struct Difference {
    lhs: u32,
    rhs: u32,
}

impl Display for Difference {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Can you subtract these two numbers for me.")?;
        write!(f, "{} – {} = ?", self.lhs, self.rhs)
    }
}

impl Question for Difference {
    fn answer(&self, answer: &str) -> Outcome {
        match parse(answer) {
            Ok(answer) => {
                let expected = i32::try_from(self.lhs).unwrap() - i32::try_from(self.rhs).unwrap();
                if answer == expected {
                    Outcome::Correct
                } else {
                    Outcome::Incorrect
                }
            }
            Err(err) => Outcome::Invalid(err),
        }
    }
}

fn parse(answer: &str) -> Result<i32, String> {
    i32::from_str(answer).map_err(|_| format!("'{answer}' does not appear to be a valid integer"))
}

pub mod presets {
    use super::{Config, Subtraction};

    pub fn subtraction_1() -> Subtraction {
        Config {
            min_val: 0,
            max_val: 10,
        }
        .try_into().expect("misconfigured module")
    }

    #[allow(missing_docs)]
    pub fn subtraction_2() -> Subtraction {
        Config {
            min_val: 0,
            max_val: 9999,
        }
        .try_into().expect("misconfigured module")
    }
}

#[cfg(test)]
mod tests;