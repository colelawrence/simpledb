use simpledb;

#[test]
fn set_values_are_gettable() {
    let key = String::from("foo");
    let value: u32 = 10;
    let mut db = simpledb::SimpleDB::new();

    db.set(key.clone(), value);
    assert_eq!(db.get(key.clone()), Some(&value));
}

#[test]
fn unset_values_return_none() {
    let key = String::from("foo");
    let mut db = simpledb::SimpleDB::new();

    assert_eq!(db.get(key.clone()), None);
}

#[test]
fn values_can_be_unset() {
    let key = String::from("foo");
    let value: u32 = 10;
    let mut db = simpledb::SimpleDB::new();

    db.set(key.clone(), value);
    db.unset(key.clone());

    assert_eq!(db.get(key.clone()), None);
}

#[test]
fn rollback_reverts_only_current_transaction() {
    let key = String::from("foo");
    let first_value: u32 = 10;
    let second_value: u32 = 20;

    let mut db = simpledb::SimpleDB::new();

    db.begin_transaction();
    db.set(key.clone(), first_value);
    assert_eq!(db.get(key.clone()), Some(&first_value));

    db.begin_transaction();
    db.set(key.clone(), second_value);
    assert_eq!(db.get(key.clone()), Some(&second_value));

    db.rollback();
    assert_eq!(db.get(key.clone()), Some(&first_value));

    db.rollback();
    assert_eq!(db.get(key.clone()), None);
}

#[test]
fn commit_commits_all_transactions() {
    let key = String::from("foo");
    let first_value: u32 = 10;
    let second_value: u32 = 20;

    let mut db = simpledb::SimpleDB::new();

    db.begin_transaction();
    db.set(key.clone(), first_value);

    db.begin_transaction();
    db.set(key.clone(), second_value);

    db.commit();
    assert_eq!(db.get(key.clone()), Some(&second_value));

    assert_eq!(
        db.rollback(),
        Err(String::from("No transactions in progress"))
    )
}

#[test]
fn commit_errors_if_no_transactions() {
    let mut db = simpledb::SimpleDB::new();
    assert_eq!(
        db.commit(),
        Err(String::from("No transactions in progress"))
    )
}
