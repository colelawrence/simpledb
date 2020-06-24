use std::collections::HashMap;

use super::SimpleDB;
use super::TransactionError;

pub struct InMemoryDB {
    transaction_stack: Vec<HashMap<String, u32>>,
    commit: HashMap<String, u32>,
}

impl InMemoryDB {
    pub fn new() -> Self {
        InMemoryDB {
            transaction_stack: Vec::new(),
            commit: HashMap::new(),
        }
    }
}

impl SimpleDB for InMemoryDB {
    fn set(&mut self, key: String, value: u32) {}

    fn get(&mut self, key: String) -> Option<&u32> {
        None
    }

    fn unset(&mut self, key: String) {}

    fn begin_transaction(&mut self) {}

    fn rollback(&mut self) -> Result<(), TransactionError> {
        Err(TransactionError::NoTransactionsInProgress)
    }

    fn commit(&mut self) -> Result<(), TransactionError> {
        match self.transaction_stack.pop() {
            Some(transaction) => Ok(self.commit.extend(transaction.into_iter())),
            None => Err(TransactionError::NoTransactionsInProgress),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SimpleDB;
    use crate::TransactionError;

    #[macro_use]
    macro_rules! assert_set {
        ($db:expr, $key:expr, $value:expr) => {{
            assert_eq!($db.get($key), Some(&$value));
        }};
    }

    #[test]
    fn set_values_are_gettable() {
        let mut db = InMemoryDB::new();

        db.set(s("foo"), 10);
        assert_set!(&mut db, s("foo"), 10);
    }

    #[test]
    fn unset_values_return_none() {
        let mut db = InMemoryDB::new();

        assert_unset(&mut db, s("foo"));
    }

    #[test]
    fn values_can_be_unset() {
        let mut db = InMemoryDB::new();

        db.set(s("foo"), 10);
        assert_set!(&mut db, s("foo"), 10);

        db.unset(s("foo"));
        assert_unset(&mut db, s("foo"));
    }

    #[test]
    fn rollback_reverts_only_current_transaction() {
        let mut db = InMemoryDB::new();

        db.begin_transaction();
        db.set(s("foo"), 10);
        assert_set!(&mut db, s("foo"), 10);

        db.begin_transaction();
        db.set(s("foo"), 20);
        assert_set!(&mut db, s("foo"), 20);

        db.rollback();
        assert_set!(&mut db, s("foo"), 10);

        db.rollback();
        assert_unset(&mut db, s("foo"));
    }

    #[test]
    fn commit_commits_all_transactions() {
        let mut db = InMemoryDB::new();

        db.begin_transaction();
        db.set(s("foo"), 10);

        db.begin_transaction();
        db.set(s("foo"), 20);

        db.commit().unwrap();
        assert_set!(&mut db, s("foo"), 20);

        assert_tx_error(db.rollback());
    }

    #[test]
    fn commit_errors_if_no_transactions() {
        let mut db = InMemoryDB::new();
        assert_tx_error(db.commit());
    }

    #[test]
    fn interleave_keys() {
        let mut db = InMemoryDB::new();

        db.set(s("foo"), 10);
        db.set(s("bar"), 10);
        assert_set!(&mut db, s("foo"), 10);
        assert_set!(&mut db, s("bar"), 10);

        db.begin_transaction();
        db.set(s("foo"), 20);
        assert_set!(&mut db, s("foo"), 20);
        assert_set!(&mut db, s("bar"), 10);

        db.begin_transaction();
        db.set(s("bar"), 30);
        assert_set!(&mut db, s("foo"), 20);
        assert_set!(&mut db, s("bar"), 30);

        assert_no_tx_error(db.rollback());
        assert_set!(&mut db, s("foo"), 20);
        assert_set!(&mut db, s("bar"), 10);

        assert_no_tx_error(db.rollback());
        assert_set!(&mut db, s("foo"), 10);
        assert_set!(&mut db, s("bar"), 10);
    }

    #[test]
    fn rollback_unset() {
        let mut db = InMemoryDB::new();

        db.set(s("foo"), 10);
        assert_set!(&mut db, s("foo"), 10);

        db.begin_transaction();
        assert_set!(&mut db, s("foo"), 10);

        db.set(s("foo"), 20);
        assert_set!(&mut db, s("foo"), 20);

        db.begin_transaction();
        db.unset(s("foo"));
        assert_unset(&mut db, s("foo"));

        assert_no_tx_error(db.rollback());
        assert_set!(&mut db, s("foo"), 20);

        assert_no_tx_error(db.commit());
        assert_set!(&mut db, s("foo"), 20);
    }

    #[test]
    fn commit_unset() {
        let mut db = InMemoryDB::new();

        db.set(s("foo"), 10);
        assert_set!(&mut db, s("foo"), 10);

        db.begin_transaction();
        db.unset(s("foo"));
        assert_unset(&mut db, s("foo"));

        assert_no_tx_error(db.commit());
        assert_unset(&mut db, s("foo"));

        db.begin_transaction();
        assert_unset(&mut db, s("foo"));

        db.set(s("foo"), 20);
        assert_set!(&mut db, s("foo"), 20);

        assert_no_tx_error(db.commit());
        assert_set!(&mut db, s("foo"), 20);
    }

    fn s(s: &str) -> String {
        String::from(s)
    }

    // fn assert_set(db: &mut InMemoryDB, key: String, value: u32) {}

    fn assert_unset(db: &mut InMemoryDB, key: String) {
        assert_eq!(db.get(key), None);
    }

    fn assert_tx_error(result: Result<(), TransactionError>) {
        assert_eq!(
            result.expect_err("expecting transaction error"),
            TransactionError::NoTransactionsInProgress
        );
    }

    fn assert_no_tx_error(result: Result<(), TransactionError>) {
        result.expect("expecting no transaction error");
    }
}
