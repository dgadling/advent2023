use crate::utils;
use std::fmt;
use std::io::BufRead;

#[derive(Debug, Default)]
struct Position {
    x: u16,
    y: u16,
}

#[derive(Debug, Default)]
struct PartNumber {
    row: u16,
    number: u16,
    left_bound: u16,
    right_bound: u16,
    top_bound: u16,
    lower_bound: u16,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl PartNumber {
    fn is_adjacent_to(self: &PartNumber, pos: &Position) -> bool {
        // Adjacent means that all of these must be true:

        (pos.x >= self.left_bound && pos.x <= self.right_bound)
            && (pos.y >= self.lower_bound && pos.y <= self.top_bound)
    }
}

impl fmt::Display for PartNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:03} @ ({:03}-{:03}, {:03}-{:03})",
            self.number, self.left_bound, self.right_bound, self.top_bound, self.lower_bound
        )
    }
}

fn find_numbers_and_symbols() -> (Vec<PartNumber>, Vec<Position>) {
    let reader = utils::get_reader_for_day(3);
    let mut curr_num = String::new();
    let mut in_num = false;
    let mut nums: Vec<PartNumber> = Vec::new();
    let mut syms: Vec<Position> = Vec::new();
    let mut curr_part = PartNumber::default();

    let input_data: Vec<Vec<char>> = reader
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();

    for (line_no, line) in input_data.iter().enumerate() {
        for (col_no, our_char) in line.iter().enumerate() {
            if our_char.is_ascii_digit() {
                curr_num.push(*our_char);

                if !in_num {
                    in_num = true;
                    curr_part.row = line_no as u16;
                    curr_part.lower_bound = if line_no == 0 {
                        0 as u16
                    } else {
                        (line_no - 1) as u16
                    };
                    curr_part.top_bound = (line_no + 1) as u16;
                    curr_part.left_bound = if col_no == 0 {
                        0 as u16
                    } else {
                        col_no as u16 - 1
                    };
                }
                continue;
            }

            // It's either a symbol or a '.'. Either way if we were in the middle of a number, we're not now!
            if in_num {
                curr_part.right_bound = col_no as u16;
                curr_part.number = curr_num.parse::<u16>().unwrap();
                nums.push(curr_part);

                curr_num.clear();
                curr_part = PartNumber::default();
                in_num = false;
            }

            if *our_char == '.' {
                continue;
            }

            syms.push(Position {
                x: col_no as u16,
                y: line_no as u16,
            });
        }

        // We hit the end of the line, literally, and still in the middle of a number. Finish it off.
        if in_num {
            curr_part.right_bound = line.len() as u16;
            curr_part.number = curr_num.parse::<u16>().unwrap();
            nums.push(curr_part);

            curr_num.clear();
            curr_part = PartNumber::default();
            in_num = false;
        }
    }

    (nums, syms)
}

fn day3_part1() {
    let mut our_sum: u128 = 0;

    let (nums, syms) = find_numbers_and_symbols();

    for num in nums {
        if syms.iter().any(|s| num.is_adjacent_to(s)) {
            our_sum += num.number as u128;
        }
    }
    println!("Day 3, part 1 = {}", our_sum);
}

pub fn day3() {
    day3_part1();
}
