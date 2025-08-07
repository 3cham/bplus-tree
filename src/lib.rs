mod tree;

use crate::tree::Node;

pub struct BTree<Any> {
    order: u32,
    root: Node<Any>
}

impl<Any> BTree<Any> {
    pub fn new(order: u32, key: u32, value: Any) -> Self {
        BTree { order: order, root: Node::new(key, value) }
    }

    pub fn get(&self, key: u32) -> Option<&Any> {
        self.root.get(key)
    }
}
