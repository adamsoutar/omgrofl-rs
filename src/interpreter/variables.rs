use std::collections::HashMap;

pub struct Variables {
    vars: HashMap<usize, u8>,
    // Omgrofl has one collection that acts as a Stack and a Queue
    staque: Vec<u8>
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

    pub fn inc (&mut self, key: usize) {
        let val = self.get(key);
        self.set(key, val + 1);
    }

    pub fn dec (&mut self, key: usize) {
        let val = self.get(key);
        self.set(key, val - 1);
    }

    pub fn staque_push (&mut self, val: u8) {
        self.staque.push(val);
    }

    pub fn staque_pop_to_var (&mut self, key: usize) {
        let val = self.staque.pop().unwrap_or(0);
        self.set(key, val);
    }

    pub fn staque_dequeue_to_var (&mut self, key: usize) {
        let val =
            if self.staque.len() < 1 { 0 }
            else { self.staque.remove(0) };
        self.set(key, val);
    }

    pub fn print (&self) {
        for (key, val) in &self.vars {
            println!("Var #{} - {}", key, val);
        }

        for i in 0..self.staque.len() {
            let val = self.staque[i];
            println!("Staque #{} - {}", i, val)
        }
    }

    pub fn new () -> Variables {
        Variables {
            vars: HashMap::new(),
            staque: vec![]
        }
    }
}
