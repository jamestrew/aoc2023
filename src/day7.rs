use std::cmp::Ordering;
use std::collections::HashMap;

use aoc2023::fetch_input;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let input = fetch_input(7).await?;
    println!("PART 1: {}", part1(&input));
    println!("PART 2: {}", part2(&input));

    Ok(())
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'J' => Self::Jack,
            'T' => Self::Ten,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum HandType {
    HighCard(Card, Card, Card, Card, Card),
    OnePair(Card, Card, Card, Card),
    TwoPair(Card, Card, Card),
    ThreeOfKind(Card, Card, Card),
    FullHouse(Card, Card, Card),
    FourOfKind(Card, Card),
    FiveOfKind(Card),
}

impl HandType {
    fn new(cards: [Card; 5]) -> Self {
        let mut counts = HashMap::new();
        cards.into_iter().for_each(|card| {
            *counts.entry(card).or_insert(0) += 1;
        });

        let mut cards = counts.into_iter().collect::<Vec<_>>();
        cards.sort_by(|a, b| match b.1.cmp(&a.1) {
            Ordering::Equal => b.0.cmp(&a.0),
            other => other,
        });

        match cards.len() {
            1 => Self::FiveOfKind(cards[0].0),
            2 => Self::FourOfKind(cards[0].0, cards[1].0),
            3 => {
                if cards[0].1 == 2 {
                    Self::TwoPair(cards[0].0, cards[1].0, cards[2].0)
                } else if cards[1].1 == 2 {
                    Self::FullHouse(cards[0].0, cards[1].0, cards[2].0)
                } else {
                    Self::ThreeOfKind(cards[0].0, cards[1].0, cards[2].0)
                }
            }
            4 => Self::OnePair(cards[0].0, cards[1].0, cards[2].0, cards[3].0),
            5 => Self::HighCard(cards[0].0, cards[1].0, cards[2].0, cards[3].0, cards[4].0),
            _ => unreachable!(),
        }
    }

    fn type_order(&self) -> u8 {
        match self {
            HandType::HighCard(_, _, _, _, _) => 1,
            HandType::OnePair(_, _, _, _) => 2,
            HandType::TwoPair(_, _, _) => 3,
            HandType::ThreeOfKind(_, _, _) => 4,
            HandType::FullHouse(_, _, _) => 5,
            HandType::FourOfKind(_, _) => 6,
            HandType::FiveOfKind(_) => 7,
        }
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        let type_order = self.type_order().cmp(&other.type_order());

        if type_order == Ordering::Equal {
            match (self, other) {
                (
                    HandType::HighCard(a1, a2, a3, a4, a5),
                    HandType::HighCard(b1, b2, b3, b4, b5),
                ) => (a1, a2, a3, a4, a5).cmp(&(b1, b2, b3, b4, b5)),
                (HandType::OnePair(a1, a2, a3, a4), HandType::OnePair(b1, b2, b3, b4)) => {
                    (a1, a2, a3, a4).cmp(&(b1, b2, b3, b4))
                }
                (HandType::TwoPair(a1, a2, a3), HandType::TwoPair(b1, b2, b3)) => {
                    (a1, a2, a3).cmp(&(b1, b2, b3))
                }
                (HandType::ThreeOfKind(a1, a2, a3), HandType::ThreeOfKind(b1, b2, b3)) => {
                    (a1, a2, a3).cmp(&(b1, b2, b3))
                }
                (HandType::FullHouse(a1, a2, a3), HandType::FullHouse(b1, b2, b3)) => {
                    (a1, a2, a3).cmp(&(b1, b2, b3))
                }
                (HandType::FourOfKind(a1, a2), HandType::FourOfKind(b1, b2)) => {
                    (a1, a2).cmp(&(b1, b2))
                }

                (HandType::FiveOfKind(a), HandType::FiveOfKind(b)) => a.cmp(b),
                _ => unreachable!(),
            }
        } else {
            type_order
        }
    }
}

#[derive(Debug)]
struct Hand {
    cards: [Card; 5],
    bid: usize,
}

fn parse_hands(input: &str) -> Vec<Hand> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let cards = parts
                .next()
                .unwrap()
                .chars()
                .map(|ch| ch.into())
                .collect::<Vec<_>>();
            let bid = parts.next().unwrap().parse::<usize>().unwrap();
            Hand {
                cards: cards.try_into().unwrap(),
                bid,
            }
        })
        .collect()
}

fn part1(input: &str) -> usize {
    let mut hands = parse_hands(input)
        .iter()
        .map(|hand| (HandType::new(hand.cards), hand.bid))
        .collect::<Vec<_>>();

    hands.sort_by(|a, b| a.0.cmp(&b.0));
    hands
        .iter()
        .enumerate()
        .map(|(idx, (_, bid))| bid * (idx + 1))
        .sum()
}

fn part2(input: &str) -> usize {
    let _ = input;
    7
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = "
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"
        .trim();
        assert_eq!(part1(input), 6440);
    }

    #[test]
    fn part2_sample() {
        let input = "
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"
        .trim();
        assert_eq!(part2(input), 7);
    }
}
