struct ListNode {
    key: i32,
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(key: i32, val: i32) -> Self {
        ListNode {
            key,
            val,
            next: None,
        }
    }
}

struct MyHashMap {
    buckets: Vec<Option<Box<ListNode>>>,
    size: usize,
}

impl MyHashMap {
    pub fn new() -> Self {
        let size = 1000;
        let mut buckets = Vec::with_capacity(size);
        for _ in 0..size {
            buckets.push(None);
        }
        MyHashMap { buckets, size }
    }

    fn hash(&self, key: i32) -> usize {
        (key as usize) % self.size
    }

    pub fn put(&mut self, key: i32, value: i32) {
        let idx = self.hash(key);

        // Check if key already exists
        let mut curr = &mut self.buckets[idx];
        while let Some(node) = curr {
            if node.key == key {
                node.val = value;
                return;
            }
            curr = &mut node.next;
        }

        // Key doesn't exist, add new node at the beginning
        let mut new_node = Box::new(ListNode::new(key, value));
        new_node.next = self.buckets[idx].take();
        self.buckets[idx] = Some(new_node);
    }

    pub fn get(&self, key: i32) -> i32 {
        let idx = self.hash(key);
        let mut curr = &self.buckets[idx];
        while let Some(node) = curr {
            if node.key == key {
                return node.val;
            }
            curr = &node.next;
        }
        -1
    }

    pub fn remove(&mut self, key: i32) {
        let idx = self.hash(key);

        // Handle case where head needs to be removed
        if let Some(ref node) = self.buckets[idx] {
            if node.key == key {
                self.buckets[idx] = self.buckets[idx].take().unwrap().next;
                return;
            }
        }

        // Search through the list
        let mut curr = &mut self.buckets[idx];
        while let Some(node) = curr {
            if let Some(ref next_node) = node.next {
                if next_node.key == key {
                    node.next = node.next.take().unwrap().next;
                    return;
                }
            }
            curr = &mut node.next;
        }
    }
}
