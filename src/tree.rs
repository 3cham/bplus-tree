pub(super) struct Node<Any> {
    key: u32,
    children: Vec<Node<Any>>,
    values: Vec<Kv<Any>>
}

struct Kv<Any> {
    key: u32,
    value: Any
}

impl<Any> Node<Any> {
    pub(super) fn new(key: u32, value: Any) -> Self {
        let mut values = Vec::new();
        values.push(Kv {
            key: key,
            value: value
        });

        Node { key: key, children: Vec::new(), values: values }
    }

    fn is_leaf_node(&self) -> bool {
        !self.values.is_empty()
    }

    pub fn insert(&mut self, key: u32, value: Any, order: u32) -> bool {
        if self.is_leaf_node() {
            self.values.push(Kv{key, value});
            return true;
        } else {
            let idx = self.children.iter().position(|child| child.key > key);
            match idx {
                Some(i) if i > 0 => self.children[i - 1].insert(key, value, order),
                _ => self.children[0].insert(key, value, order),
            }
        }
    }

    // pub fn delete(key: u32) -> bool {

    // }

    // pub fn update(key: u32, value: Any) -> bool {

    // }

    pub(super) fn find(&self, key: u32) -> Option<&Any> {
        if !self.is_leaf_node() {
            // find the first child whose key is greater than the search key
            // if not found, default to the first child
            let idx = self.children.iter().position(|child| child.key > key);
            match idx {
                Some(i) if i > 0 => self.children[i - 1].find(key),
                _ => self.children.get(0).and_then(|child| child.find(key)),
            }
        } else {
            self.values.iter()
                .find(|kv| kv.key == key)
                .map(|kv| &kv.value)
        }
    }

    // fn rebalance(&mut self) -> bool {

    // }
}

#[test]
fn test_node_new() {
    let n = Node::new(1, 2);
    assert_eq!(n.key, 1);
    assert_eq!(n.children.len(), 0);
    assert_eq!(n.values.len(), 1);
    assert_eq!(n.values.get(0).map(|elem| elem.value), Some(2));
}

#[test]
fn test_node_get() {
    let n = Node::new(1, 2);
    assert_eq!(n.find(1), Some(&2))
}

#[test]
fn test_node_insert() {
    let mut n = Node::new(1, 2);
    let order = 3;
    for i in 2..10 {
        _ = n.insert(i, i, order);
        assert_eq!(n.find(i), Some(&i))
    }
}
