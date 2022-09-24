use super::Outcome;

#[test]
fn outcome_implements_debug() {
    let s = format!("{:?}", Outcome::Invalid(String::from("foo")));
    assert!(s.contains("Invalid"));
    assert!(s.contains("foo"));
}