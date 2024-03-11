use l_system::system::{dump, Symbol, System};
use l_system::{state, symbol};
use std::collections::HashMap;

#[test]
fn test_algae() {
    let mut rules = HashMap::new();
    rules.insert(symbol!('A'), state!("AB"));
    rules.insert(symbol!('B'), state!("A"));

    let start = state!("A");

    let system = System::new(rules, start);
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

    let actual: Vec<_> = iterator.take(expected.len()).map(|s| dump(&s)).collect();

    assert_eq!(actual, expected);
}

#[test]
fn test_fractal_tree() {
    let mut rules = HashMap::new();
    rules.insert(symbol!('1'), state!("11"));
    rules.insert(symbol!('0'), state!("1[0]0"));
    let start = state!("0");

    let system = System::new(rules, start);
    let iterator = system.into_iter();

    let expected = vec![
        "0",
        "1[0]0",
        "11[1[0]0]1[0]0",
        "1111[11[1[0]0]1[0]0]11[1[0]0]1[0]0",
    ];

    let actual: Vec<_> = iterator.take(expected.len()).map(|s| dump(&s)).collect();

    assert_eq!(actual, expected);
}

#[test]
fn test_koch_curve() {
    let mut rules = HashMap::new();
    rules.insert(symbol!('F'), state!("F+F-F-F+F"));
    let start = vec![symbol!('F')];

    let system = System::new(rules, start);
    let iterator = system.into_iter();

    let expected = vec![
        "F",
        "F+F-F-F+F",
        "F+F-F-F+F+F+F-F-F+F-F+F-F-F+F-F+F-F-F+F+F+F-F-F+F",
        "F+F-F-F+F+F+F-F-F+F-F+F-F-F+F-F+F-F-F+F+F+F-F-F+F+\
F+F-F-F+F+F+F-F-F+F-F+F-F-F+F-F+F-F-F+F+F+F-F-F+F-\
F+F-F-F+F+F+F-F-F+F-F+F-F-F+F-F+F-F-F+F+F+F-F-F+F-\
F+F-F-F+F+F+F-F-F+F-F+F-F-F+F-F+F-F-F+F+F+F-F-F+F+\
F+F-F-F+F+F+F-F-F+F-F+F-F-F+F-F+F-F-F+F+F+F-F-F+F",
    ];

    let actual: Vec<_> = iterator.take(expected.len()).map(|s| dump(&s)).collect();

    assert_eq!(actual, expected);
}
