pub fn shell_sort<T: Ord + Copy>(values: &mut [T]) {
    // shell sort workks by swiping the value at a given gap
    // and decreasing the gap to 1
    let mut count_sublist = values.len() / 2;
    while count_sublist > 0 {
        for pos_start in 0..count_sublist {
            insertion(values, pos_start, count_sublist);
        }
        count_sublist /= 2;
    }
}

fn insertion<T: Ord + Copy>(values: &mut [T], start: usize, gap: usize) {
    for i in ((start + gap)..values.len()).step_by(gap) {
        let val_current = values[i];
        let mut pos = i;
        // make swaps
        while pos >= gap && values[pos - gap] > val_current {
            values[pos] = values[pos - gap];
            pos -= gap;
        }
        values[pos] = val_current;
    }
}
