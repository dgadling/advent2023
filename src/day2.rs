const DAY: u8 = 2;

pub fn main() {
    part1();
    part2();
}

use std::collections::HashMap;

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

fn part1() {
    let cube_counts: HashMap<&str, u8> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    let our_sum = crate::utils::lines(DAY).fold(0, |acc, l| {
        acc + if game_is_valid(&l, &cube_counts) {
            get_game_index(&l)
        } else {
            0
        }
    });

    done!(DAY, 1, our_sum);
}

fn part2() {
    let our_sum = crate::utils::lines(DAY).fold(0, |acc, l| acc + get_game_power(&l));

    done!(DAY, 2, our_sum);
}
