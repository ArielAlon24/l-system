use l_system::graphics::{Config, Visualizer};

use l_system::{state, symbol, Symbol, System};
use std::collections::HashMap;

fn main() {
    let config = Config::new(3, 1.0, 1.1, 22.5, 45.0);

    let mut rules = HashMap::new();

    rules.insert(symbol!('F'), state!("FF"));
    rules.insert(symbol!('X'), state!("F-[[X]+X]+F[+FX]-X"));
    let constants = state!("-+[]");
    let start = state!("X");

    let system = System::new(rules, constants, start);

    let mut visualizer = Visualizer::new("L-Systems", (640, 480), system, config);
    visualizer.run();
}
