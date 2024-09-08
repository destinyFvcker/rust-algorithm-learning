use std::io;
use std::time::Instant;

pub fn test_algorithm_in_place<T: Ord + Copy>(target: &mut [T], func: fn(&mut [T])) {
    let start = Instant::now();
    func(target);
    let end = Instant::now();

    println!("cost: {}", (end - start).as_micros());
}

pub fn test_algorithm<T: Ord + Copy>(target: &mut [T], func: fn(&[T]) -> Vec<T>) -> io::Result<()> {
    let start = Instant::now();
    func(target);
    let end = Instant::now();

    println!("cost: {}", (end - start).as_micros());
    Ok(())
}
