pub fn selection_sort<T: PartialOrd>(arr: &mut [T]) {
    if arr.len() < 1 {
        return;
    }

    let size = arr.len();
    for i in 0..size - 1 {
        let mut min_index = i;
        for j in (i + 1)..size {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }

        if min_index != i {
            arr.swap(i, min_index);
        }
    }
}
