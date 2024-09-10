pub fn selection_sort<T: PartialOrd>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let size = arr.len();
    for i in 0..size - 1 {
        let mut min_index = i;
        for j in i + 1..size {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }

        if i != min_index {
            arr.swap(min_index, i);
        }
    }
}
