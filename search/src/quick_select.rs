use std::cmp::Ordering;

pub fn quick_select<T>(input: &mut [T], k: usize) -> Option<T>
where
    T: PartialOrd + Copy,
{
    if input.is_empty() {
        return None;
    }

    let kth = quick_select_range(input, k, 0, input.len() - 1);
    Some(kth)
}

fn quick_select_range<T>(input: &mut [T], k: usize, lo: usize, hi: usize) -> T
where
    T: PartialOrd + Copy,
{
    if lo == hi {
        return input[lo];
    }

    let pivot = partition(input, lo, hi);

    match k.cmp(&pivot) {
        Ordering::Equal => input[pivot],
        Ordering::Less => quick_select_range(input, k, lo, pivot - 1),
        Ordering::Greater => quick_select_range(input, k, pivot + 1, hi),
    }
}

fn partition<T: PartialOrd>(arr: &mut [T], lo: usize, hi: usize) -> usize {
    let pivot = lo;
    let (mut left, mut right) = (lo, hi);

    while left < right {
        while left < right && arr[right] >= arr[pivot] {
            right -= 1;
        }

        while left < right && arr[left] <= arr[pivot] {
            left += 1;
        }

        if left != right {
            arr.swap(left, right);
        }
    }

    arr.swap(pivot, left);

    left
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quickselect_basic() {
        let mut arr = vec![3, 6, 8, 2, 10, 1, 4];
        assert_eq!(quick_select(&mut arr, 2), Some(3)); // 第 2 小的元素应该是 3
    }

    #[test]
    fn test_quickselect_single_element() {
        let mut arr = vec![7];
        assert_eq!(quick_select(&mut arr, 0), Some(7)); // 唯一的元素应该返回
    }

    #[test]
    fn test_quickselect_large_k() {
        let mut arr = vec![12, 3, 5, 7, 19, 1];
        assert_eq!(quick_select(&mut arr, 5), Some(19)); // 第 5 小的元素是 19
    }

    #[test]
    fn test_quickselect_first_element() {
        let mut arr = vec![8, 3, 7, 6, 10, 9, 2];
        assert_eq!(quick_select(&mut arr, 0), Some(2)); // 第 0 小的元素是 2
    }

    #[test]
    fn test_quickselect_last_element() {
        let mut arr = vec![15, 6, 2, 9, 11, 7];
        assert_eq!(quick_select(&mut arr, 5), Some(15)); // 最后一个元素应该是 15
    }

    #[test]
    fn test_quickselect_duplicate_elements() {
        let mut arr = vec![5, 1, 5, 3, 7, 3, 5];
        assert_eq!(quick_select(&mut arr, 3), Some(5)); // 排序后第 3 小的元素是 5
    }
}
