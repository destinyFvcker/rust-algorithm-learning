pub fn shell_sort<T: PartialOrd>(arr: &mut [T]) {
    let mut sublist_len = arr.len() / 2;

    while sublist_len > 0 {
        for i in 0..sublist_len {
            insertion(arr, i, sublist_len);
        }
        sublist_len /= 2;
    }
}

fn insertion<T: PartialOrd>(arr: &mut [T], start: usize, gap: usize) {
    let size = arr.len();
    for i in ((start + gap)..size).step_by(gap) {
        let mut j = i;
        while j > start && arr[j - gap] > arr[j] {
            arr.swap(j - gap, j);
            j -= gap;
        }
    }
}
