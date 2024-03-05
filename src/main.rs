use l_system::graphics::{Config, Visualizer};
use l_system::{Character, System};
use std::collections::HashMap;

fn main() {
    let config = Config::new(1, 1.0, 1.1, 120.0, 45.0);

    let mut rules = HashMap::new();
    rules.insert(
        Character::Draw,
        vec![
            Character::Draw,
            Character::Right,
            Character::Draw,
            Character::Left,
            Character::Draw,
        ],
    );
    let constants = vec![Character::Left, Character::Right];
    let start = vec![
        Character::Draw,
        Character::Left,
        Character::Draw,
        Character::Left,
        Character::Draw,
    ];
    let system = System::new(rules, constants, start);

    let mut visualizer = Visualizer::new("L-Systems", (640, 480), system, config);
    visualizer.run();
}
