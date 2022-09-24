use crate::syllabus::{presets, Course, Syllabus};
use crate::topic::{addition, subtraction, Module};
use std::collections::HashMap;

#[test]
fn presets() {
    let syllabus = presets::primary();
    assert!(!syllabus.courses.is_empty());
    for course in syllabus.courses.values() {
        assert!(!course.modules.is_empty());
    }
}

#[test]
fn course_get_topic_names() {
    let course = Course {
        modules: HashMap::from([
            (
                String::from("addition_2"),
                boxify(addition::presets::addition_2()),
            ),
            (
                String::from("subtraction_2"),
                boxify(subtraction::presets::subtraction_2()),
            ),
            (
                String::from("addition_1"),
                boxify(addition::presets::addition_1()),
            ),
        ]),
    };
    assert_eq!(vec!["addition", "subtraction"], course.get_topic_names());
}

#[test]
fn syllabus_get_topic_names() {
    let syllabus = Syllabus {
        courses: HashMap::from([
            (
                String::from("arithmetics_1"),
                Course {
                    modules: HashMap::from([(
                        String::from("addition_1"),
                        boxify(addition::presets::addition_2()),
                    )]),
                },
            ),
            (
                String::from("arithmetics_2"),
                Course {
                    modules: HashMap::from([(
                        String::from("subtraction_2"),
                        boxify(subtraction::presets::subtraction_2()),
                    )]),
                },
            ),
        ]),
    };
    assert_eq!(vec!["addition", "subtraction"], syllabus.get_topic_names());
}

fn boxify(t: impl Module + 'static) -> Box<dyn Module> {
    Box::new(t)
}
