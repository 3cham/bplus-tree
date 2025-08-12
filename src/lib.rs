mod tree;

use crate::tree::Node;

pub struct BTree<Any> {
    order: u32,
    root: Node<Any>
}

impl<Any> BTree<Any> {
    pub fn new(order: u32) -> Self {
        BTree { order: order, root: Node::new() }
    }

    pub fn insert(&mut self, key: u32, value: Any) -> bool {
        self.root.insert(key, value, self.order)
    }

    pub fn get(&self, key: u32) -> Option<&Any> {
        self.root.find(key)
    }
}
