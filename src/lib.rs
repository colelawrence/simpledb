use std::collections::hash_map::HashMap;

pub struct SimpleDB {
    values: HashMap<String, u32>,
}

impl SimpleDB {
    pub fn new() -> Self {
        SimpleDB {
            values: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: u32) {
        self.values.insert(key, value);
    }

    pub fn get(&mut self, key: String) -> Option<&u32> {
        self.values.get(&key)
    }
}
