#[derive(Debug)]
struct Race {
    max_time: i128,
    max_dist: i128,
}

impl Race {
    fn distance(&self, t: i128) -> i128 {
        /*
          Math time!
          To figure out how far you go for a given time holding the button, there's a simple equation:
              dist(time) = time * (race.max_time - time)
                 or
              dist(time) = time * race.max_time - time^2
        */
        t * (self.max_time - t)
    }

    fn min_to_win(&self) -> i128 {
        /*
         distance() being a quadratic equation we know it describes a parabola. Looking at
         it on say WolframAlpha we can see the middle of 1..race.max_time is
         going to give us peak distance.

         To figure out the time to press to get a given distance, you flip it around to get:
            time(dist) = (1/2) * (sqrt(race.max_time^2 - 4*dist) + race.max_time)
        */

        let c = self.max_time as f64;
        let y = self.max_dist as f64;
        let c2 = c.powi(2);

        let sq = (c2 - (4.0 * y)).sqrt();
        let p = c - sq;
        let x = p / 2.0;
        x.ceil() as i128
    }
}

/*
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
*/

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

/*
fn brute_ways_to_win(r: &Race) -> u128 {
    (1..r.max_time)
        .map(|t| t * (r.max_time - t))
        .filter(|d| *d > r.max_dist)
        .fold(0_u128, |acc, _| acc + 1)
}
*/

fn sneaky_ways_to_win(r: &Race) -> u128 {
    let mut min_to_win = r.min_to_win();
    if r.distance(min_to_win) == r.max_dist {
        // Our minimum time to win actually *ties*, so go up one
        min_to_win += 1;
    }

    let mid = (r.max_time / 2) as i128;

    let width = mid - min_to_win;
    let mut max_to_win = mid + width;
    if r.distance(max_to_win + 1) > r.max_dist {
        // Because button presses are integers, we have one more winner
        max_to_win += 1;
    }

    let ways_to_win = max_to_win - min_to_win + 1;
    ways_to_win as u128
}

fn day6_part1() {
    let races = get_races();

    let margin = races
        .iter()
        .map(|r| sneaky_ways_to_win(r))
        .reduce(|acc, e| acc * e)
        .unwrap();

    println!("Day  6, part 1 = {}", margin);
}

fn day6_part2() {
    println!(
        "Day  6, part 2 = {}",
        sneaky_ways_to_win(&Race {
            max_time: 56717999,
            max_dist: 334113513502430
        })
    );
}

pub fn day6() {
    day6_part1();
    day6_part2();
}
