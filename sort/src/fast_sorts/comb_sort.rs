pub fn comb_sort<T: PartialOrd>(arr: &mut [T]) {
    let mut gap = arr.len();
    let shrink = 1.3;
    let mut sorted = false;

    while !sorted {
        gap = (gap as f32 / shrink).floor() as usize;
        if gap < 1 {
            sorted = true;
            gap = 1;
        }

        for i in 0..arr.len() - gap {
            let j = i + gap;
            if arr[i] > arr[j] {
                arr.swap(i, j);
                sorted = false;
            }
        }
    }
}

pub fn comb_sort11<T: PartialOrd>(arr: &mut [T]) {
    let mut gap = arr.len();
    let shrink = 1.3;
    let mut sorted = false;

    while !sorted {
        gap = (gap as f32 / shrink).floor() as usize;
        if gap < 1 {
            sorted = true;
            gap = 1;
        }
        if 9 == gap || 10 == gap {
            gap = 11;
        }

        for i in 0..arr.len() - gap {
            let j = i + gap;
            if arr[i] > arr[j] {
                arr.swap(i, j);
                sorted = false;
            }
        }
    }
}
