use std::{collections::HashMap, io::BufRead};

use crate::utils;

fn get_game_index(raw_line: &str) -> u128 {
    let first_cut: Vec<&str> = raw_line.split(": ").collect();

    let game_info: Vec<&str> = first_cut.first().unwrap().split(" ").collect();

    game_info.last().unwrap().parse::<u128>().unwrap()
}

fn game_is_valid(raw_line: &str, cube_counts: &HashMap<&str, u8>) -> bool {
    let first_cut: Vec<&str> = raw_line.split(": ").collect();

    let draw_strs: Vec<&str> = first_cut.last().unwrap().split("; ").collect();

    for draw in draw_strs {
        let color_pulls: Vec<&str> = draw.split(", ").collect();
        for pull in color_pulls {
            let pull_info: Vec<&str> = pull.split(" ").collect();
            let count = pull_info.first().unwrap().parse::<u8>().unwrap();
            let color = pull_info.last().unwrap();

            if count > *cube_counts.get(color).unwrap() {
                return false;
            }
        }
    }

    true
}

fn get_game_power(raw_line: &str) -> u128 {
    let first_cut: Vec<&str> = raw_line.split(": ").collect();

    let draw_strs: Vec<&str> = first_cut.last().unwrap().split("; ").collect();

    let mut min_seen: HashMap<&str, u8> = HashMap::new();

    for draw in draw_strs {
        let color_pulls: Vec<&str> = draw.split(", ").collect();
        for pull in color_pulls {
            let pull_info: Vec<&str> = pull.split(" ").collect();
            let count = pull_info.first().unwrap().parse::<u8>().unwrap();
            let color = pull_info.last().unwrap();

            if min_seen.contains_key(color) {
                if count > *(min_seen.get(color).unwrap()) {
                    min_seen.insert(&color, count);
                }
            } else {
                min_seen.insert(&color, count);
            }
        }
    }

    min_seen
        .values()
        .fold(1 as u128, |acc, e| acc * (*e as u128))
}

fn day2_part1() {
    let buf_reader = utils::get_reader_for_day(2);
    let mut our_sum: u128 = 0;

    let cube_counts: HashMap<&str, u8> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    for _line in buf_reader.lines() {
        let line = _line.unwrap();
        let game_idx = get_game_index(&line);

        if game_is_valid(&line, &cube_counts) {
            our_sum += game_idx;
        }
    }

    println!("Day  2, part 1 = {}", our_sum);
}

fn day2_part2() {
    let buf_reader = utils::get_reader_for_day(2);
    let mut our_sum: u128 = 0;

    for _line in buf_reader.lines() {
        let line = _line.unwrap();

        our_sum += get_game_power(&line);
    }

    println!("Day  2, part 2 = {}", our_sum);
}

pub fn day2() {
    day2_part1();
    day2_part2();
}
