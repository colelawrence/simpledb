use simpledb;

#[test]
fn set_values_are_gettable() {
    let mut db = simpledb::SimpleDB::new();

    db.set("foo".to_string(), 10);
    assert_eq!(db.get("foo".to_string()), 10);
}
