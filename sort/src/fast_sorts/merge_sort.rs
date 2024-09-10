use std::mem;

pub fn merge_sort<T>(arr: &mut [T])
where
    T: Clone + Default + PartialOrd,
{
    if arr.len() > 1 {
        merge_sort_range(arr, 0, arr.len() - 1)
    }
}

fn merge_sort_range<T>(arr: &mut [T], lo: usize, hi: usize)
where
    T: Clone + Default + PartialOrd,
{
    if lo < hi {
        let mid = lo + ((hi - lo) >> 1);
        merge_sort_range(arr, lo, mid);
        merge_sort_range(arr, mid + 1, hi);
        merge_two_array(arr, lo, mid, hi);
    }
}

fn merge_two_array<T>(arr: &mut [T], lo: usize, mid: usize, hi: usize)
where
    T: Clone + Default + PartialOrd,
{
    let mut arr1 = arr[lo..=mid].to_vec();
    let mut arr2 = arr[mid + 1..=hi].to_vec();

    let (mut i, mut j) = (0, 0);
    while i < arr1.len() && j < arr2.len() {
        if arr1[i] < arr2[j] {
            arr[lo + i + j] = mem::take(&mut arr1[i]);
            i += 1;
        } else {
            arr[lo + i + j] = mem::take(&mut arr2[j]);
            j += 1;
        }
    }

    while i < arr1.len() {
        arr[lo + i + j] = mem::take(&mut arr1[i]);
        i += 1;
    }

    while j < arr2.len() {
        arr[lo + i + j] = mem::take(&mut arr2[j]);
        j += 1;
    }
}
