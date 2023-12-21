const DAY: u8 = 1;

pub fn main() {
    part1();
    part2();
}

use std::collections::HashMap;
use std::u128;

fn part1() {
    let mut our_sum: u128 = 0;

    for line in crate::utils::lines(DAY) {
        let numbers: Vec<u32> = line.chars().filter_map(|a| a.to_digit(10)).collect();
        our_sum += (numbers.first().unwrap() * 10) as u128 + (*numbers.last().unwrap() as u128);
    }

    done!(DAY, 1, our_sum);
}

fn part2() {
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

    for line in crate::utils::lines(DAY){
        first_digit_idx = usize::MAX;
        first_digit = &0;
        last_digit_idx = 0;
        last_digit = &0;

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

    done!(DAY, 2, our_sum);
}
