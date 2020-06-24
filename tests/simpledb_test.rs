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
