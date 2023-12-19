use std::cmp::Ordering;
use std::{collections::HashMap, io::BufRead};

use crate::utils::get_reader_for_day;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandKind {
    High = 1,
    Pair,
    TwoPair,
    Three,
    House,
    Four,
    Five,
}

#[derive(Debug, Eq)]
struct Hand {
    cards: String,
    bid: u32,
    kind: HandKind,
}

impl Hand {
    pub fn new(line: String) -> Self {
        let parts: Vec<&str> = line.trim_end().split(" ").collect();
        let cards = parts.first().unwrap();
        let bid = parts.last().unwrap();

        Hand {
            cards: cards.to_string(),
            bid: bid.parse::<u32>().unwrap(),
            kind: Hand::get_hand_kind(cards),
        }
    }

    fn get_hand_kind(cards: &str) -> HandKind {
        /*
         Apparently we have to do this by:
           1. Generate the counts in a HashMap
           2. Turn the HashMap into a Vec of tuples of (count, card)
           3. Sort the Vec
        */
        let mut card_count: HashMap<char, u8> = HashMap::new();
        for card in cards.chars() {
            *card_count.entry(card).or_insert(0) += 1;
        }

        let mut count_by_card: Vec<(&u8, &char)> = card_count.iter().map(|(k, v)| (v, k)).collect();
        // Default sort is *ascending*, so largest at the end. Convenient!
        count_by_card.sort();

        let most_frequent = *count_by_card.pop().unwrap().0;
        if most_frequent == 5 {
            return HandKind::Five;
        }

        // Since all 5 cards weren't the same there *will* be a second most frequent
        let second_most_frequent = *count_by_card.pop().unwrap().0;

        // We can figure these out with just the most frequent card
        match most_frequent {
            4 => return HandKind::Four,
            3 if second_most_frequent == 2 => return HandKind::House,
            3 => return HandKind::Three,
            2 if second_most_frequent == 2 => return HandKind::TwoPair,
            2 => return HandKind::Pair,
            1 => return HandKind::High,
            0_u8 | 5_u8..=u8::MAX => panic!("Wrong number of cards entirely!"),
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let ord = self.kind.cmp(&other.kind);

        if ord != Ordering::Equal {
            return Some(ord);
        }

        let our_cards: Vec<char> = self.cards.chars().collect();
        let their_cards: Vec<char> = other.cards.chars().collect();
        let (mut our_card_strength, mut their_card_strength): (u32, u32);

        for i in 0..5 {
            our_card_strength = strength_of_card(our_cards.get(i).unwrap()).unwrap();
            their_card_strength = strength_of_card(their_cards.get(i).unwrap()).unwrap();

            if our_card_strength == their_card_strength {
                continue;
            }

            if our_card_strength > their_card_strength {
                return Some(Ordering::Greater);
            } else {
                return Some(Ordering::Less);
            }
        }

        Some(Ordering::Equal)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

const fn strength_of_card(c: &char) -> Option<u32> {
    let num = c.to_digit(10);

    if num.is_some() {
        return num;
    }

    match c {
        'T' => Some(10),
        'J' => Some(11),
        'Q' => Some(12),
        'K' => Some(13),
        'A' => Some(14),
        _ => None,
    }
}

fn get_hands() -> Vec<Hand> {
    let r = get_reader_for_day(7);

    r.lines().map(|l| Hand::new(l.unwrap())).collect()
}

pub fn day7() {
    day7_part1();
    // day7_part2();
}

fn day7_part1() {
    let mut hands = get_hands();
    hands.sort();

    let winnings: u32 = hands
        .iter()
        .enumerate()
        .map(|(idx, h)| h.bid * (idx + 1) as u32)
        .sum();

    println!("We win {}", winnings);
}
