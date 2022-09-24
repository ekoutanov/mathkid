use crate::{addition, Topic};
use itertools::Itertools;
use std::collections::HashMap;

pub struct Syllabus {
    pub courses: HashMap<String, Course>,
}

impl Syllabus {
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

pub struct Course {
    pub modules: HashMap<String, Box<dyn Topic>>,
}

impl Course {
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
    use super::{Course, Syllabus, addition};
    use crate::subtraction;
    use std::collections::HashMap;

    pub fn primary() -> Syllabus {
        Syllabus {
            courses: HashMap::from([
                (String::from("arithmetics_1"), arithmetics_1()),
                (String::from("arithmetics_2"), arithmetics_2()),
            ]),
        }
    }

    fn arithmetics_1() -> Course {
        Course {
            modules: HashMap::from([
                (String::from("addition"), addition::presets::addition_1()),
                (
                    String::from("subtraction"),
                    subtraction::presets::subtraction_1(),
                ),
            ]),
        }
    }

    fn arithmetics_2() -> Course {
        Course {
            modules: HashMap::from([
                (String::from("addition"), addition::presets::addition_2()),
                (
                    String::from("subtraction"),
                    subtraction::presets::subtraction_2(),
                ),
            ]),
        }
    }
}
