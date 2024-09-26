use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

struct RandomizedSet {
    map: HashMap<i32, usize>, // 记录元素及其在vec中的索引
    vec: Vec<i32>,            // 存储元素
}

impl RandomizedSet {
    // 初始化一个新的 RandomizedSet 对象
    fn new() -> Self {
        RandomizedSet {
            map: HashMap::new(),
            vec: Vec::new(),
        }
    }

    // 插入一个元素，如果元素不存在则插入并返回 true，否则返回 false
    fn insert(&mut self, val: i32) -> bool {
        if self.map.contains_key(&val) {
            return false;
        }
        self.vec.push(val);
        self.map.insert(val, self.vec.len() - 1); // 记录元素在 vec 中的索引
        true
    }

    // 删除一个元素，如果元素存在则删除并返回 true，否则返回 false
    fn remove(&mut self, val: i32) -> bool {
        if let Some(&index) = self.map.get(&val) {
            let last = *self.vec.last().unwrap();
            self.vec[index] = last; // 用最后一个元素覆盖要删除的元素
            self.map.insert(last, index); // 更新最后一个元素的索引
            self.vec.pop(); // 删除最后一个元素
            self.map.remove(&val); // 从 map 中删除
            return true;
        }
        false
    }

    // 随机返回一个集合中的元素
    fn get_random(&self) -> i32 {
        let mut seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        seed = seed % self.vec.len() as u128;
        self.vec[seed as usize]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_test() {
        let mut obj = RandomizedSet::new();
        assert_eq!(obj.insert(1), true);
        assert_eq!(obj.insert(1), false);
        assert_eq!(obj.insert(2), true);
        assert_eq!(obj.remove(1), true);
        assert_eq!(obj.remove(1), false);
        println!("Random element: {}", obj.get_random());
    }
}
