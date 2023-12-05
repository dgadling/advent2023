use std::collections::HashMap;
use std::io::{self, BufRead, ErrorKind};
use std::u128;

use crate::utils;

fn part1() -> Result<u128, io::Error> {
    let buf_reader = utils::get_reader_for_day(1);
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

fn part2() -> Result<u128, io::Error> {
    let buf_reader = utils::get_reader_for_day(1);
    let mut our_sum: u128 = 0;
    let digits: HashMap<&str, u8> = HashMap::from([
        ("one", 1),
        ("1", 1),
        ("two", 2),
        ("2", 2),
        ("three", 3),
        ("3", 3),
        ("four", 4),
        ("4", 4),
        ("five", 5),
        ("5", 5),
        ("six", 6),
        ("6", 6),
        ("seven", 7),
        ("7", 7),
        ("eight", 8),
        ("8", 8),
        ("nine", 9),
        ("9", 9),
    ]);

    let mut first_digit_idx: usize;
    let mut first_digit: &u8;
    let mut last_digit_idx: usize;
    let mut last_digit: &u8;

    for _line in buf_reader.lines() {
        first_digit_idx = usize::MAX;
        first_digit = &0;
        last_digit_idx = 0;
        last_digit = &0;

        let line = _line.unwrap();
        for digit in digits.keys() {
            if let Some(idx) = line.find(digit) {
                if idx < first_digit_idx {
                    first_digit_idx = idx;
                    first_digit = digits.get(digit).unwrap();
                }

                // We haven't found a last digit yet, there may only be this one digit!
                if last_digit == &0 {
                    last_digit_idx = idx;
                    last_digit = digits.get(digit).unwrap();
                }
            }

            if let Some(idx) = line.rfind(digit) {
                if idx > last_digit_idx {
                    last_digit_idx = idx;
                    last_digit = digits.get(digit).unwrap();
                }
            }
        }

        our_sum += ((first_digit * 10) + *last_digit) as u128;
    }

    return Ok(our_sum);
}

pub fn day1() {
    let part1_result = part1();
    match part1_result {
        Ok(total) => {
            println!("Part 1 total is {}", total)
        }
        Err(error) => panic!("Oh jeeze: {:?}", error),
    }

    let part2_result = part2();
    match part2_result {
        Ok(total) => {
            println!("Part 2 total is {}", total)
        }
        Err(error) => panic!("Oh jeeze: {:?}", error),
    }
}
