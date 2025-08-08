pub struct Node<Any> {
    key: u32,
    childrens: Vec<Node<Any>>,
    values: Vec<Kv<Any>>
}

pub struct Kv<Any> {
    key: u32,
    value: Any
}

impl<Any> Node<Any> {
    pub fn new(key: u32, value: Any) -> Self {
        let mut values = Vec::new();
        values.push(Kv {
            key: key,
            value: value
        });

        Node { key: key, childrens: Vec::new(), values: values }
    }

    // pub fn insert(&mut self, key: u32, value: Any) -> bool {

    // }

    // pub fn delete(key: u32) -> bool {

    // }

    // pub fn update(key: u32, value: Any) -> bool {

    // }

    pub fn get(&self, key: u32) -> Option<&Any> {
        let node = &self;
        if node.values.is_empty() {
            for i in 1..node.childrens.len() {
                if node.childrens[i].key > key {
                    return node.childrens[i - 1].get(key)
                }
            }
            return node.childrens.get(0).and_then(|child| child.get(key));
        } else {
            for i in 0..node.values.len() {
                if node.values[i].key == key {
                    let kv = &node.values[i];
                    return Some(&kv.value)
                }
            }
            return None
        }
    }

    // fn rebalance(&mut self) -> bool {

    // }
}
