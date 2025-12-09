use itertools::ZippableLongest;

#[test]
fn test_zip_longest_both_empty() {
    let first = "";
    let second = "";
    let expected = [(' ', ' '); 0];
    let actual = first.chars().zip_longest(second.chars()).collect::<Vec<_>>();

    assert_eq!(&expected[..], &actual[..]);
    assert_eq!(first.chars().zip_longest(second.chars()).count(), expected.len());
    assert_eq!(first.chars().zip_longest(second.chars()).size_hint(), (expected.len(), Some(expected.len())));
}

#[test]
fn test_zip_longest_first_empty() {
    let first = "";
    let second = "abc";
    let expected = [('\0', 'a'), ('\0', 'b'), ('\0', 'c')];
    let actual = first.chars().zip_longest(second.chars()).collect::<Vec<_>>();

    assert_eq!(&expected[..], &actual[..]);
    assert_eq!(first.chars().zip_longest(second.chars()).count(), expected.len());
    assert_eq!(first.chars().zip_longest(second.chars()).size_hint(), (1, Some(expected.len())));
}

#[test]
fn test_zip_longest_second_empty() {
    let first = "abc";
    let second = "";
    let expected = [('a', '\0'), ('b', '\0'), ('c', '\0')];
    let actual = first.chars().zip_longest(second.chars()).collect::<Vec<_>>();

    assert_eq!(&expected[..], &actual[..]);
    assert_eq!(first.chars().zip_longest(second.chars()).count(), expected.len());
    assert_eq!(first.chars().zip_longest(second.chars()).size_hint(), (1, Some(expected.len())));
}

#[test]
fn test_zip_longest() {
    let first = "abc";
    let second = "0123";
    let expected = [('a', '0'), ('b', '1'), ('c', '2'), ('\0', '3')];
    let actual = first.chars().zip_longest(second.chars()).collect::<Vec<_>>();

    assert_eq!(&expected[..], &actual[..]);
    assert_eq!(first.chars().zip_longest(second.chars()).count(), expected.len());
    assert_eq!(first.chars().zip_longest(second.chars()).size_hint(), (1, Some(expected.len())));
}
