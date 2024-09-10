pub fn heap_sort<T: PartialOrd>(arr: &mut [T]) {
    let size = arr.len();

    for i in (0..size / 2).rev() {
        heaplify(arr, i, size);
    }

    for i in (0..size).rev() {
        arr.swap(0, i);
        heaplify(arr, 0, i);
    }
}

fn heaplify<T: PartialOrd>(arr: &mut [T], root: usize, end: usize) {
    let mut largest = root;

    let left_child = root * 2 + 1;
    if left_child < end && arr[left_child] > arr[largest] {
        largest = left_child;
    }

    let right_child = left_child + 1;
    if right_child < end && arr[right_child] > arr[largest] {
        largest = right_child;
    }

    if largest != root {
        arr.swap(root, largest);
        heaplify(arr, largest, end);
    }
}
