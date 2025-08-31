pub(super) struct Node<Any> {
    keys: Vec<u32>,
    children: Vec<Node<Any>>,
    values: Vec<Kv<Any>>
}

struct Kv<Any> {
    key: u32,
    value: Any
}

// denotes the maximum number of children that any node in the tree **can** have
const TREE_ORDER: u32 = 4;
// also, TREE_ORDER / 2 is the minimum number of children that any node in the tree **should** have
const MIN_CHILDREN_COUNT: u32 = TREE_ORDER / 2;

impl<Any> Node<Any> {
    pub(super) fn new() -> Self {
        Node { keys: Vec::new(), children: Vec::new(), values: Vec::new() }
    }

    fn is_leaf_node(&self) -> bool {
        self.children.is_empty()
    }

    pub(super) fn insert(&mut self, key: u32, value: Any) -> bool {
        if self.is_leaf_node() {
            // if self.values.len() < TREE_ORDER as usize {
                let idx = self.keys.iter().position(|elem| *elem > key);
                match idx {
                    Some(i) => {
                        self.values.insert(i, Kv{key, value});
                        self.keys.insert(i, key);
                    },
                    _ => {
                        self.values.push(Kv{key, value});
                        self.keys.push(key);
                    },
                }
            // } else {

            // }
            return true;
        } else {
            let idx = self.keys.iter().position(|ckey| *ckey > key);
            match idx {
                Some(i) if i > 0 => self.children[i - 1].insert(key, value),
                _ => self.children[0].insert(key, value),
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
            let idx = self.keys.iter().position(|ckey| *ckey > key);
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

    fn has_overflown_leaf(&self) -> bool {
        if self.is_leaf_node() {
            self.values.len() > TREE_ORDER as usize
        } else {
            self.children.iter().any(|child| child.has_overflown_leaf())
        }
    }

    fn has_non_sorted_leaf(&self) -> bool {
        if self.is_leaf_node() {
            !self.values.is_sorted_by(|a, b| a.key <= b.key)
        } else {
            self.children.iter().any(|child| child.has_non_sorted_leaf())
        }
    }
}

mod test {
    use super::Node;

    #[test]
    fn test_node_new() {
        let mut n = Node::new();
        n.insert(1, 2);
        assert_eq!(n.keys.first(), Some(&1));
        assert_eq!(n.children.len(), 0);
        assert_eq!(n.values.len(), 1);
        assert_eq!(n.values.get(0).map(|elem| elem.value), Some(2));
        assert_eq!(n.values.get(0).map(|elem| elem.key), Some(1));
    }

    #[test]
    fn test_node_find() {
        let mut n = Node::new();
        n.insert(1, 2);
        assert_eq!(n.find(1), Some(&2))
    }

    #[test]
    fn test_node_insert() {
        let mut n = Node::new();
        for i in 1..10 {
            _ = n.insert(i, i);
            assert_eq!(n.find(i), Some(&i));
            assert_eq!(n.has_non_sorted_leaf(), false);
            // assert_eq!(n.has_overflown_leaf(order), false)
        }
    }
}
