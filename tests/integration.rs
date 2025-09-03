use std::collections::BTreeMap;
use bplus_tree::BTree;
use rand::random;

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

#[test]
fn test_compare_with_btreemap() {
    let mut t = BTree::<u32>::new();
    let mut bm = BTreeMap::<u32, u32>::new();

    for _ in [0..10e6 as i32] {
        let (k, v) = (random::<u32>(), random::<u32>());
        t.insert(k, v);
        bm.insert(k, v);
    }

    for key in bm.keys() {
        assert_eq!(bm.get(key), t.get(*key));
    }
}
