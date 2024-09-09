// use sort::gnome_sort::gnome_sort;
use sort::op_file;
use sort::slow_sorts::bubble_sort;
use sort::slow_sorts::insertion_sort;
use sort::slow_sorts::insertion_sort_binary_search;
use sort::slow_sorts::selection_sort;
use sort::test_counter::test_algorithm_in_place;
use std::io;

fn main() -> io::Result<()> {
    let target = op_file::read_nums_from_file("../../rand_numbers.txt")?;
    test_algorithm_in_place(&mut target.clone(), insertion_sort);
    test_algorithm_in_place(&mut target.clone(), selection_sort);
    test_algorithm_in_place(&mut target.clone(), bubble_sort);
    test_algorithm_in_place(&mut target.clone(), insertion_sort_binary_search);
    // test_algorithm(&mut target.clone(), gnome_sort)?;
    Ok(())
}
