use std::env::current_dir;
use std::fs::File;
use std::io::{self, BufRead, BufReader, ErrorKind};
use std::u128;

fn day1_part1(in_f_path: &str) -> Result<u128, io::Error> {
    let file = File::open(in_f_path)
        .expect(format!("Really, the path ({}) is wrong?", in_f_path).as_str());
    let buf_reader = BufReader::new(file);
    let mut our_sum: u128 = 0;

    for line in buf_reader.lines() {
        let numbers: Vec<u32> = line?.chars().filter_map(|a| a.to_digit(10)).collect();
        let first_digit = numbers
            .first()
            .ok_or(io::Error::new(ErrorKind::Other, "No first digit?!"))?;
        let last_digit = numbers
            .last()
            .ok_or(io::Error::new(ErrorKind::Other, "No last digit?!"))?;
        our_sum += (first_digit * 10) as u128 + (*last_digit as u128);
    }

    return Ok(our_sum);
}

fn main() {
    let current_dir = current_dir().expect("Can't get current directory?!");
    let result = day1_part1(
        current_dir
            .join("day")
            .join("1")
            .join("input.txt")
            .to_str()
            .unwrap(),
    );
    match result {
        Ok(total) => {
            println!("Total is {}", total)
        }
        Err(error) => panic!("Oh jeeze: {:?}", error),
    }
}
