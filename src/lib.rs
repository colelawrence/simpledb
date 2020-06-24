use std::collections::hash_map::HashMap;

pub trait SimpleDB {
    fn set(&mut self, key: String, value: u32);
    fn get(&mut self, key: String) -> Option<&u32>;
    fn unset(&mut self, key: String);
    fn begin_transaction(&mut self);
    fn rollback(&mut self) -> Result<(), String>;
    fn commit(&mut self) -> Result<(), String>;
}

pub struct InMemoryDB {
    depth: usize,
    transactions: Vec<HashMap<String, u32>>,
}

impl InMemoryDB {
    pub fn new() -> Self {
        InMemoryDB {
            depth: 0,
            transactions: vec![HashMap::new()],
        }
    }
}

impl SimpleDB for InMemoryDB {
    fn set(&mut self, key: String, value: u32) {
        self.transactions[self.depth].insert(key, value);
    }

    fn get(&mut self, key: String) -> Option<&u32> {
        self.transactions[self.depth].get(&key)
    }

    fn unset(&mut self, key: String) {
        self.transactions[self.depth].remove(&key);
    }

    fn begin_transaction(&mut self) {
        let new_transaction = self.transactions[self.depth].clone();
        self.transactions.push(new_transaction);
        self.depth = self.depth + 1;
    }

    fn rollback(&mut self) -> Result<(), String> {
        if self.depth == 0 {
            return Err(String::from("No transactions in progress"));
        }

        self.transactions.pop();
        self.depth = self.depth - 1;
        Ok(())
    }

    fn commit(&mut self) -> Result<(), String> {
        if self.depth == 0 {
            return Err(String::from("No transactions in progress"));
        }

        self.transactions = vec![self.transactions.pop().unwrap().clone()];
        self.depth = 0;
        Ok(())
    }
}
