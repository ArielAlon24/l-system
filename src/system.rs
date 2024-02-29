use std::collections::HashMap;
use std::mem;

#[derive(Debug)]
pub struct System {
    rules: HashMap<char, &'static str>,
    start: String,
}

impl<'a> System {
    pub fn new(rules: HashMap<char, &'static str>, start: String) -> Self {
        Self { rules, start }
    }
}

impl IntoIterator for System {
    type Item = String;

    type IntoIter = SystemIterator;

    fn into_iter(self) -> Self::IntoIter {
        SystemIterator::new(self.start, self.rules)
    }
}

pub struct SystemIterator {
    state: String,
    rules: HashMap<char, &'static str>,
}

impl<'a> SystemIterator {
    fn new(state: String, rules: HashMap<char, &'static str>) -> Self {
        Self { state, rules }
    }
}

impl Iterator for SystemIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next = String::new();
        for c in self.state.chars() {
            match self.rules.get(&c) {
                Some(replacement) => next.push_str(&replacement),
                None => next.push(c),
            }
        }

        let previous = mem::replace(&mut self.state, next);
        Some(previous)
    }
}
