pub mod utils;

#[macro_export]
macro_rules! done {
    ($day:expr, $part:expr, $answer:expr) => {
        println!("Day {:>2}, part {} = {}", $day, $part, $answer);
    };
}

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

fn main() {
    vec![
    day1::main,
    day2::main,
    day3::main,
    day4::main,
    day5::main,
    day6::main,
    day7::main,
    day8::main,
    day9::main,
    ].last().unwrap()()
}
