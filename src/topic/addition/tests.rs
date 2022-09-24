use tinyrand::RandRange;
use crate::topic::addition::{Addition, Config};
use crate::topic::addition::presets::addition_1;
use tinyrand_alloc::mock::Mock;
use crate::topic::Topic;

#[test]
fn name() {
    let topic = addition_1();
    assert_eq!("addition", topic.name());
}

#[test]
fn ask() {
    let topic = Addition::new(Config {
        min_val: 10,
        max_val: 20
    });
    let rands = vec![12, 13];
    let rand = Mock::default()
        .with_next_lim_u128(move |surrogate, _| {
            rands[surrogate.state().next_lim_u128_invocations() as usize]
        });

    let rand: Box<dyn RandRange<u32>> = Box::new(rand);
    topic.ask(&mut Box::new(rand));
}