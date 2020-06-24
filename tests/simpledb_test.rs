use simpledb;

#[test]
fn set_values_are_gettable() {
    let key = s("foo");
    let mut db = simpledb::SimpleDB::new();

    db.set(key.clone(), 10);
    assert_eq!(db.get(key.clone()), Some(&10));
}

#[test]
fn unset_values_return_none() {
    let key = s("foo");
    let mut db = simpledb::SimpleDB::new();

    assert_eq!(db.get(key.clone()), None);
}

#[test]
fn values_can_be_unset() {
    let key = s("foo");
    let mut db = simpledb::SimpleDB::new();

    db.set(key.clone(), 10);
    db.unset(key.clone());

    assert_eq!(db.get(key.clone()), None);
}

#[test]
fn rollback_reverts_only_current_transaction() {
    let key = s("foo");

    let mut db = simpledb::SimpleDB::new();

    db.begin_transaction();
    db.set(key.clone(), 10);
    assert_eq!(db.get(key.clone()), Some(&10));

    db.begin_transaction();
    db.set(key.clone(), 20);
    assert_eq!(db.get(key.clone()), Some(&20));

    db.rollback();
    assert_eq!(db.get(key.clone()), Some(&10));

    db.rollback();
    assert_eq!(db.get(key.clone()), None);
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
    assert_eq!(db.get(key.clone()), Some(&20));

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

fn s(s: &str) -> String {
    String::from(s)
}
