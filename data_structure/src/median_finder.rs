use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct MedianFinder {
    small: BinaryHeap<i32>,          // 最大堆，存储较小的一半元素
    large: BinaryHeap<Reverse<i32>>, // 最小堆，存储较大的一半元素
}

impl MedianFinder {
    fn new() -> Self {
        MedianFinder {
            small: BinaryHeap::new(),
            large: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        // 将元素加入到最大堆
        self.small.push(num);

        // 确保最大堆中的所有元素都小于最小堆中的元素
        if let Some(small_top) = self.small.pop() {
            self.large.push(Reverse(small_top));
        }

        // 保证最大堆的元素数量 >= 最小堆的元素数量
        if self.small.len() < self.large.len() {
            if let Some(Reverse(large_top)) = self.large.pop() {
                self.small.push(large_top);
            }
        }
    }

    fn find_median(&self) -> f64 {
        if self.small.len() > self.large.len() {
            *self.small.peek().unwrap() as f64
        } else {
            let small_top = *self.small.peek().unwrap() as f64;
            let large_top = self.large.peek().unwrap().0 as f64;
            (small_top + large_top) / 2.0
        }
    }
}
