use itertools::{Accumulatable, accumulate};

#[test]
fn test_accumulate0() {
    let values   = [0; 0];
    let expected = [0; 0];
    let actual: Vec<_> = accumulate(values.into_iter()).collect();

    assert_eq!(&expected[..], &actual[..]);
    assert_eq!(accumulate(values.into_iter()).count(), expected.len());
    assert_eq!(accumulate(values.into_iter()).size_hint(), expected.iter().size_hint());
}

#[test]
fn test_accumulate1() {
    let values   = [3];
    let expected = [3];
    let actual: Vec<_> = values.into_iter().accumulate().collect();

    assert_eq!(&expected[..], &actual[..]);
    assert_eq!(accumulate(values.into_iter()).count(), expected.len());
    assert_eq!(accumulate(values.into_iter()).size_hint(), expected.iter().size_hint());
}

#[test]
fn test_accumulate5() {
    let values   = [1, 0, 3, 1, 2];
    let expected = [1, 1, 4, 5, 7];
    let actual: Vec<_> = values.into_iter().accumulate().collect();

    assert_eq!(&expected[..], &actual[..]);
    assert_eq!(accumulate(values.into_iter()).count(), expected.len());
    assert_eq!(accumulate(values.into_iter()).size_hint(), expected.iter().size_hint());
}
