pub struct HashMap {
    buckets: Vec<Vec<(String, i32)>>,
    entries_count: u32,
}

// Tunables
const MIN_BUCKET_COUNT: usize = 10;
const MAX_LOAD_FACTOR: f32 = 1.5;
const MIN_LOAD_FACTOR: f32 = 0.25;
const RESIZE_FACTOR: f32 = 1.4;

impl HashMap {
    pub fn new() -> Self {
        let mut buckets = Vec::with_capacity(MIN_BUCKET_COUNT);
        for _ in 0..MIN_BUCKET_COUNT {
            buckets.push(Vec::new());
        }
        HashMap {
            buckets,
            entries_count: 0,
        }
    }

    // Get item by key
    pub fn get(&self, key: &str) -> Option<i32> {
        let bucket = &self.buckets[HashMap::hash(key, self.buckets.len()) as usize];
        for i in 0..bucket.len() {
            if bucket[i].0 == key {
                return Some(bucket[i].1);
            }
        }
        None
    }

    pub fn set(&mut self, key: &str, value: i32) {
        self.delete(key);

        let bucket_index = HashMap::hash(key, self.buckets.len()) as usize;
        let v = &mut self.buckets[bucket_index];
        v.push((key.to_string(), value));
        self.entries_count += 1;

        self.resize_if_necessary();
    }

    pub fn delete(&mut self, key: &str) {
        let bucket_index = HashMap::hash(key, self.buckets.len()) as usize;
        let v = &mut self.buckets[bucket_index];
        for i in 0..v.len() {
            if v[i].0 == key {
                v.remove(i);
                self.entries_count -= 1;
                return;
            }
        }
        self.resize_if_necessary();
    }

    pub fn get_bucket(&self, idx: usize) -> Result<&Vec<(String, i32)>, String> {
        match self.buckets.get(idx) {
            Some(bucket) => Ok(bucket),
            None => Err(String::from("bucket index out of bounds")),
        }
    }

    pub fn get_entries_count(&self) -> u32 {
        return self.entries_count;
    }

    pub fn get_buckets_count(&self) -> usize {
        return self.buckets.len();
    }

    // uses the FNV-1a hash
    fn hash(key: &str, bucket_count: usize) -> usize {
        const FNV_OFFSET_BASIS: u64 = 14695981039346656037;
        const FNV_PRIME: u64 = 1099511628211;

        let mut hash = FNV_OFFSET_BASIS;
        for octet in key.as_bytes() {
            hash ^= *octet as u64;
            hash = hash.wrapping_mul(FNV_PRIME);
        }

        (hash as usize) % bucket_count
    }

    fn resize_if_necessary(&mut self) {
        let load_factor = self.entries_count as f32 / self.buckets.len() as f32;

        if load_factor > MAX_LOAD_FACTOR {
            let new_size = (self.buckets.len() as f32 * RESIZE_FACTOR).ceil() as usize;
            self.resize(new_size);
        } else if load_factor < MIN_LOAD_FACTOR && self.buckets.len() / 2 > MIN_BUCKET_COUNT {
            let new_size = (self.buckets.len() as f32 / RESIZE_FACTOR).ceil() as usize;
            self.resize(new_size);
        }
    }

    fn resize(&mut self, new_bucket_count: usize) {
        println!("{}", new_bucket_count);
        let mut new_buckets = Vec::with_capacity(new_bucket_count);
        for _ in 0..new_bucket_count {
            new_buckets.push(Vec::new());
        }

        for bucket in &self.buckets {
            for item in bucket {
                new_buckets[HashMap::hash(&item.0, new_bucket_count)]
                    .push((item.0.clone(), item.1));
            }
        }
        self.buckets = new_buckets;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_item_count() {
        let mut hm = HashMap::new();
        for i in 0..100 {
            hm.set(&(&i.to_string()), i);
        }
        assert_eq!(hm.get_entries_count(), 100);
    }

    #[test]
    fn item_not_found() {
        let hm = HashMap::new();
        let key = "test";
        assert_eq!(None, hm.get(key));
    }

    #[test]
    fn item_found() {
        let mut hm = HashMap::new();
        hm.set("test", 5);
        let key = "test";
        assert_eq!(Some(5), hm.get(key));
    }
}
