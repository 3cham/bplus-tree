mod tree;

use crate::tree::Node;

pub struct BTree<Any> {
    root: Node<Any>
}

impl<Any> BTree<Any> {
    pub fn new() -> Self {
        BTree { root: Node::new() }
    }

    pub fn insert(&mut self, key: u32, value: Any) -> bool {
        self.root.insert(key, value)
    }

    pub fn get(&self, key: u32) -> Option<&Any> {
        self.root.find(key)
    }
}
