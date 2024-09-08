// pub fn insertion_sort<T: PartialOrd>(arr: &mut [T]) {
//     // 从第二个元素开始排序
//     for i in 1..arr.len() {
//         // 找到arr[i]该插入的位置
//         let mut j = i;
//         while j > 0 && arr[j - 1] > arr[j] {
//             arr.swap(j - 1, j);
//             j -= 1;
//         }
//     }
// }

pub fn insertion_sort<T: PartialOrd>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

pub fn insertion_sort_binary_search<T: PartialOrd + Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let pos = arr[..i].binary_search(&arr[i]).unwrap_or_else(|pos| pos);
        let mut j = i;
        while j > pos {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}
