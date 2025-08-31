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
