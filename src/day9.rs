use std::io::BufRead;

pub fn day9() {
    day9_part1();
    day9_part2();
}

fn day9_part1() {
    let r = crate::utils::get_reader_for_day(9);

    let answer: i128 = r
        .lines()
        .map(|l| get_last_num(crate::utils::ints(l.unwrap())))
        .sum();

    println!("Day  9, part 1 = {}", answer);
}

fn get_last_num(nums: Vec<i128>) -> i128 {
    if nums.iter().all(|n| *n == 0) {
        return 0;
    }

    return nums.last().unwrap()
        + get_last_num(
            (0..(nums.len() - 1))
                .map(|i| nums[i + 1] - nums[i])
                .collect(),
        );
}

fn day9_part2() {
    let r = crate::utils::get_reader_for_day(9);

    let answer: i128 = r
        .lines()
        .map(|l| get_first_num(crate::utils::ints(l.unwrap())))
        .sum();

    println!("Day  9, part 2 = {}", answer);
}

fn get_first_num(nums: Vec<i128>) -> i128 {
    if nums.iter().all(|n| *n == 0) {
        return 0;
    }

    return nums.first().unwrap()
        - get_first_num(
            (0..(nums.len() - 1))
                .map(|i| nums[i + 1] - nums[i])
                .collect(),
        );
}
