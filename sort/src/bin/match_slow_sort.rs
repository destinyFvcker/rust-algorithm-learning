use sort::fast_sorts::comb_sort;
use sort::fast_sorts::comb_sort11;
// use sort::gnome_sort::gnome_sort;
use sort::op_file;
use sort::slow_sorts::bubble_sort;
use sort::slow_sorts::insertion_sort;
use sort::slow_sorts::insertion_sort_binary_search;
use sort::slow_sorts::selection_sort;
use sort::test_counter::test_algorithm_in_place;
use std::io;

fn main() -> io::Result<()> {
    let target = op_file::read_nums_from_file("rand_numbers.txt")?;
    test_algorithm_in_place(&mut target.clone(), insertion_sort, "insertion_sort");
    test_algorithm_in_place(&mut target.clone(), selection_sort, "selection_sort");
    test_algorithm_in_place(&mut target.clone(), bubble_sort, "bubble_sort");
    test_algorithm_in_place(
        &mut target.clone(),
        insertion_sort_binary_search,
        "insertion_sort_binary_search",
    );
    test_algorithm_in_place(&mut target.clone(), comb_sort, "comb_sort");
    test_algorithm_in_place(&mut target.clone(), comb_sort11, "comb_sort11");

    // test_algorithm(&mut target.clone(), gnome_sort)?;
    Ok(())
}
