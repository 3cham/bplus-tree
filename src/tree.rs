use std::fmt::Display;

// denotes the maximum number of children that any node in the tree **can** have
const TREE_ORDER: u32 = 8;

#[derive(Clone)]
pub(super) struct Node<Any: Clone> {
    is_root: bool,
    keys: Vec<u32>,
    children: Vec<Node<Any>>,
    values: Vec<Kv<Any>>,
}

#[derive(Clone)]
struct Kv<Any: Clone> {
    key: u32,
    value: Any,
}

impl<Any: Clone> Display for Kv<Any> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.key)
    }
}

impl<Any: Clone> Node<Any> {
    pub(super) fn new() -> Self {
        Node {
            keys: Vec::new(),
            children: Vec::new(),
            values: Vec::new(),
            is_root: true,
        }
    }

    pub(super) fn insert(&mut self, key: u32, value: Any) -> Option<Node<Any>> {
        if self.is_leaf_node() && self.is_root {
            // if our tree is still empty
            self.keys.push(key);
            self.children.push(Node {
                keys: [key].to_vec(),
                children: Vec::new(),
                values: [Kv{key: key, value: value}].to_vec(),
                is_root: false,
            });
            return None;
        }

        // perform insert
        if self.is_leaf_node() {
            self.insert_key_value_at_leaf(key, value);
        } else {
            let child_index = self.find_child_index(key);
            if let Some(new_node) = self.children[child_index].insert(key, value) {
                self.insert_child_node(new_node);
            }
        };

        // perform node splitting if needed:
        if self.keys.len() > TREE_ORDER as usize {
            self.split_node()
        } else {
            None
        }
    }

    // pub fn delete(key: u32) -> bool {

    // }

    pub fn update(&mut self, key: u32, value: Any) -> bool {
        if !self.is_leaf_node() {
            let idx = self.find_child_index(key);

            if let Some(child) = self.children.get_mut(idx) {
                child.update(key, value)
            } else {
                false
            }
        } else {
            match self.values.iter().position(|elem| elem.key == key) {
                Some(idx) => {
                    if let Some(elem) = self.values.get_mut(idx) {
                        elem.value = value;
                        true
                    } else {
                        false
                    }
                }
                _ => false,
            }
        }
    }

    // pub fn find_range(min_key: u32, max_key: u32) -> Vec<Any> {
    //
    // }

    pub(super) fn find(&self, key: u32) -> Option<&Any> {
        if !self.is_leaf_node() {
            let child_index = self.find_child_index(key);
            self.children[child_index].find(key)
        } else {
            self.values
                .iter()
                .find(|kv| kv.key == key)
                .map(|kv| &kv.value)
        }
    }

    fn copy(&self) -> Self {
        Node {
            keys: self.keys.to_vec(),
            children: self.children.to_vec(),
            values: self.values.to_vec(),
            is_root: false,
        }
    }

    fn is_leaf_node(&self) -> bool {
        self.children.is_empty()
    }

    fn insert_key_value_at_leaf(&mut self, key: u32, value: Any) {
        match self.keys.iter().position(|elem| *elem > key) {
            Some(i) => {
                self.values.insert(i, Kv { key, value });
                self.keys.insert(i, key);
            }
            None => {
                self.values.push(Kv { key, value });
                self.keys.push(key);
            }
        }
    }

    fn insert_child_node(&mut self, node: Node<Any>) {
        let new_key = node.keys[0];
        match self.keys.iter().position(|elem| *elem > new_key) {
            Some(i) => {
                self.keys.insert(i, new_key);
                self.children.insert(i, node);
            }
            None => {
                self.keys.push(new_key);
                self.children.push(node);
            }
        }
    }

    fn find_child_index(&self, key: u32) -> usize {
        // find the first child whose key is greater than the search key
        // if not found, default to the last child
        match self.keys.iter().position(|ckey| *ckey > key) {
            Some(i) if i > 0 => i - 1,
            _ => self.keys.len() - 1,
        }
    }

    fn split_node(&mut self) -> Option<Node<Any>> {
        let mid = self.keys.len() / 2;

        let new_right_node = Node {
            keys: self.keys[mid..].to_vec(),
            values: if self.is_leaf_node() { self.values[mid..].to_vec() } else { Vec::new() },
            children: if self.is_leaf_node() { Vec::new() } else { self.children[mid..].to_vec() },
            is_root: false,
        };

        self.keys.truncate(mid);

        if self.is_leaf_node() {
            self.values.truncate(mid);
        } else {
            self.children.truncate(mid);
        }

        if self.is_root {
            let new_left_node = self.copy();
            self.keys = [new_left_node.keys[0], new_right_node.keys[0]].to_vec();
            self.children = [new_left_node, new_right_node].to_vec();
            return None;
        } else {
            return Some(new_right_node);
        }
    }

    fn _has_overflown_leaf(&self) -> bool {
        if self.is_leaf_node() {
            self.values.len() > TREE_ORDER as usize
        } else {
            self.children.iter().any(|child| child._has_overflown_leaf())
        }
    }

    fn _has_non_sorted_leaf(&self) -> bool {
        if self.is_leaf_node() {
            !self.values.is_sorted_by(|a, b| a.key <= b.key)
        } else {
            self.children.iter().any(|child| child._has_non_sorted_leaf())
        }
    }

    fn _display(&self) {
        if self.is_leaf_node() {
            self.values.iter().for_each(|kv| print!("{kv} "));
            print!("|")
        } else {
            self.children.iter().for_each(|node| node._display());
            println!();
            self.keys.iter().for_each(|key| print!("{key} "));
            print!("|")
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_node_new() {
        let mut n = super::Node::new();
        n.insert(1, 2);
        assert_eq!(n.keys.first(), Some(&1));
        assert_eq!(n.children.len(), 1);
        assert_eq!(n.values.len(), 0);
        assert_eq!(n.children[0].values.get(0).map(|elem| elem.value), Some(2));
        assert_eq!(n.children[0].values.get(0).map(|elem| elem.key), Some(1));
    }

    #[test]
    fn test_node_find() {
        let mut n = super::Node::new();
        n.insert(1, 2);
        assert_eq!(n.find(1), Some(&2))
    }

    #[test]
    fn test_node_update() {
        let mut n = super::Node::new();
        n.insert(1, 2);
        assert_eq!(n.find(1), Some(&2));
        n.update(1, 5);
        assert_eq!(n.find(1), Some(&5));
    }

    #[test]
    fn test_node_insert() {
        let mut n = super::Node::new();
        for i in 1..20 {
            _ = n.insert(i, i);
            assert_eq!(n.find(i), Some(&i));
            assert_eq!(n._has_non_sorted_leaf(), false);
            assert_eq!(n._has_overflown_leaf(), false)
        }
    }

    #[test]
    fn test_invalid_update() {
        let mut n = super::Node::new();
        for i in 1..20 {
            _ = n.insert(i, i);
        }

        let updated = n.update(21, 0);
        assert_eq!(updated, false);
    }
}
