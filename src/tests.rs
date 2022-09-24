use crate::{Outcome, Profile};

#[test]
fn outcome_implements_debug() {
    let s = format!("{:?}", Outcome::Invalid(String::from("foo")));
    assert!(s.contains("Invalid"));
    assert!(s.contains("foo"));
}

#[test]
fn profile_implements_debug() {
    let profile = Profile {
        first_name: "Fred".into(),
        course: "algebra".into(),
    };
    let s = format!("{:?}", profile);
    assert!(s.contains("Profile"));
    assert!(s.contains("first_name"));
    assert!(s.contains("Fred"));
    assert!(s.contains("course"));
    assert!(s.contains("algebra"));
}

#[test]
fn profile_to_json() {
    let profile = Profile {
        first_name: "Fred".into(),
        course: "algebra".into(),
    };
    let json = profile.to_json().unwrap();
    assert_eq!(r#"{"first_name":"Fred","course":"algebra"}"#, json);
}

#[test]
fn profile_from_json() {
    let json = r#"{"first_name":"Fred","course":"algebra"}"#;
    let profile = Profile::from_json(json).unwrap();
    assert_eq!(
        Profile {
            first_name: "Fred".into(),
            course: "algebra".into()
        },
        profile
    );
}

#[test]
fn test_sanitised_first_name() {
    #[derive(Debug)]
    struct Case {
        input: &'static str,
        expected: &'static str,
    }

    for case in vec![
        Case {
            input: "Emil",
            expected: "emil",
        },
        Case {
            input: "Mary-Jane",
            expected: "maryjane",
        },
        Case {
            input: "Алеша",
            expected: "alesha",
        },
        Case {
            input: "André",
            expected: "andre",
        },
        Case {
            input: "Цун Куй Чай",
            expected: "tsunkuichai",
        },
        Case {
            input: "Эмиль",
            expected: "emil",
        }
    ] {
        let profile = Profile {
            first_name: case.input.into(),
            course: "".into(),
        };
        assert_eq!(
            case.expected,
            &profile.sanitised_first_name(),
            "for {:?}",
            case
        )
    }
}
