use aoc2023::fetch_input;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let input = fetch_input(4).await?;
    println!("PART 1: {}", part1(&input));
    println!("PART 2: {}", part2(&input));

    Ok(())
}

#[derive(Debug)]
struct Card {
    // card_num: usize,
    winning_nums: Vec<usize>,
    available_nums: Vec<usize>,
}

impl Card {
    fn parse_cards(input: &str) -> Vec<Self> {
        let mut cards = Vec::new();
        for line in input.lines() {
            let mut split = line.split(':');
            let _ = split.next();

            let mut num_split = split.next().unwrap().split('|');

            let winning_nums_str = num_split.next().unwrap();
            let winning_nums = winning_nums_str
                .split_whitespace()
                .map(|num| num.trim().parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            let available_nums_str = num_split.next().unwrap();
            let available_nums = available_nums_str
                .split_whitespace()
                .map(|num| num.trim().parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            cards.push(Self {
                winning_nums,
                available_nums,
            });
        }

        cards
    }

    fn winning_numbers(&self) -> Vec<usize> {
        self.winning_nums
            .iter()
            .filter(|num| self.available_nums.contains(num))
            .cloned()
            .collect()
    }
}

fn part1(input: &str) -> usize {
    let cards = Card::parse_cards(input);
    cards
        .iter()
        .filter_map(|card| {
            let win_count = card.winning_numbers().len() as i32;
            if win_count > 0 {
                Some(2_i32.pow((win_count - 1_i32) as u32) as usize)
            } else {
                None
            }
        })
        .sum()
}

fn calc_card(cards: &[Card], start_idx: usize, copy_count: usize) -> usize {
    let mut count = 0;
    cards[start_idx..start_idx + copy_count]
        .iter()
        .enumerate()
        .for_each(|(idx, card)| {
            let wins = card.winning_numbers().len();
            count += 1;
            count += calc_card(cards, start_idx + idx + 1, wins)
        });
    count
}

fn part2(input: &str) -> usize {
    let cards = Card::parse_cards(input);
    calc_card(&cards, 0, cards.len())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = "
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"
        .trim();

        assert_eq!(part1(input), 13);
    }

    #[test]
    fn part2_sample() {
        let input = "
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"
        .trim();
        assert_eq!(part2(input), 30);
    }
}
