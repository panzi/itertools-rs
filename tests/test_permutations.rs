use itertools::{permutations, Permutable};

#[test]
fn test_permutations0() {
    let values = [0; 0];
    let expected: Vec<Vec<&u32>> = vec![Vec::new()];
    let actual: Vec<Vec<&u32>> = values.permutations().collect();

    assert_eq!(&expected[..], &actual[..]);
    assert_eq!(values.permutations().count(), expected.len());
    assert_eq!(values.as_slice().permutations().size_hint(), (expected.len(), Some(expected.len())));
}

#[test]
fn test_permutations1() {
    let values = [0];
    let expected: Vec<Vec<&u32>> = vec![values.iter().collect()];
    let actual: Vec<Vec<&u32>> = values.as_slice().permutations().collect();

    assert_eq!(&expected[..], &actual[..]);
    assert_eq!(values.as_slice().permutations().count(), expected.len());
    assert_eq!(values.as_slice().permutations().size_hint(), (expected.len(), Some(expected.len())));
}

#[test]
fn test_permutations5() {
    let values = "abcde".chars().collect::<Vec<_>>();

    let mut expected = [
        "abcde", "abced", "abdce", "abdec", "abecd", "abedc", "acbde", "acbed",
        "acdbe", "acdeb", "acebd", "acedb", "adbce", "adbec", "adcbe", "adceb",
        "adebc", "adecb", "aebcd", "aebdc", "aecbd", "aecdb", "aedbc", "aedcb",
        "bacde", "baced", "badce", "badec", "baecd", "baedc", "bcade", "bcaed",
        "bcdae", "bcdea", "bcead", "bceda", "bdace", "bdaec", "bdcae", "bdcea",
        "bdeac", "bdeca", "beacd", "beadc", "becad", "becda", "bedac", "bedca",
        "cabde", "cabed", "cadbe", "cadeb", "caebd", "caedb", "cbade", "cbaed",
        "cbdae", "cbdea", "cbead", "cbeda", "cdabe", "cdaeb", "cdbae", "cdbea",
        "cdeab", "cdeba", "ceabd", "ceadb", "cebad", "cebda", "cedab", "cedba",
        "dabce", "dabec", "dacbe", "daceb", "daebc", "daecb", "dbace", "dbaec",
        "dbcae", "dbcea", "dbeac", "dbeca", "dcabe", "dcaeb", "dcbae", "dcbea",
        "dceab", "dceba", "deabc", "deacb", "debac", "debca", "decab", "decba",
        "eabcd", "eabdc", "eacbd", "eacdb", "eadbc", "eadcb", "ebacd", "ebadc",
        "ebcad", "ebcda", "ebdac", "ebdca", "ecabd", "ecadb", "ecbad", "ecbda",
        "ecdab", "ecdba", "edabc", "edacb", "edbac", "edbca", "edcab", "edcba",
    ];
    expected.sort();

    let mut actual = permutations(&values).map(
        |item| item.iter().cloned().collect::<String>()
    ).collect::<Vec<_>>();
    actual.sort();

    assert_eq!(&expected[..], &actual[..]);

    for n in 0..expected.len() {
        let iter = permutations(&values);
        let iter = iter.skip(n);
        assert_eq!(iter.count(), expected.len() - n);

        let iter = permutations(&values);
        let iter = iter.skip(n);
        let upper = expected.len() - n;
        assert_eq!(iter.size_hint(), (upper, Some(upper)));
    }
}
