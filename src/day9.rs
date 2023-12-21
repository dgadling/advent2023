const DAY: u8 = 9;

pub fn main() {
    part1();
    part2();
}

fn part1() {
    let answer: i128 = crate::utils::lines(DAY)
        .map(|l| get_last_num(crate::utils::ints(l)))
        .sum();

    done!(DAY, 1, answer);
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

fn part2() {
    let answer: i128 = crate::utils::lines(DAY)
        .map(|l| get_first_num(crate::utils::ints(l)))
        .sum();

    done!(DAY, 2, answer);
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
