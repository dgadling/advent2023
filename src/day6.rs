#[derive(Debug)]
struct Race {
    max_time: u128,
    max_dist: u128,
}

fn get_test_races() -> Vec<Race> {
    vec![
        Race {
            max_time: 7,
            max_dist: 9,
        },
        Race {
            max_time: 15,
            max_dist: 40,
        },
        Race {
            max_time: 30,
            max_dist: 200,
        },
    ]
}

fn get_races() -> Vec<Race> {
    vec![
        Race {
            max_time: 56,
            max_dist: 334,
        },
        Race {
            max_time: 71,
            max_dist: 1135,
        },
        Race {
            max_time: 79,
            max_dist: 1350,
        },
        Race {
            max_time: 99,
            max_dist: 2430,
        },
    ]
}

fn brute_ways_to_win(r: &Race) -> u128 {
    (1..r.max_time)
        .map(|t| t * (r.max_time - t))
        .filter(|d| *d > r.max_dist)
        .fold(0_u128, |acc, _| acc + 1)
}

fn day6_part1() {
    let races = get_races();

    let margin = races
        .iter()
        .map(|r| brute_ways_to_win(r))
        .reduce(|acc, e| acc * e)
        .unwrap();

    println!("Margin = {}", margin);
}

fn day6_part2() {
    println!(
        "Ways to win = {}",
        brute_ways_to_win(&Race {
            max_time: 56717999,
            max_dist: 334113513502430
        })
    );
}

pub fn day6() {
    day6_part1();
    day6_part2();
}
