//! Questions on addition.

use crate::topic::{Outcome, Question, Module};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use tinyrand::RandRange;

/// The addition module.
pub struct Addition {
    config: Config,
}

/// Configuration for [`Addition`].
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

impl TryFrom<Config> for Addition {
    type Error = String;

    fn try_from(config: Config) -> Result<Self, Self::Error> {
        config.validate()?;
        Ok(Self { config })
    }
}

impl Module for Addition {
    fn topic_name(&self) -> String {
        String::from("addition")
    }

    fn ask(&self, rand: &mut dyn RandRange<u32>) -> Box<dyn Question> {
        let lhs = rand.next_range(self.config.min_val..self.config.max_val);
        let rhs = rand.next_range(self.config.min_val..self.config.max_val);
        Box::new(Sum { lhs, rhs })
    }
}

struct Sum {
    lhs: u32,
    rhs: u32,
}

impl Display for Sum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Can you add these two numbers for me.")?;
        write!(f, "{} + {} = ?", self.lhs, self.rhs)
    }
}

impl Question for Sum {
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
            Err(err) => Outcome::Invalid(err),
        }
    }
}

fn parse(answer: &str) -> Result<u32, String> {
    u32::from_str(answer)
        .map_err(|_| format!("'{answer}' does not appear to be a valid natural number"))
}

pub mod presets {
    use super::{Addition, Config};

    pub fn addition_1() -> Addition {
        Config {
            min_val: 0,
            max_val: 10,
        }
        .try_into().expect("misconfigured module")
    }

    pub fn addition_2() -> Addition {
        Config {
            min_val: 0,
            max_val: 9_999,
        }
        .try_into().expect("misconfigured module")
    }

    pub fn addition_3() -> Addition {
        Config {
            min_val: 0,
            max_val: 99_999_999,
        }
        .try_into().expect("misconfigured module")
    }
}

#[cfg(test)]
mod tests;
