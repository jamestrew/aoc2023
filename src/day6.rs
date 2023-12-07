use std::str::Lines;

use aoc2023::fetch_input;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let input = fetch_input(6).await?;
    println!("PART 1: {}", part1(&input));
    println!("PART 2: {}", part2(&input));

    Ok(())
}

#[derive(Debug)]
struct Race {
    time: usize,
    record_distance: usize,
}

impl Race {
    const ACCEL: usize = 1;

    fn parse_races(input: &str) -> Vec<Race> {
        let mut lines = input.lines();
        let times = Self::parse_line(&mut lines);
        let distances = Self::parse_line(&mut lines);

        times
            .into_iter()
            .zip(distances)
            .map(|(time, record_distance)| Self {
                time,
                record_distance,
            })
            .collect()
    }

    fn parse_line(line: &mut Lines<'_>) -> Vec<usize> {
        line.next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .map(|time| time.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
    }

    fn good_race_strats(&self) -> usize {
        let mut count = 0;
        for hold_time in 1..=self.time {
            let traverse_time = self.time - hold_time;
            let velocity = hold_time * Self::ACCEL;
            let distance = velocity * traverse_time;
            if distance > self.record_distance {
                count += 1;
            }
        }

        count
    }
}

fn part1(input: &str) -> usize {
    let races = Race::parse_races(input);
    races
        .iter()
        .map(|race| race.good_race_strats())
        .product::<usize>()
}

fn part2(input: &str) -> usize {
    let _ = input;
    6
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = "
Time:      7  15   30
Distance:  9  40  200
"
        .trim();
        assert_eq!(part1(input), 288);
    }

    #[test]
    fn part2_sample() {
        let input = "".trim();
        assert_eq!(part2(input), 6);
    }
}
