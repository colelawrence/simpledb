mod in_memory;

pub trait SimpleDB {
    fn set(&mut self, key: String, value: u32);
    fn get(&mut self, key: String) -> Option<&u32>;
    fn unset(&mut self, key: String);
    fn begin_transaction(&mut self);
    fn rollback(&mut self) -> Result<(), TransactionError>;
    fn commit(&mut self) -> Result<(), TransactionError>;
}

#[derive(Debug, PartialEq)]
pub enum TransactionError {
    NoTransactionsInProgress,
}

impl std::fmt::Display for TransactionError {
    fn fmt(&self, mut f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionError::NoTransactionsInProgress => {
                write!(&mut f, "No transactions in progress")
            }
        }
    }
}

impl std::error::Error for TransactionError {}
