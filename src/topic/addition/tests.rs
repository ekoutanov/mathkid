use tinyrand_alloc::mock::Mock;
use crate::topic::addition::{Addition, Config, presets};
use crate::topic::{Module, Outcome};

#[test]
fn name() {
    let module = presets::addition_1();
    assert_eq!("addition", module.topic_name());
}

#[test]
fn display_ask_answer() {
    let module = Addition::try_from(Config {
        min_val: 10,
        max_val: 30
    }).unwrap();

    let rand_nums = vec![12, 13];
    let mut rand = Mock::default()
        .with_next_lim_u128(|surrogate, lim| {
            assert_eq!(20, lim);
            rand_nums[surrogate.state().next_lim_u128_invocations() as usize]
        });

    let question = module.ask(&mut rand);
    let s = format!("{}", question);
    assert!(s.contains("Can you add these two numbers for me."), "{}", s);
    assert!(s.contains("22 + 23"), "{}", s);

    assert_eq!(Outcome::Invalid("'foo' does not appear to be a valid natural number".into()), question.answer("foo"));
    assert_eq!(Outcome::Invalid("'-1' does not appear to be a valid natural number".into()), question.answer("-1"));
    assert_eq!(Outcome::Incorrect, question.answer("44"));
    assert_eq!(Outcome::Incorrect, question.answer("46"));
    assert_eq!(Outcome::Correct, question.answer("45"));
}

#[test]
fn invalid_config() {
    let module = Addition::try_from(Config {
        min_val: 10,
        max_val: 10
    });
    assert_eq!("min_val must be less than max_val", module.err().unwrap());

    let module = Addition::try_from(Config {
        min_val: 10,
        max_val: (u32::MAX << 1) + 1
    });
    assert_eq!(format!("max_val cannot exceed {}", u32::MAX << 1), module.err().unwrap());
}

#[test]
fn presets() {
    presets::addition_1();
    presets::addition_2();
}