use l_system::{dump, Character, System};
use std::collections::HashMap;

#[test]
fn test_algae() {
    let mut rules = HashMap::new();
    rules.insert(
        Character::Var('A'),
        vec![Character::Var('A'), Character::Var('B')],
    );
    rules.insert(Character::Var('B'), vec![Character::Var('A')]);

    let constants = vec![];
    let start = vec![Character::Var('A')];

    let system = System::new(rules, constants, start);
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

    let actual: Vec<_> = iterator.take(expected.len()).map(|s| dump(s)).collect();

    assert_eq!(actual, expected);
}

#[test]
fn test_fractal_tree() {
    let mut rules = HashMap::new();
    rules.insert(
        Character::Var('1'),
        vec![Character::Var('1'), Character::Var('1')],
    );
    rules.insert(
        Character::Var('0'),
        vec![
            Character::Var('1'),
            Character::Push,
            Character::Var('0'),
            Character::Pop,
            Character::Var('0'),
        ],
    );

    let constants = vec![Character::Push, Character::Pop];
    let start = vec![Character::Var('0')];

    let system = System::new(rules, constants, start);
    let iterator = system.into_iter();

    let expected = vec![
        "0",
        "1[0]0",
        "11[1[0]0]1[0]0",
        "1111[11[1[0]0]1[0]0]11[1[0]0]1[0]0",
    ];

    let actual: Vec<_> = iterator.take(expected.len()).map(|s| dump(s)).collect();

    assert_eq!(actual, expected);
}

#[test]
fn test_koch_curve() {
    let mut rules = HashMap::new();
    rules.insert(
        Character::Draw,
        vec![
            Character::Draw,
            Character::Left,
            Character::Draw,
            Character::Right,
            Character::Draw,
            Character::Right,
            Character::Draw,
            Character::Left,
            Character::Draw,
        ],
    );

    let constants = vec![Character::Left, Character::Right];
    let start = vec![Character::Draw];

    let system = System::new(rules, constants, start);
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

    let actual: Vec<_> = iterator.take(expected.len()).map(|s| dump(s)).collect();

    assert_eq!(actual, expected);
}
