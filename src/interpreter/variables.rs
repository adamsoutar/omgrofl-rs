use std::collections::HashMap;

pub struct Variables {
    vars: HashMap<usize, u8>
}

impl Variables {
    pub fn set (&mut self, key: usize, val: u8) {
        self.vars.insert(key, val);
    }

    pub fn get (&self, key: usize) -> u8 {
        if self.vars.contains_key(&key) {
            self.vars[&key]
        } else { 0 }
    }

    pub fn print (&self) {
        for (key, val) in &self.vars {
            println!("Var #{} - {}", key, val);
        }
    }

    pub fn new () -> Variables {
        Variables {
            vars: HashMap::new()
        }
    }
}
