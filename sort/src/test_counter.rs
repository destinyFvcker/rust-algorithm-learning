use std::time::Instant;

pub fn test_algorithm_in_place<T: Ord + Copy>(
    target: &mut [T],
    func: fn(&mut [T]),
    func_name: &str,
) {
    let start = Instant::now();
    func(target);
    let end = Instant::now();

    assert!(test_is_sorted(target));

    println!("{} process cost: {}", func_name, (end - start).as_micros());
}

pub fn test_algorithm<T: Ord + Copy>(target: &mut [T], func: fn(&[T]) -> Vec<T>, func_name: &str) {
    let start = Instant::now();
    let result = func(target);
    let end = Instant::now();

    assert!(test_is_sorted(&result));

    println!("{} process cost: {}", func_name, (end - start).as_micros());
}

pub fn test_is_sorted<T: PartialOrd>(target: &[T]) -> bool {
    for i in 1..target.len() {
        if target[i - 1] > target[i] {
            return false;
        }
    }

    true
}
