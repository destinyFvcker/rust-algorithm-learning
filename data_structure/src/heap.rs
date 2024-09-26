/*
    堆是一种 完全二叉树，并且具有以下两种性质之一：

    1. 最小堆（Min-Heap）性质：
        - 每个节点的值都小于等于它的子节点的值。
        - 根节点是整个堆中的最小元素。

    2. 最大堆（Max-Heap）性质：
        - 每个节点的值都大于等于它的子节点的值。
        - 根节点是整个堆中的最大元素。

    ### 堆的关键性质：
    - 完全二叉树：堆的每一层都尽可能地填满，只有最后一层可以有未满的情况，并且所有的节点必须从左到右依次排列。
    - 插入与删除：堆的插入操作会将元素添加到树的最底层，通常是最左边的位置，然后通过“上移”操作（在最小堆中把较小的元素往上调整）保持堆的性质。
    删除通常是删除根节点（最小堆中的最小值或最大堆中的最大值），然后将最后一个元素移动到根节点，通过“下移”操作恢复堆的性质。

    ### 堆的用途：
    - 优先队列：堆常用于实现优先队列，其中根据优先级选择最优元素（在最小堆中选择最小值，在最大堆中选择最大值）。
    - 排序：堆可以用来实现堆排序（Heap Sort），堆排序的时间复杂度为 O(n log n)。
*/
use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

pub struct MaxHeap;

impl MaxHeap {
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

pub struct MinHeap;

impl MinHeap {
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

// public methods
impl<T> Heap<T>
where
    T: Default,
{
    /// 创建一个新的堆，可以根据 comparator 来决定是创建一个小根堆
    /// 还是一个大根堆
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    /// 返回堆的大小
    pub fn len(&self) -> usize {
        self.count
    }

    /// 判断堆是否为空
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// 向堆中添加一个元素
    pub fn add(&mut self, value: T) {
        self.count += 1;
        self.items.push(value);
        self.swim(self.count); // 插入后上移
    }

    /// 在取出堆顶元素的同时维护堆的性质
    pub fn pop(&mut self) -> Option<T> {
        if self.count == 0 {
            return None;
        }
        self.items.swap(1, self.count);
        let min = self.items.pop();
        self.count -= 1;
        if self.count > 0 {
            self.sink(1);
        }
        min
    }
}

// private methods
impl<T> Heap<T>
where
    T: Default,
{
    /// 返回根节点的索引
    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    /// 返回左子节点的索引
    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    /// 返回右子节点的索引
    fn right_child_idx(&self, idx: usize) -> usize {
        // 假如连左子节点都不存在，则右子节点肯定也不存在
        self.left_child_idx(idx) + 1
    }

    /// 返回最小子节点的索引
    fn smallest_child_idx(&self, idx: usize) -> usize {
        let left = self.left_child_idx(idx);
        let right = self.right_child_idx(idx);

        // 这里还是比较巧妙的，因为 left 就只比 right 小 1
        // 所以只判断 right 是否越界就可以了
        if right <= self.count && (self.comparator)(&self.items[right], &self.items[left]) {
            right
        } else {
            // 包含了只有左子节点的情况
            left
        }
    }

    /// 查找一个节点是否有字节点
    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    /// 递归上移，将新插入的元素与其父节点比较，如果不符合堆的比较规则，就交换位置
    fn swim(&mut self, mut idx: usize) {
        while idx > 1 {
            let parent = self.parent_idx(idx); // 解决 Rust 生命周期问题，避免重复借用
            if (self.comparator)(&self.items[idx], &self.items[parent]) {
                self.items.swap(idx, parent);
                idx = parent;
            } else {
                break;
            }
        }
    }

    /// 当删除堆顶元素时，最后一个元素会被移动到堆顶，然后通过下移操作回复堆的性质
    fn sink(&mut self, mut idx: usize) {
        while self.children_present(idx) {
            let smaller_child = self.smallest_child_idx(idx);
            if !(self.comparator)(&self.items[smaller_child], &self.items[idx]) {
                break;
            }
            self.items.swap(idx, smaller_child);
            idx = smaller_child;
        }
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}
