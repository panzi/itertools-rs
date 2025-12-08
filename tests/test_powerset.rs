use itertools::{Powersetable, powerset};

#[test]
fn test_powerset0() {
    let values = "";
    let expected = [""].map(str::to_string);
    let actual: Vec<String> = powerset(values.chars()).map(
        |item| item.into_iter().collect()
    ).collect();

    assert_eq!(&expected[..], &actual[..]);
}

#[test]
fn test_powerset1() {
    let values = "a";
    let expected = ["", "a"].map(str::to_string);
    let actual: Vec<String> = values.chars().powerset().map(
        |item| item.into_iter().collect()
    ).collect();

    assert_eq!(&expected[..], &actual[..]);
}

#[test]
fn test_powerset3() {
    let values = "abc";
    let expected = ["", "a", "b", "c", "ab", "ac", "bc", "abc"].map(str::to_string);
    let actual: Vec<String> = powerset(values.chars()).map(
        |item| item.into_iter().collect()
    ).collect();

    assert_eq!(&expected[..], &actual[..]);
}
