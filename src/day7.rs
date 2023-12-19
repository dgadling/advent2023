use std::cmp::Ordering;
use std::{collections::HashMap, io::BufRead};

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
    jokers: bool,
}

impl Hand {
    pub fn new(line: String) -> Self {
        Self::_new(line, false)
    }

    pub fn new_with_jokers(line: String) -> Self {
        Self::_new(line, true)
    }

    fn _new(line: String, jokers: bool) -> Self {
        let parts: Vec<&str> = line.trim_end().split(" ").collect();
        let cards = parts.first().unwrap();
        let bid = parts.last().unwrap();

        Hand {
            cards: cards.to_string(),
            bid: bid.parse::<u32>().unwrap(),
            kind: Hand::get_hand_kind(cards, jokers),
            jokers,
        }
    }

    fn get_hand_kind(cards: &str, jokers: bool) -> HandKind {
        /*
         No native collections.Counter like in Python, so:
           1. Generate the counts in a HashMap
           2. Turn the HashMap into a Vec of tuples of (count, card)
           3. Sort the Vec
        */
        let mut card_count: HashMap<char, u8> = HashMap::new();
        for card in cards.chars() {
            *card_count.entry(card).or_insert(0) += 1;
        }

        let mut count_by_card: Vec<(u8, &char)> = card_count.iter().map(|(k, v)| (*v, k)).collect();

        let num_jokers = *card_count.get(&'J').unwrap_or(&0_u8);

        // Default sort is *ascending*, so largest at the end. Convenient!
        count_by_card.sort();

        if count_by_card.first().unwrap().0 == 5 {
            return HandKind::Five;
        }

        let most_frequent = count_by_card.pop().unwrap();
        let second_most_frequent = count_by_card.pop().unwrap();
        let jokes_first = most_frequent.1 == &'J';
        let jokes_second = second_most_frequent.1 == &'J';

        if !jokers {
            match most_frequent.0 {
                4 => return HandKind::Four,
                3 if second_most_frequent.0 == 2 => return HandKind::House,
                3 => return HandKind::Three,
                2 if second_most_frequent.0 == 2 => return HandKind::TwoPair,
                2 => return HandKind::Pair,
                1 => return HandKind::High,
                0_u8 | 5_u8..=u8::MAX => panic!("Wrong number of cards entirely!"),
            }
        } else {
            match most_frequent.0 {
                4 if jokes_first => return HandKind::Five,
                4 if num_jokers == 1 => return HandKind::Five,
                4 => return HandKind::Four,

                3 if jokes_first && second_most_frequent.0 == 2 => return HandKind::Five,
                3 if num_jokers == 2 => return HandKind::Five,
                3 if jokes_first => return HandKind::Four,
                3 if num_jokers == 1 => return HandKind::Four,
                3 if second_most_frequent.0 == 2 => return HandKind::House,
                3 => return HandKind::Three,

                2 if jokes_first && second_most_frequent.0 == 2 => return HandKind::Four,
                2 if num_jokers == 2 && jokes_second => return HandKind::Four,
                2 if second_most_frequent.0 == 2 && num_jokers == 1 => return HandKind::House,
                2 if jokes_first && second_most_frequent.0 == 1 => return HandKind::Three,
                2 if num_jokers == 1 => return HandKind::Three,
                2 if second_most_frequent.0 == 2 => return HandKind::TwoPair,
                2 => return HandKind::Pair,

                1 if num_jokers == 1 => return HandKind::Pair,
                1 => return HandKind::High,

                0_u8 | 5_u8..=u8::MAX => panic!("Wrong number of cards entirely!"),
            }
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

        assert!(
            self.jokers == other.jokers,
            "Can't compare a hands with different joker states!"
        );

        let all_jokes = self.jokers;

        for i in 0..5 {
            our_card_strength = strength_of_card(our_cards.get(i).unwrap(), all_jokes).unwrap();
            their_card_strength = strength_of_card(their_cards.get(i).unwrap(), all_jokes).unwrap();

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

const fn strength_of_card(c: &char, jokers: bool) -> Option<u32> {
    let num = c.to_digit(10);

    if num.is_some() {
        return num;
    }

    match c {
        'T' => Some(10),
        'J' if jokers => Some(1),
        'J' => Some(11),
        'Q' => Some(12),
        'K' => Some(13),
        'A' => Some(14),
        _ => None,
    }
}

pub fn day7() {
    day7_part1();
    day7_part2();
}

fn day7_part1() {
    let r = crate::utils::get_reader_for_day(7);

    let mut hands: Vec<Hand> = r.lines().map(|l| Hand::new(l.unwrap())).collect();
    hands.sort();

    let winnings: u32 = hands
        .iter()
        .enumerate()
        .map(|(idx, h)| h.bid * (idx + 1) as u32)
        .sum();

    println!("We win {}", winnings);
}

fn day7_part2() {
    let r = crate::utils::get_reader_for_day(7);

    let mut hands: Vec<Hand> = r
        .lines()
        .map(|l| Hand::new_with_jokers(l.unwrap()))
        .collect();
    hands.sort();

    let winnings: u32 = hands
        .iter()
        .enumerate()
        .map(|(idx, h)| h.bid * (idx + 1) as u32)
        .sum();

    println!("We win {}", winnings);
}
