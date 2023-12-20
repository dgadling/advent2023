use std::io::BufRead;

pub fn day9() {
    day9_part1();
    day9_part2();
}

fn day9_part1() {
    let r = crate::utils::get_reader_for_day(9);

    let mut curr_nums: Vec<i128>;
    let mut final_nums: Vec<i128> = Vec::new();

    for line in r.lines().map(|l| l.unwrap()) {
        curr_nums = line
            .split_ascii_whitespace()
            .map(|c| c.parse::<i128>().unwrap())
            .collect();

        final_nums.push(get_last_num(curr_nums));
    }

    println!("Sum = {}", final_nums.iter().sum::<i128>());
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

    let mut curr_nums: Vec<i128>;
    let mut first_nums: Vec<i128> = Vec::new();
    let mut first_num: i128;

    for line in r.lines().map(|l| l.unwrap()) {
        curr_nums = line
            .split_ascii_whitespace()
            .map(|c| c.parse::<i128>().unwrap())
            .collect();

        first_num = get_first_num(curr_nums);
        first_nums.push(first_num);
    }

    println!("Sum = {}", first_nums.iter().sum::<i128>());
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
