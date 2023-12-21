const DAY: u8 = 8;

pub fn main() {
    part1();
    part2();
}

use std::collections::HashMap;

type Label = String;

#[derive(Debug)]
struct Node {
    left: Label,
    right: Label,
}

fn get_directions_and_graph() -> (String, HashMap<Label, Node>) {
    let mut directions = String::new();
    let mut graph: HashMap<Label, Node> = HashMap::new();

    for line in crate::utils::lines(DAY) {
        if line.is_empty() {
            continue;
        }

        if !line.contains("=") {
            directions = line.to_string();
            continue;
        }

        let first_cut: Vec<&str> = line.split("=").collect();
        let second_cut: Vec<&str> = first_cut.last().unwrap().split(",").collect();

        let left = second_cut
            .first()
            .unwrap()
            .replace("(", "")
            .trim()
            .to_string();
        let right = second_cut.last().unwrap().trim().replace(")", "");

        graph.insert(
            first_cut.first().unwrap().trim_end().to_string(),
            Node { left, right },
        );
    }
    (directions, graph)
}

fn part1() {
    let (directions, graph) = get_directions_and_graph();

    let steps: Vec<char> = directions.chars().collect();
    let mut step_idx = 0_usize;
    let max_steps = steps.len();
    let mut steps_taken = 0;

    let mut curr_loc: Label = "AAA".to_string();
    let mut current_node: &Node;

    while curr_loc != "ZZZ" {
        if step_idx == max_steps {
            step_idx = 0;
        }

        current_node = graph.get(&curr_loc).unwrap();
        if steps[step_idx] == 'L' {
            curr_loc = current_node.left.clone();
        } else {
            curr_loc = current_node.right.clone();
        }

        steps_taken += 1;
        step_idx += 1;
    }

    done!(DAY, 1, steps_taken);
}

/*
  Taken from https://github.com/TheAlgorithms/Rust/blob/master/src/math/lcm_of_n_numbers.rs {{{
*/
fn lcm(nums: &[u128]) -> u128 {
    if nums.len() == 1 {
        return nums[0];
    }

    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: u128, b: u128) -> u128 {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}
/*
  }}}
*/

fn part2() {
    /*
      This is inspired from the insights in this Medium post
      https://medium.com/@matthias.vombruch/how-to-really-solve-the-advent-of-code-2023-challenge-day-8-part-2-spoiler-646e0b7c440d
      The primary insight (that in retrospect is hinted at in the problem) is
      that each "ghost" will travel in a loop visiting a *Z square every n
      times where n is unique to each ghost.

      So we:
        1. Figure out how many ghosts we need and where they're going to start
        2. Have them follow the directions in lock step
        3. After each step we see how many are at an ending location.
        4. Any that are at a valid ending location have the number of steps taken recorded and stop walking
        5. Once all the ghosts have finished their loop we find the LCM of how many steps they took

      Since they're walking in cycles it will take LCM(min_steps_per_ghost) for them to synchronize.
    */
    let (directions, graph) = get_directions_and_graph();

    let steps: Vec<char> = directions.chars().collect();
    let mut step_idx = 0_usize;
    let max_steps = steps.len();
    let mut steps_taken = 0;

    let mut ghosts: Vec<&String> = graph.keys().filter(|k| k.ends_with("A")).collect();
    let mut ghost_steps: Vec<u128> = Vec::new();
    let mut finished_ghost_count: u16;

    while ghosts.is_empty() == false {
        if step_idx == max_steps {
            step_idx = 0;
        }

        if steps[step_idx] == 'L' {
            ghosts = ghosts
                .iter()
                .map(|g| &(graph.get(*g).unwrap().left))
                .collect();
        } else {
            ghosts = ghosts
                .iter()
                .map(|g| &(graph.get(*g).unwrap().right))
                .collect();
        }

        steps_taken += 1;
        step_idx += 1;

        finished_ghost_count = ghosts
            .iter()
            .fold(0, |acc, g| acc + (if g.ends_with("Z") { 1 } else { 0 }));

        if finished_ghost_count > 0 {
            ghosts.retain(|g| g.ends_with("Z") == false);

            for _ in 0..finished_ghost_count {
                ghost_steps.push(steps_taken);
            }
        }
    }

    done!(DAY, 2, lcm(&ghost_steps));
}
