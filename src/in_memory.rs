use std::collections::HashMap;

use super::SimpleDB;
use super::TransactionError;

pub struct InMemoryDB {
    // first in, last out, new transactions applied at front
    transaction_stack: Vec<HashMap<String, Option<u32>>>,
    commited_values: HashMap<String, u32>,
}

impl InMemoryDB {
    pub fn new() -> Self {
        InMemoryDB {
            transaction_stack: Vec::new(),
            commited_values: HashMap::new(),
        }
    }
}

impl SimpleDB for InMemoryDB {
    fn set(&mut self, key: String, value: u32) {
        if let Some(last_tx) = self.transaction_stack.first_mut() {
            last_tx.insert(key, Some(value));
        } else {
            self.commited_values.insert(key, value);
        };
    }

    fn get(&mut self, key: String) -> Option<&u32> {
        if let Some(set_opt) = self
            .transaction_stack
            .iter()
            .find_map(|tx_map|
                tx_map.get(&key).map(|v| v.as_ref()))
        {
            return set_opt;
        } else {
            self.commited_values.get(&key)
        }
    }

    fn unset(&mut self, key: String) {
        if let Some(last_tx) = self.transaction_stack.first_mut() {
            last_tx.insert(key, None);
        } else {
            self.commited_values.remove(&key);
        };
    }

    fn begin_transaction(&mut self) {
        self.transaction_stack.insert(0, HashMap::new())
    }

    fn rollback(&mut self) -> Result<(), TransactionError> {
        if self.transaction_stack.is_empty() {
            Err(TransactionError::NoTransactionsInProgress)
        } else {
            self.transaction_stack.remove(0);
            Ok(())
        }
    }

    fn commit(&mut self) -> Result<(), TransactionError> {
        if self.transaction_stack.is_empty() {
            Err(TransactionError::NoTransactionsInProgress)
        } else {
            let mut transactions_to_commit: Vec<HashMap<_, _>> =
                self.transaction_stack.drain(..).collect();
            transactions_to_commit.reverse(); // apply with end first
            for hash_map in transactions_to_commit {
                for (key, value_opt) in hash_map {
                    match value_opt {
                        Some(value) => self.commited_values.insert(key, value),
                        None => self.commited_values.remove(&key),
                    };
                }
            }
            Ok(())
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

    #[test]
    fn get_value_from_before_commit() {
        let mut db = InMemoryDB::new();
        db.begin_transaction();
        db.set(s("foo"), 10);
        db.begin_transaction();
        db.set(s("bar"), 10);
        assert_no_tx_error(db.commit());
        assert_set!(&mut db, s("foo"), 10);
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
