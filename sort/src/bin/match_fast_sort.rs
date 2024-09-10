use sort::fast_sorts::heap_sort;
use sort::fast_sorts::merge_sort;
use sort::fast_sorts::quick_sort;
use sort::op_file;
use sort::test_counter::test_algorithm_in_place;
use std::io;

fn main() -> io::Result<()> {
    let target = op_file::read_nums_from_file("rand_numbers.txt")?;
    test_algorithm_in_place(&mut target.clone(), quick_sort, "quick_sort");
    test_algorithm_in_place(&mut target.clone(), merge_sort, "merge_sort");
    test_algorithm_in_place(&mut target.clone(), heap_sort, "heap_sort");

    Ok(())
}
