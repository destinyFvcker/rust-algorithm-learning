use std::{
    fs::OpenOptions,
    io::{self, Write},
};

use rand::Rng;

fn gen_rand_num_to_file(length: usize) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("rand_numbers.txt")?;

    let mut rng = rand::thread_rng();
    let vec: Vec<isize> = (0..length).map(|_| rng.gen::<isize>()).collect();

    for number in vec {
        writeln!(file, "{}", number)?;
    }
    Ok(())
}

fn main() -> io::Result<()> {
    gen_rand_num_to_file(10_000)
}
