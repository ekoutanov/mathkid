//! High-level organisation of learning material.

use crate::topic::Module;
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

/// A course specifies a set of named modules.
pub struct Course {
    /// Mapping of module names to preconfigured modules.
    pub modules: HashMap<String, Box<dyn Module>>,
}

impl Course {
    /// Obtains an ordered set of topic names that are taught in this course.
    pub fn get_topic_names(&self) -> Vec<String> {
        self.modules
            .values()
            .into_iter()
            .map(|topic| topic.topic_name())
            .unique()
            .sorted()
            .collect()
    }
}

pub mod presets {
    use super::{Course, Syllabus};
    use crate::topic::{addition, subtraction, Module};
    use std::collections::HashMap;

    pub fn primary() -> Syllabus {
        Syllabus {
            courses: HashMap::from([
                (String::from("arithmetics_1"), arithmetics_1()),
                (String::from("arithmetics_2"), arithmetics_2()),
                (String::from("arithmetics_3"), arithmetics_3()),
            ]),
        }
    }

    fn arithmetics_1() -> Course {
        Course {
            modules: HashMap::from([
                (
                    String::from("addition_1"),
                    boxify(addition::presets::addition_1()),
                ),
                (
                    String::from("subtraction_1"),
                    boxify(subtraction::presets::subtraction_1()),
                ),
            ]),
        }
    }

    fn arithmetics_2() -> Course {
        Course {
            modules: HashMap::from([
                (
                    String::from("addition_2"),
                    boxify(addition::presets::addition_2()),
                ),
                (
                    String::from("subtraction_2"),
                    boxify(subtraction::presets::subtraction_2()),
                ),
            ]),
        }
    }

    fn arithmetics_3() -> Course {
        Course {
            modules: HashMap::from([
                (
                    String::from("addition_3"),
                    boxify(addition::presets::addition_3()),
                ),
                (
                    String::from("subtraction_3"),
                    boxify(subtraction::presets::subtraction_3()),
                ),
            ]),
        }
    }

    fn boxify(t: impl Module + 'static) -> Box<dyn Module> {
        Box::new(t)
    }
}

#[cfg(test)]
mod tests;
