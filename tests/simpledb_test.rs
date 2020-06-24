use simpledb;

#[test]
fn set_values_are_gettable() {
    let key = s("foo");
    let mut db = simpledb::SimpleDB::new();

    db.set(key.clone(), 10);
    assert_set(&mut db, key.clone(), 10);
}

#[test]
fn unset_values_return_none() {
    let key = s("foo");
    let mut db = simpledb::SimpleDB::new();

    assert_unset(&mut db, key.clone());
}

#[test]
fn values_can_be_unset() {
    let key = s("foo");
    let mut db = simpledb::SimpleDB::new();

    db.set(key.clone(), 10);
    db.unset(key.clone());

    assert_unset(&mut db, key.clone());
}

#[test]
fn rollback_reverts_only_current_transaction() {
    let key = s("foo");

    let mut db = simpledb::SimpleDB::new();

    db.begin_transaction();
    db.set(key.clone(), 10);
    assert_set(&mut db, key.clone(), 10);

    db.begin_transaction();
    db.set(key.clone(), 20);
    assert_set(&mut db, key.clone(), 20);

    db.rollback();
    assert_set(&mut db, key.clone(), 10);

    db.rollback();
    assert_unset(&mut db, key.clone());
}

#[test]
fn commit_commits_all_transactions() {
    let key = s("foo");

    let mut db = simpledb::SimpleDB::new();

    db.begin_transaction();
    db.set(key.clone(), 10);

    db.begin_transaction();
    db.set(key.clone(), 20);

    db.commit();
    assert_set(&mut db, key.clone(), 20);

    assert_tx_error(db.rollback());
}

#[test]
fn commit_errors_if_no_transactions() {
    let mut db = simpledb::SimpleDB::new();
    assert_tx_error(db.commit());
}

fn s(s: &str) -> String {
    String::from(s)
}

fn assert_set(db: &mut simpledb::SimpleDB, key: String, value: u32) {
    assert_eq!(db.get(key), Some(&value));
}

fn assert_unset(db: &mut simpledb::SimpleDB, key: String) {
    assert_eq!(db.get(key.clone()), None);
}

fn assert_tx_error(result: Result<(), String>) {
    assert_eq!(result, Err(String::from("No transactions in progress")))
}
