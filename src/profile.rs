//! Definition of a student's profile.

use serde::{Serialize, Deserialize};
use unidecode::unidecode;

/// A student's profile.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
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