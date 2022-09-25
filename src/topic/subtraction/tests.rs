use crate::topic::subtraction::{presets, Config, Subtraction};
use crate::topic::{Module, Outcome};
use tinyrand_alloc::Mock;

#[test]
fn name() {
    let module = presets::subtraction_1();
    assert_eq!("subtraction", module.topic_name());
}

#[test]
fn display_ask_answer_nonnegative() {
    let module = Subtraction::try_from(Config {
        min_val: 10,
        max_val: 30,
        allow_negative: false,
    })
    .unwrap();

    let rand_nums = vec![17, 11];
    let mut rand = Mock::default().with_next_lim_u128(|surrogate, lim| {
        if surrogate.state().next_lim_u128_invocations() == 0 {
            assert_eq!(20, lim);
        } else {
            assert_eq!(27, lim);
        }
        rand_nums[surrogate.state().next_lim_u128_invocations() as usize]
    });

    let question = module.ask(&mut rand);
    let s = format!("{}", question);
    assert!(
        s.contains("Can you subtract these two numbers for me."),
        "{}",
        s
    );
    assert!(s.contains("27 – 11"), "{}", s);

    assert_eq!(
        Outcome::Invalid("'foo' does not appear to be a valid integer".into()),
        question.answer("foo")
    );
    assert_eq!(Outcome::Incorrect, question.answer("15"));
    assert_eq!(Outcome::Incorrect, question.answer("17"));
    assert_eq!(Outcome::Correct, question.answer("16"));
}

#[test]
fn question_with_zero_nonnegative() {
    let module = Subtraction::try_from(Config {
        min_val: 0,
        max_val: 30,
        allow_negative: false,
    })
    .unwrap();

    let mut rand = Mock::default().with_next_lim_u128(|_, lim| {
        assert_eq!(30, lim);
        0
    });
    let question = module.ask(&mut rand);
    let s = format!("{}", question);
    assert!(s.contains("0 – 0"), "{}", s);
    assert_eq!(Outcome::Correct, question.answer("0"));
    assert_eq!(1, rand.state().next_lim_u128_invocations());
}

#[test]
fn display_ask_answer_negative() {
    let module = Subtraction::try_from(Config {
        min_val: 10,
        max_val: 30,
        allow_negative: true,
    })
    .unwrap();

    let rand_nums = vec![17, 29];
    let mut rand = Mock::default().with_next_lim_u128(|surrogate, lim| {
        if surrogate.state().next_lim_u128_invocations() == 0 {
            assert_eq!(20, lim);
        } else {
            assert_eq!(30, lim);
        }
        rand_nums[surrogate.state().next_lim_u128_invocations() as usize]
    });

    let question = module.ask(&mut rand);
    let s = format!("{}", question);
    assert!(
        s.contains("Can you subtract these two numbers for me."),
        "{}",
        s
    );
    assert!(s.contains("27 – 29"), "{}", s);

    assert_eq!(
        Outcome::Invalid("'foo' does not appear to be a valid integer".into()),
        question.answer("foo")
    );
    assert_eq!(Outcome::Incorrect, question.answer("-1"));
    assert_eq!(Outcome::Incorrect, question.answer("-3"));
    assert_eq!(Outcome::Correct, question.answer("-2"));
}

#[test]
fn invalid_config() {
    let module = Subtraction::try_from(Config {
        min_val: 10,
        max_val: 10,
        allow_negative: false,
    });
    assert_eq!("min_val must be less than max_val", module.err().unwrap());

    let module = Subtraction::try_from(Config {
        min_val: 10,
        max_val: (u32::MAX << 1) + 1,
        allow_negative: false,
    });
    assert_eq!(
        format!("max_val cannot exceed {}", u32::MAX << 1),
        module.err().unwrap()
    );
}

#[test]
fn presets() {
    presets::subtraction_1();
    presets::subtraction_2();
    presets::subtraction_3();
}
