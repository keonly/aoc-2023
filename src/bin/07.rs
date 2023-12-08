use itertools::Itertools;
use phf::phf_map;

advent_of_code::solution!(7);

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPairs = 2,
    OnePair = 1,
    HighCard = 0,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
struct Hand(u8, u8, u8, u8, u8);

static CARD_SCORES_1: phf::Map<char, u8> = phf_map! {
    'A' => 13,
    'K' => 12,
    'Q' => 11,
    'J' => 10,
    'T' => 9,
    '9' => 8,
    '8' => 7,
    '7' => 6,
    '6' => 5,
    '5' => 4,
    '4' => 3,
    '3' => 2,
    '2' => 1,
};

fn parse_input(input: &str) -> Vec<(&str, u32)> {
    input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(" ").unwrap();
            (hand, bid.parse::<u32>().unwrap())
        })
        .collect()
}

fn score_hand_1(hand: &str) -> (HandType, Hand) {
    let values = if let [a, b, c, d, e] = hand.chars().collect::<Vec<char>>()[..] {
        Hand(
            *CARD_SCORES_1.get(&a).unwrap(),
            *CARD_SCORES_1.get(&b).unwrap(),
            *CARD_SCORES_1.get(&c).unwrap(),
            *CARD_SCORES_1.get(&d).unwrap(),
            *CARD_SCORES_1.get(&e).unwrap(),
        )
    } else {
        panic!("Input must be a five-letter string");
    };
    let mut counts: Vec<(char, usize)> = hand
        .chars()
        .counts()
        .iter()
        .map(|(k, v)| (*k, *v))
        .collect::<Vec<(char, usize)>>();
    counts.sort_by(|a, b| b.1.cmp(&a.1));
    match counts.len() {
        1 => (HandType::FiveOfAKind, values),
        2 => {
            if counts[0].1 == 4 {
                (HandType::FourOfAKind, values)
            } else {
                (HandType::FullHouse, values)
            }
        }
        3 => {
            if counts[0].1 == 3 {
                (HandType::ThreeOfAKind, values)
            } else {
                (HandType::TwoPairs, values)
            }
        }
        4 => (HandType::OnePair, values),
        5 => (HandType::HighCard, values),
        v => {
            panic!("Unexpected number of cards given: {}", v);
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let hand_bid_vec = parse_input(input);
    let mut sorted_by_score = hand_bid_vec
        .iter()
        .map(|(hand, bid)| (score_hand_1(hand), *bid))
        .collect::<Vec<((HandType, Hand), u32)>>();
    sorted_by_score.sort_by(|a, b| a.0.cmp(&b.0));
    sorted_by_score
        .iter()
        .enumerate()
        .fold(0, |acc, (i, (_, bid))| acc + (i as u32 + 1) * bid)
        .into()
}

static CARD_SCORES_2: phf::Map<char, u8> = phf_map! {
    'A' => 13,
    'K' => 12,
    'Q' => 11,
    'T' => 10,
    '9' => 9,
    '8' => 8,
    '7' => 7,
    '6' => 6,
    '5' => 5,
    '4' => 4,
    '3' => 3,
    '2' => 2,
    'J' => 1,
};

fn score_hand_2(hand: &str) -> (HandType, Hand) {
    let values = if let [a, b, c, d, e] = hand.chars().collect::<Vec<char>>()[..] {
        Hand(
            *CARD_SCORES_2.get(&a).unwrap(),
            *CARD_SCORES_2.get(&b).unwrap(),
            *CARD_SCORES_2.get(&c).unwrap(),
            *CARD_SCORES_2.get(&d).unwrap(),
            *CARD_SCORES_2.get(&e).unwrap(),
        )
    } else {
        panic!("Input must be a five-letter string");
    };
    let mut joker_counts: usize = 0;

    let mut counts: Vec<(char, usize)> = hand
        .chars()
        .counts()
        .iter()
        .filter_map(|(k, v)| {
            if *k == 'J' {
                joker_counts += v;
                None
            } else {
                Some((*k, *v))
            }
        })
        .collect::<Vec<(char, usize)>>();

    counts.sort_by(|a, b| b.1.cmp(&a.1));

    if joker_counts == 5 {
        return (HandType::FiveOfAKind, values);
    }

    match counts[0].1 + joker_counts {
        5 => (HandType::FiveOfAKind, values),
        4 => (HandType::FourOfAKind, values),
        3 => {
            if counts[1].1 == 2 {
                (HandType::FullHouse, values)
            } else {
                (HandType::ThreeOfAKind, values)
            }
        }
        2 => {
            if joker_counts == 0 && counts[1].1 == 2 {
                (HandType::TwoPairs, values)
            } else {
                (HandType::OnePair, values)
            }
        }
        1 => (HandType::HighCard, values),
        v => {
            panic!("Unexpected number of cards given: {}", v);
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let hand_bid_vec = parse_input(input);
    let mut sorted_by_score = hand_bid_vec
        .iter()
        .map(|(hand, bid)| (score_hand_2(hand), *bid))
        .collect::<Vec<((HandType, Hand), u32)>>();
    sorted_by_score.sort_by(|a, b| a.0.cmp(&b.0));
    sorted_by_score
        .iter()
        .enumerate()
        .fold(0, |acc, (i, (_, bid))| acc + (i as u32 + 1) * bid)
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
