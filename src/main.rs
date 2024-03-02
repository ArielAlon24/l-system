use l_system::graphics::{Config, Visualizer};
use l_system::{Character, System};
use std::collections::HashMap;

fn main() {
    let config = Config::new(1, 1.1, 90.0, 10.0);
    let mut window = Visualizer::new("L-System - Dragon Curve", (640, 480), config);

    let mut rules = HashMap::new();
    rules.insert(
        Character::Draw,
        vec![
            Character::Draw,
            Character::Right,
            Character::Draw,
            Character::Left,
        ],
    );
    let constants = vec![Character::Left, Character::Right];
    let start = vec![Character::Draw, Character::Right];
    let system = System::new(rules, constants, start);

    window.run(system);
}
