mod tree;

use crate::tree::Node;

pub struct BTree<Any: Clone> {
    root: Node<Any>
}

impl<Any: Clone> BTree<Any> {
    pub fn new() -> Self {
        BTree { root: Node::new() }
    }

    pub fn insert(&mut self, key: u32, value: Any) -> bool {
        _ = self.root.insert(key, value);
        return true;
    }

    pub fn get(&self, key: u32) -> Option<&Any> {
        self.root.find(key)
    }
}
