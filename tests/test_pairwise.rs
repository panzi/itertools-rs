use itertools::{Pairwiseable, pairwise};

#[test]
fn test_pairwise0() {
    let expected = [(0u32, 032); 0];
    let actual: Vec<(u32, u32)> = pairwise([].into_iter()).collect();

    assert_eq!(&expected[..], &actual[..]);
    assert_eq!(pairwise([0; 0].into_iter()).count(), expected.len());
    assert_eq!(pairwise([0; 0].into_iter()).size_hint(), (expected.len(), Some(expected.len())));
}

#[test]
fn test_pairwise1() {
    let values = "a";
    let expected = [(' ', ' '); 0];
    let actual: Vec<_> = pairwise(values.chars()).collect();

    assert_eq!(&expected[..], &actual[..]);
    assert_eq!(pairwise(values.chars()).count(), expected.len());
    assert_eq!(pairwise(values.chars()).size_hint(), (expected.len(), Some(expected.len())));
}

#[test]
fn test_pairwise2() {
    let values = "ab";
    let expected = [('a', 'b')];
    let actual: Vec<_> = values.chars().pairwise().collect();

    assert_eq!(&expected[..], &actual[..]);
    assert_eq!(values.chars().pairwise().count(), expected.len());
    assert_eq!(values.chars().pairwise().size_hint(), (1, Some(expected.len())));
}

#[test]
fn test_pairwise5() {
    let values = "abcde";
    let expected = [('a', 'b'), ('b', 'c'), ('c', 'd'), ('d', 'e')];
    let actual: Vec<_> = pairwise(values.chars()).collect();

    assert_eq!(&expected[..], &actual[..]);
    assert_eq!(pairwise(values.chars()).count(), expected.len());
    assert_eq!(pairwise(values.chars()).size_hint(), (1, Some(expected.len())));
}
