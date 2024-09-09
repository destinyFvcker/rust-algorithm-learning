use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};

pub fn read_nums_from_file(path: &str) -> io::Result<Vec<isize>> {
    let mut numbers = Vec::new();
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if let Ok(number) = line.trim().parse::<isize>() {
            numbers.push(number);
        }
    }

    Ok(numbers)
}

pub fn write_nums_into_file(path: &str, numbers: &[isize]) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)?;

    for number in numbers {
        writeln!(file, "{}", number)?;
    }

    Ok(())
}
