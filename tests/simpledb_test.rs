use simpledb;

#[test]
fn set_values_are_gettable() {
    let mut db = simpledb::SimpleDB::new();

    db.set(s("foo"), 10);
    assert_set(&mut db, s("foo"), 10);
}

#[test]
fn unset_values_return_none() {
    let mut db = simpledb::SimpleDB::new();

    assert_unset(&mut db, s("foo"));
}

#[test]
fn values_can_be_unset() {
    let mut db = simpledb::SimpleDB::new();

    db.set(s("foo"), 10);
    db.unset(s("foo"));

    assert_unset(&mut db, s("foo"));
}

#[test]
fn rollback_reverts_only_current_transaction() {
    let mut db = simpledb::SimpleDB::new();

    db.begin_transaction();
    db.set(s("foo"), 10);
    assert_set(&mut db, s("foo"), 10);

    db.begin_transaction();
    db.set(s("foo"), 20);
    assert_set(&mut db, s("foo"), 20);

    db.rollback();
    assert_set(&mut db, s("foo"), 10);

    db.rollback();
    assert_unset(&mut db, s("foo"));
}

#[test]
fn commit_commits_all_transactions() {
    let mut db = simpledb::SimpleDB::new();

    db.begin_transaction();
    db.set(s("foo"), 10);

    db.begin_transaction();
    db.set(s("foo"), 20);

    db.commit();
    assert_set(&mut db, s("foo"), 20);

    assert_tx_error(db.rollback());
}

#[test]
fn commit_errors_if_no_transactions() {
    let mut db = simpledb::SimpleDB::new();
    assert_tx_error(db.commit());
}

#[test]
fn interleave_keys() {
    let mut db = simpledb::SimpleDB::new();

    db.set(s("foo"), 10);
    db.set(s("bar"), 10);
    assert_set(&mut db, s("foo"), 10);
    assert_set(&mut db, s("bar"), 10);

    db.begin_transaction();
    db.set(s("foo"), 20);
    assert_set(&mut db, s("foo"), 20);
    assert_set(&mut db, s("bar"), 10);

    db.begin_transaction();
    db.set(s("bar"), 30);
    assert_set(&mut db, s("foo"), 20);
    assert_set(&mut db, s("bar"), 30);

    assert_no_tx_error(db.rollback());
    assert_set(&mut db, s("foo"), 20);
    assert_set(&mut db, s("bar"), 10);

    assert_no_tx_error(db.rollback());
    assert_set(&mut db, s("foo"), 10);
    assert_set(&mut db, s("bar"), 10);
}

#[test]
fn rollback_unset() {
    let mut db = simpledb::SimpleDB::new();

    db.set(s("foo"), 10);
    assert_set(&mut db, s("foo"), 10);

    db.begin_transaction();
    assert_set(&mut db, s("foo"), 10);

    db.set(s("foo"), 20);
    assert_set(&mut db, s("foo"), 20);

    db.begin_transaction();
    db.unset(s("foo"));
    assert_unset(&mut db, s("foo"));

    assert_no_tx_error(db.rollback());
    assert_set(&mut db, s("foo"), 20);

    assert_no_tx_error(db.commit());
    assert_set(&mut db, s("foo"), 20);
}

#[test]
fn commit_unset() {
    let mut db = simpledb::SimpleDB::new();

    db.set(s("foo"), 10);
    assert_set(&mut db, s("foo"), 10);

    db.begin_transaction();
    db.unset(s("foo"));
    assert_unset(&mut db, s("foo"));

    assert_no_tx_error(db.commit());
    assert_unset(&mut db, s("foo"));

    db.begin_transaction();
    assert_unset(&mut db, s("foo"));

    db.set(s("foo"), 20);
    assert_set(&mut db, s("foo"), 20);

    assert_no_tx_error(db.commit());
    assert_set(&mut db, s("foo"), 20);
}

fn s(s: &str) -> String {
    String::from(s)
}

fn assert_set(db: &mut simpledb::SimpleDB, key: String, value: u32) {
    assert_eq!(db.get(key), Some(&value));
}

fn assert_unset(db: &mut simpledb::SimpleDB, key: String) {
    assert_eq!(db.get(key), None);
}

fn assert_tx_error(result: Result<(), String>) {
    assert_eq!(result, Err(String::from("No transactions in progress")));
}

fn assert_no_tx_error(result: Result<(), String>) {
    assert_eq!(result, Ok(()));
}
