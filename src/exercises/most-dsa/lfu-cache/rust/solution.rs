use std::collections::HashMap;

struct LFUCache {
    capacity: usize,
    min_freq: i32,
    key_to_val: HashMap<i32, i32>,
    key_to_freq: HashMap<i32, i32>,
    freq_to_keys: HashMap<i32, Vec<i32>>,
    key_to_pos: HashMap<i32, usize>,
}

impl LFUCache {
    pub fn new(capacity: i32) -> Self {
        LFUCache {
            capacity: capacity as usize,
            min_freq: 0,
            key_to_val: HashMap::new(),
            key_to_freq: HashMap::new(),
            freq_to_keys: HashMap::new(),
            key_to_pos: HashMap::new(),
        }
    }

    fn update_freq(&mut self, key: i32) {
        let freq = *self.key_to_freq.get(&key).unwrap();
        self.key_to_freq.insert(key, freq + 1);

        // Remove from current frequency list
        let keys = self.freq_to_keys.get_mut(&freq).unwrap();
        let pos = *self.key_to_pos.get(&key).unwrap();
        keys.remove(pos);

        // Update positions for keys after the removed one
        for i in pos..keys.len() {
            self.key_to_pos.insert(keys[i], i);
        }

        // If this was the min frequency list and it's now empty, increment min_freq
        if freq == self.min_freq && keys.is_empty() {
            self.min_freq += 1;
        }

        // Add to new frequency list
        let new_freq = freq + 1;
        self.freq_to_keys.entry(new_freq).or_insert_with(Vec::new);
        let new_keys = self.freq_to_keys.get_mut(&new_freq).unwrap();
        self.key_to_pos.insert(key, new_keys.len());
        new_keys.push(key);
    }

    pub fn get(&mut self, key: i32) -> i32 {
        if !self.key_to_val.contains_key(&key) {
            return -1;
        }
        self.update_freq(key);
        *self.key_to_val.get(&key).unwrap()
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if self.capacity == 0 {
            return;
        }

        if self.key_to_val.contains_key(&key) {
            self.key_to_val.insert(key, value);
            self.update_freq(key);
            return;
        }

        // Evict if at capacity
        if self.key_to_val.len() == self.capacity {
            let min_keys = self.freq_to_keys.get_mut(&self.min_freq).unwrap();
            let evict_key = min_keys.remove(0);

            // Update positions for remaining keys
            for i in 0..min_keys.len() {
                self.key_to_pos.insert(min_keys[i], i);
            }

            self.key_to_val.remove(&evict_key);
            self.key_to_freq.remove(&evict_key);
            self.key_to_pos.remove(&evict_key);
        }

        // Insert new key
        self.key_to_val.insert(key, value);
        self.key_to_freq.insert(key, 1);
        self.min_freq = 1;

        self.freq_to_keys.entry(1).or_insert_with(Vec::new);
        let keys = self.freq_to_keys.get_mut(&1).unwrap();
        self.key_to_pos.insert(key, keys.len());
        keys.push(key);
    }
}
