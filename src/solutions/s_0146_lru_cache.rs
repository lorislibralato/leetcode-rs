#[allow(unused)]
struct LRUCache {
    inner: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(unused)]
impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            inner: vec![-1; capacity as usize],
        }
    }

    fn get(&self, key: i32) -> i32 {
        let idx = self.inner.capacity() as i32 % key;
        self.inner[idx as usize]
    }

    fn put(&self, key: i32, value: i32) {
        todo!()
    }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */

#[test]
fn test1() {
    todo!()
}
