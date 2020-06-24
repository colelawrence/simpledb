use std::collections::hash_map::HashMap;

pub struct SimpleDB {
    depth: usize,
    transactions: Vec<HashMap<String, u32>>,
}

impl SimpleDB {
    pub fn new() -> Self {
        SimpleDB {
            depth: 0,
            transactions: vec![HashMap::new()],
        }
    }

    pub fn set(&mut self, key: String, value: u32) {
        self.transactions[self.depth].insert(key, value);
    }

    pub fn get(&mut self, key: String) -> Option<&u32> {
        self.transactions[self.depth].get(&key)
    }

    pub fn unset(&mut self, key: String) {
        self.transactions[self.depth].remove(&key);
    }

    pub fn begin_transaction(&mut self) {
        let new_transaction = self.transactions[self.depth].clone();
        self.transactions.push(new_transaction);
        self.depth = self.depth + 1;
    }

    pub fn rollback(&mut self) -> Result<(), String> {
        if self.depth == 0 {
            return Err(String::from("No transactions in progress"));
        }

        self.transactions.pop();
        self.depth = self.depth - 1;
        Ok(())
    }

    pub fn commit(&mut self) -> Result<(), String> {
        if self.depth == 0 {
            return Err(String::from("No transactions in progress"));
        }

        self.transactions = vec![self.transactions.pop().unwrap().clone()];
        self.depth = 0;
        Ok(())
    }
}
