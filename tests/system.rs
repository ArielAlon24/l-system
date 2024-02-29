use l_system::System;
use std::collections::HashMap;

#[test]
fn test_algae() {
    let mut rules = HashMap::new();
    rules.insert('A', "AB");
    rules.insert('B', "A");
    let start = "A";

    let system = System::new(rules, start.to_string());
    let iterator = system.into_iter();

    let expected = vec![
        "A",
        "AB",
        "ABA",
        "ABAAB",
        "ABAABABA",
        "ABAABABAABAAB",
        "ABAABABAABAABABAABABA",
        "ABAABABAABAABABAABABAABAABABAABAAB",
    ];

    let actual: Vec<_> = iterator.take(expected.len()).collect();

    assert_eq!(actual, expected);
}
