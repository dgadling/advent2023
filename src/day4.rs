use crate::utils;
use std::collections::HashSet;
use std::io::BufRead;

#[derive(Debug)]
struct Card {
    num: u16,
    winners: HashSet<u16>,
    ours: Vec<u16>,
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
            .parse::<u16>()
            .unwrap();

        let cut_2: Vec<&str> = cut_1.last().unwrap().split("|").collect();

        let winners: HashSet<u16> = HashSet::from_iter(
            cut_2
                .first()
                .unwrap()
                .split_whitespace()
                .map(|n| n.parse::<u16>().unwrap()),
        );
        let ours: Vec<u16> = cut_2
            .last()
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse::<u16>().unwrap())
            .collect();

        Card {
            num: card_num,
            winners: winners,
            ours: ours,
        }
    }

    fn num_matches(&self) -> usize {
        self
            .ours
            .iter()
            .filter(|x| self.winners.contains(x))
            .collect::<Vec<&u16>>().len()
    }

    fn value(&self) -> u128 {
        let matches = self.num_matches();

        if matches > 0 {
            1 << (matches - 1)
        } else {
            0
        }
    }
}

fn get_cards() -> Vec<Card> {
    let reader = utils::get_reader_for_day(4);

    reader
        .lines()
        .map(|l| Card::from_input(l.unwrap()))
        .collect()
}

fn day4_part1() {
    let cards = get_cards();
    let our_sum: u128 = cards.iter().map(|c| c.value()).sum();

    println!("Day 4, part 1 = {}", our_sum);
}

pub fn day4() {
    day4_part1();
}
