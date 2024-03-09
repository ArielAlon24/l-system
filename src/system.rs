use crate::Symbol;
use std::collections::HashMap;
use std::mem;

pub type State = Vec<Symbol>;
pub type Rules = HashMap<Symbol, State>;

pub fn dump(state: &State) -> String {
    let mut string = String::new();
    for symbol in state {
        string.push_str(&format!("{}", symbol));
    }
    string
}

#[derive(Debug, Clone)]
pub struct System {
    rules: Rules,
    start: State,
}

impl System {
    pub fn new(mut rules: Rules, constants: Vec<Symbol>, start: State) -> Self {
        for constant in constants.into_iter() {
            rules
                .entry(constant.clone())
                .or_insert_with(|| vec![constant]);
        }

        Self {
            rules,
            start: start.to_vec(),
        }
    }
}

impl IntoIterator for System {
    type Item = State;

    type IntoIter = SystemIterator;

    fn into_iter(self) -> Self::IntoIter {
        SystemIterator::new(self.start, self.rules)
    }
}

pub struct SystemIterator {
    state: State,
    rules: Rules,
    buffer: State,
}

impl SystemIterator {
    fn new(state: State, rules: Rules) -> Self {
        Self {
            state: state.clone(),
            rules,
            buffer: state,
        }
    }

    pub fn state(&self) -> &State {
        &self.state
    }
}

impl Iterator for SystemIterator {
    type Item = State;

    fn next(&mut self) -> Option<Self::Item> {
        self.buffer.clear();
        for character in &self.state {
            match self.rules.get(&character) {
                Some(replacement) => self.buffer.append(&mut replacement.clone()),
                None => self.buffer.push(character.clone()),
            }
        }
        mem::swap(&mut self.state, &mut self.buffer);
        Some(mem::take(&mut self.buffer))
    }
}
