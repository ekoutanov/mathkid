use tinyrand_alloc::Mock;
use crate::topic::subtraction::{Config, presets, Subtraction};
use crate::topic::{Outcome, Topic};

#[test]
fn name() {
    let topic = presets::subtraction_1().unwrap();
    assert_eq!("subtraction", topic.name());
}

#[test]
fn display_ask_answer() {
    let topic = Subtraction::try_from(Config {
        min_val: 10,
        max_val: 30
    }).unwrap();

    let rand_nums = vec![17, 11];
    let mut rand = Mock::default()
        .with_next_lim_u128(|surrogate, lim| {
            if surrogate.state().next_lim_u128_invocations() == 0 {
                assert_eq!(20, lim);
            } else {
                assert_eq!(27, lim);
            }
            rand_nums[surrogate.state().next_lim_u128_invocations() as usize]
        });

    let question = topic.ask(&mut rand);
    let s = format!("{}", question);
    assert!(s.contains("Can you subtract these two numbers for me."), "{}", s);
    assert!(s.contains("27 – 11"), "{}", s);

    assert_eq!(Outcome::Invalid("'foo' does not appear to be a valid integer".into()), question.answer("foo"));
    assert_eq!(Outcome::Incorrect, question.answer("15"));
    assert_eq!(Outcome::Incorrect, question.answer("17"));
    assert_eq!(Outcome::Correct, question.answer("16"));
}

#[test]
fn question_with_empty_range() {
    let topic = Subtraction::try_from(Config {
        min_val: 0,
        max_val: 30
    }).unwrap();

    let rand_nums = vec![0, 0];
    let mut rand = Mock::default()
        .with_next_lim_u128(|surrogate, lim| {
            if surrogate.state().next_lim_u128_invocations() == 0 {
                assert_eq!(30, lim);
            } else {
                assert_eq!(0, lim);
            }
            rand_nums[surrogate.state().next_lim_u128_invocations() as usize]
        });

    let question = topic.ask(&mut rand);
    let s = format!("{}", question);
    assert!(s.contains("0 – 0"), "{}", s);
    assert_eq!(Outcome::Correct, question.answer("0"));
}

#[test]
fn invalid_config() {
    let topic = Subtraction::try_from(Config {
        min_val: 10,
        max_val: 10
    });
    assert_eq!("min_val must be less than max_val", topic.err().unwrap());

    let topic = Subtraction::try_from(Config {
        min_val: 10,
        max_val: (u32::MAX << 1) + 1
    });
    assert_eq!(format!("max_val cannot exceed {}", u32::MAX << 1), topic.err().unwrap());
}

#[test]
fn presets() {
    presets::subtraction_1().unwrap();
    presets::subtraction_2().unwrap();
}