pub fn quick_sort<T: PartialOrd>(arr: &mut [T]) {
    if arr.len() > 1 {
        quick_sort_range(arr, 0, arr.len() - 1);
    }
}

fn quick_sort_range<T: PartialOrd>(arr: &mut [T], lo: usize, hi: usize) {
    if lo < hi {
        let pos = partition(arr, lo, hi);
        if pos != 0 {
            quick_sort_range(arr, lo, pos - 1);
        }
        quick_sort_range(arr, pos + 1, hi);
    }
}

fn partition<T: PartialOrd>(arr: &mut [T], lo: usize, hi: usize) -> usize {
    // 默认选取lo作为pivot
    let pivot = lo;

    let (mut left, mut right) = (lo, hi);
    while left < right {
        // 找到右边第一个不大于等于arr[pivot]的元素
        while left < right && arr[right] >= arr[pivot] {
            right -= 1;
        }

        // 找到左边第一个不小于等于arr[pivot]的元素
        while left < right && arr[left] <= arr[pivot] {
            left += 1;
        }

        // 交换前面找到的两个元素
        if left != right {
            arr.swap(left, right);
        }
    }

    arr.swap(pivot, left);

    left
}
