const DAY: u8 = 4;

pub fn main() {
    part1();
    part2();
}

use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Card {
    num: usize,
    num_matches: usize,
}

impl Card {
    fn from_input(raw_line: String) -> Card {
        let cut_1: Vec<&str> = raw_line.split(':').collect();
        let card_num = cut_1
            .first()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let cut_2: Vec<&str> = cut_1.last().unwrap().split("|").collect();

        let winners: HashSet<_> = HashSet::from_iter(
            crate::utils::ints::<u16>(cut_2.first().unwrap().to_string()).into_iter(),
        );

        let ours: Vec<u16> = crate::utils::ints::<u16>(cut_2.last().unwrap().to_string());

        let num_matches = ours
            .iter()
            .filter(|x| winners.contains(x))
            .collect::<Vec<&u16>>()
            .len();

        Card {
            num: card_num,
            num_matches,
        }
    }

    fn value(&self) -> u128 {
        let matches = self.num_matches;

        if matches > 0 {
            1 << (matches - 1)
        } else {
            0
        }
    }
}

fn get_cards() -> Vec<Card> {
    crate::utils::lines(DAY)
        .map(|l| Card::from_input(l))
        .collect()
}

fn part1() {
    let cards = get_cards();
    let our_sum: u128 = cards.iter().map(|c| c.value()).sum();

    done!(DAY, 1, our_sum);
}

fn part2() {
    let cards = get_cards();

    // card number and how many copies we have
    let mut num_copies: HashMap<usize, usize> = HashMap::new();
    let mut copies_to_add;

    for curr_card in cards.iter() {
        // Put the original in the stack. There may have already been some.
        *num_copies.entry(curr_card.num).or_insert(0) += 1;
        copies_to_add = num_copies.get(&curr_card.num).cloned().unwrap();

        if curr_card.num_matches == 0 {
            // No matches, so we don't get any additional copies
            continue;
        }

        // We have at least one copy of the current card, but possibly more.
        // To avoid nested loops, just add the current number of copies as the
        // number of copies for each of the next cards.
        for offset in 1..=curr_card.num_matches {
            *num_copies.entry(curr_card.num + offset).or_insert(0) += copies_to_add;
        }
    }

    done!(DAY, 2, num_copies.values().sum::<usize>());
}
