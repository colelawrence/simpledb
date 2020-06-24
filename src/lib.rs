pub trait SimpleDB {
    fn set(&mut self, key: String, value: u32);
    fn get(&mut self, key: String) -> Option<&u32>;
    fn unset(&mut self, key: String);
    fn begin_transaction(&mut self);
    fn rollback(&mut self) -> Result<(), String>;
    fn commit(&mut self) -> Result<(), String>;
}

pub struct InMemoryDB {}

impl InMemoryDB {
    pub fn new() -> Self {
        InMemoryDB {}
    }
}

impl SimpleDB for InMemoryDB {
    fn set(&mut self, key: String, value: u32) {}

    fn get(&mut self, key: String) -> Option<&u32> {
        None
    }

    fn unset(&mut self, key: String) {}

    fn begin_transaction(&mut self) {}

    fn rollback(&mut self) -> Result<(), String> {
        Err(String::from("not implemented"))
    }

    fn commit(&mut self) -> Result<(), String> {
        Err(String::from("not implemented"))
    }
}
