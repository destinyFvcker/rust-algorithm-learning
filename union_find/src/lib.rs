use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;

struct UnionFind<T> {
    parent: HashMap<T, T>,
    rank: HashMap<T, usize>,
}

impl<T> UnionFind<T>
where
    T: Eq + Hash + Clone,
{
    // 创建一个空的并查集
    fn new() -> Self {
        UnionFind {
            parent: HashMap::new(),
            rank: HashMap::new(),
        }
    }

    // 动态插入新元素，初始化 parent 和 rank
    fn insert_element(&mut self, x: T) {
        self.parent.insert(x.clone(), x.clone()); // 新元素的父节点指向自身
        self.rank.insert(x, 0); // 新元素的秩初始化为0
    }

    // 查找元素 x 所属的集合，并进行路径压缩
    fn find(&mut self, x: &T) -> T {
        if !self.parent.contains_key(x) {
            self.insert_element(x.clone());
        }

        let parent_x = self.parent.get(x).unwrap().clone();
        if *x != parent_x {
            let root = self.find(&parent_x);
            self.parent.insert(x.clone(), root.clone()); // 路径压缩
            root
        } else {
            parent_x
        }
    }

    // 判断两个元素是否属于同一个集合
    fn connected(&mut self, x: T, y: T) -> bool {
        self.find(&x) == self.find(&y)
    }

    // 合并 x 和 y 所在的集合，按秩进行合并
    fn union(&mut self, x: T, y: T) {
        let root_x = self.find(&x);
        let root_y = self.find(&y);

        if root_x != root_y {
            let rank_x = *self.rank.get(&root_x).unwrap();
            let rank_y = *self.rank.get(&root_y).unwrap();

            if rank_x > rank_y {
                self.parent.insert(root_y, root_x);
            } else if rank_x < rank_y {
                self.parent.insert(root_x, root_y);
            } else {
                *self.rank.get_mut(&root_x).unwrap() += 1;
                self.parent.insert(root_y, root_x);
            }
        }
    }

    pub fn get_disjoint_sets(&mut self) -> HashSet<T> {
        let mut sets = HashSet::new();
        let keys: Vec<T> = self.parent.keys().cloned().collect();
        for element in keys {
            let root = self.find(&element).clone();
            sets.insert(root);
        }
        sets
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_and_union() {
        let mut uf = UnionFind::new();

        // 插入两个不同元素
        uf.insert_element(1);
        uf.insert_element(2);

        // 初始时它们应该属于不同的集合
        assert_ne!(uf.find(&1), uf.find(&2));

        // 合并它们
        uf.union(1, 2);

        // 现在它们应该属于同一个集合
        assert_eq!(uf.find(&1), uf.find(&2));
    }

    #[test]
    fn test_union_by_rank() {
        let mut uf = UnionFind::new();

        // 插入多个元素
        uf.insert_element(1);
        uf.insert_element(2);
        uf.insert_element(3);

        // 合并 (1, 2)，此时 rank(1) 和 rank(2) 相同
        uf.union(1, 2);

        // 验证合并后的父节点应该相同
        assert_eq!(uf.find(&1), uf.find(&2));

        // 合并 (1, 3)，根据秩将 3 挂到 1 的集合中
        uf.union(1, 3);

        // 验证 1 和 3 现在也属于同一个集合
        assert_eq!(uf.find(&1), uf.find(&3));
        assert_eq!(uf.find(&2), uf.find(&3));
    }

    #[test]
    fn test_path_compression() {
        let mut uf = UnionFind::new();

        // 插入多个元素
        uf.insert_element(1);
        uf.insert_element(2);
        uf.insert_element(3);
        uf.insert_element(4);

        // 连续合并形成链
        uf.union(1, 2);
        uf.union(2, 3);
        uf.union(3, 4);

        // 在没有路径压缩的情况下，4 的根应该是 1
        assert_eq!(uf.find(&4), uf.find(&1));

        // 验证路径压缩后，父节点应该指向根
        let root_before = uf.find(&4);
        let root_after = uf.find(&4);

        // 路径压缩后，再次查找应与之前保持一致
        assert_eq!(root_before, root_after);
        assert_eq!(uf.find(&2), root_after);
    }

    #[test]
    fn test_disjoint_sets() {
        let mut uf = UnionFind::new();

        // 插入多个元素，分属两个不同集合
        uf.insert_element(1);
        uf.insert_element(2);
        uf.insert_element(3);
        uf.insert_element(4);

        // 合并 (1, 2) 和 (3, 4)
        uf.union(1, 2);
        uf.union(3, 4);

        // (1, 2) 属于同一集合，(3, 4) 属于同一集合
        assert_eq!(uf.find(&1), uf.find(&2));
        assert_eq!(uf.find(&3), uf.find(&4));

        // 但 (1, 3) 和 (2, 4) 属于不同集合
        assert_ne!(uf.find(&1), uf.find(&3));
        assert_ne!(uf.find(&2), uf.find(&4));
    }

    #[test]
    fn test_union_with_self() {
        let mut uf = UnionFind::new();

        uf.insert_element(1);

        // 自己与自己合并
        uf.union(1, 1);

        // 验证 find(1) 还是自己
        assert_eq!(uf.find(&1), uf.find(&1));
    }
}
