use bplus_tree::BTree;

#[test]
fn test_new() {
    let mut t = BTree::new();
    t.insert(10, "Test");

    assert_eq!(t.get(10), Some(&"Test"));
}

#[test]
fn test_notfound() {
    let mut t = BTree::new();
    t.insert(10, "Test");

    assert_eq!(t.get(1), None)
}

#[test]
fn test_modification() {
    let mut t = BTree::new();
    t.insert(10, "Test");
    assert_eq!(t.get(1), None);

    t.insert(1, "10");
    assert_eq!(t.get(1), Some(&"10"));

    t.update(1, "11");
    assert_eq!(t.get(1), Some(&"11"));

    assert_eq!(t.update(2, "11"), false);
}
