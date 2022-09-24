//! High-level organisation of learning material.

use crate::topic::Topic;
use itertools::Itertools;
use std::collections::HashMap;

/// The highest level of organisation. A syllabus specifies a set of named of courses that
/// are available to students.
pub struct Syllabus {
    /// Mapping of course names to the courses.
    pub courses: HashMap<String, Course>,
}

impl Syllabus {
    /// Obtains an ordered set of topic names that are taught across all courses in this syllabus.
    pub fn get_topic_names(&self) -> Vec<String> {
        self.courses
            .values()
            .into_iter()
            .flat_map(Course::get_topic_names)
            .unique()
            .sorted()
            .collect()
    }
}

/// A course specifies a set of named **modules**. A module is a [`Topic`] instance configured
/// specifically for a course.
///
/// Note that a topic is a parametrised stream of questions. Topics such as addition may be
/// taught in different grades, at different levels of difficulty.
pub struct Course {
    /// Mapping of module names to preconfigured topics.
    pub modules: HashMap<String, Box<dyn Topic>>,
}

impl Course {
    /// Obtains an ordered set of topic names that are taught in this course.
    pub fn get_topic_names(&self) -> Vec<String> {
        self.modules
            .values()
            .into_iter()
            .map(|topic| topic.name())
            .unique()
            .sorted()
            .collect()
    }
}

pub mod presets {
    use super::{Course, Syllabus};
    use crate::topic::{addition, subtraction, Topic};
    use std::collections::HashMap;

    pub fn primary() -> Result<Syllabus, String> {
        Ok(Syllabus {
            courses: HashMap::from([
                (String::from("arithmetics_1"), arithmetics_1()?),
                (String::from("arithmetics_2"), arithmetics_2()?),
            ]),
        })
    }

    fn arithmetics_1() -> Result<Course, String> {
        Ok(Course {
            modules: HashMap::from([
                (
                    String::from("addition"),
                    boxify(addition::presets::addition_1()?),
                ),
                (
                    String::from("subtraction"),
                    boxify(subtraction::presets::subtraction_1()?),
                ),
            ]),
        })
    }

    fn arithmetics_2() -> Result<Course, String> {
        Ok(Course {
            modules: HashMap::from([
                (
                    String::from("addition"),
                    boxify(addition::presets::addition_2()?),
                ),
                (
                    String::from("subtraction"),
                    boxify(subtraction::presets::subtraction_2()?),
                ),
            ]),
        })
    }

    fn boxify(t: impl Topic + 'static) -> Box<dyn Topic> {
        Box::new(t)
    }
}

#[cfg(test)]
mod tests;
