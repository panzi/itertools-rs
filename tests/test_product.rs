use itertools::{product, product4};

#[test]
fn test_product() {
    let first = ["foo", "bar", "baz"];
    let second = vec![1, 2, 3, 4];
    let expected = vec![
        ("foo", 1),
        ("foo", 2),
        ("foo", 3),
        ("foo", 4),

        ("bar", 1),
        ("bar", 2),
        ("bar", 3),
        ("bar", 4),

        ("baz", 1),
        ("baz", 2),
        ("baz", 3),
        ("baz", 4),
    ];

    assert_eq!(
        product(first.iter().cloned(), second.iter().cloned()).collect::<Vec<_>>(),
        expected
    );
}

#[test]
fn test_product_first_empty() {
    let first: [&str; 0] = [];
    let second = vec![1, 2, 3, 4];
    let expected = vec![];

    assert_eq!(
        product(first.iter().cloned(), second.iter().cloned()).collect::<Vec<_>>(),
        expected
    );
}

#[test]
fn test_product_second_empty() {
    let first = ["foo", "bar", "baz"];
    let second: Vec<i32> = vec![];
    let expected = vec![];

    assert_eq!(
        product(first.iter().cloned(), second.iter().cloned()).collect::<Vec<_>>(),
        expected
    );
}

#[test]
fn test_product_both_empty() {
    let first: [&str; 0] = [];
    let second: Vec<i32> = vec![];
    let expected = vec![];

    assert_eq!(
        product(first.iter().cloned(), second.iter().cloned()).collect::<Vec<_>>(),
        expected
    );
}

#[test]
fn test_product4() {
    let v1 = [1, 2];
    let v2 = ["a", "b"];
    let v3 = ['x', 'y'];
    let v4 = [0];
    let expected = vec![
        (1, "a", 'x', 0),
        (1, "a", 'y', 0),
        (1, "b", 'x', 0),
        (1, "b", 'y', 0),
        (2, "a", 'x', 0),
        (2, "a", 'y', 0),
        (2, "b", 'x', 0),
        (2, "b", 'y', 0)
    ];

    assert_eq!(
        product4(v1.into_iter(), v2.into_iter(), v3.into_iter(), v4.into_iter()).collect::<Vec<_>>(),
        expected
    );
}
