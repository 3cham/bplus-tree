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

    // pub fn insert(&mut self, key: u32, value: Any) -> bool {

    // }

    // pub fn delete(key: u32) -> bool {

    // }

    // pub fn update(key: u32, value: Any) -> bool {

    // }

    pub(super) fn get(&self, key: u32) -> Option<&Any> {
        if self.values.is_empty() {
            // find the first child whose key is greater than the search key
            // if not found, default to the first child
            let idx = self.children.iter().position(|child| child.key > key);
            match idx {
                Some(i) if i > 0 => self.children[i - 1].get(key),
                _ => self.children.get(0).and_then(|child| child.get(key)),
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
