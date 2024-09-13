use std::cmp::Ordering;

pub fn binary_search<T: Ord>(item: &T, arr: &[T]) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + (right - left) / 2;

        match item.cmp(&arr[mid]) {
            Ordering::Less => right = mid,
            Ordering::Equal => return Some(mid),
            Ordering::Greater => left = mid + 1,
        }
    }

    None
}

pub fn binary_search_rec<T: Ord>(
    list_of_items: &[T],
    target: &T,
    left: &usize,
    right: &usize,
) -> Option<usize> {
    if left >= right {
        return None;
    }

    let middle: usize = left + (right - left) / 2;
    match target.cmp(&list_of_items[middle]) {
        Ordering::Less => binary_search_rec(list_of_items, target, left, &middle),
        Ordering::Equal => Some(middle),
        Ordering::Greater => binary_search_rec(list_of_items, target, &(middle + 1), right),
    }
}
