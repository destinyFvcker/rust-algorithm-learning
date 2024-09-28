use std::collections::{HashMap, VecDeque};

struct LRUCache {
    capacity: usize,
    map: HashMap<i32, (i32, usize)>, // 键 -> (值, 链表中的位置)
    list: VecDeque<i32>,             // 双端队列，用于维护 LRU 的顺序
}

impl LRUCache {
    // 初始化 LRU 缓存，设定容量
    fn new(capacity: i32) -> Self {
        LRUCache {
            capacity: capacity as usize,
            map: HashMap::new(),
            list: VecDeque::new(),
        }
    }

    // 获取键的值，如果键不存在，返回 -1
    fn get(&mut self, key: i32) -> i32 {
        if let Some(&(value, _)) = self.map.get(&key) {
            // 如果键存在，将其提升为最近使用的
            self.make_recent(key);
            value
        } else {
            -1
        }
    }

    // 插入或更新键值
    fn put(&mut self, key: i32, value: i32) {
        if self.map.contains_key(&key) {
            // 如果键已经存在，更新值并将其提升为最近使用的
            self.make_recent(key);
            if let Some(entry) = self.map.get_mut(&key) {
                entry.0 = value; // 更新值
            }
        } else {
            // 如果键不存在，检查是否超出容量
            if self.map.len() == self.capacity {
                // 容量已满，移除最久未使用的键
                if let Some(least_recent) = self.list.pop_back() {
                    self.map.remove(&least_recent);
                }
            }
            // 插入新元素
            self.list.push_front(key);
            self.map.insert(key, (value, 0));
        }
    }

    // 将某个键提升为最近使用的
    fn make_recent(&mut self, key: i32) {
        // 将该键移动到双端队列的前面，表示最近使用
        if let Some(position) = self.list.iter().position(|&x| x == key) {
            self.list.remove(position); // 从原位置移除
        }
        self.list.push_front(key); // 插入到队列头部
    }
}
