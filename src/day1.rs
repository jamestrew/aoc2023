use aoc2023::fetch_input;

#[tokio::main]
async fn main() {
    fetch_input(1).await.unwrap();
    let input = std::fs::read_to_string("data/day1").unwrap();
    println!("PART 1: {}", part1(&input));
    println!("PART 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let v = line
                .chars()
                .filter(|char| char.is_numeric())
                .collect::<Vec<_>>();

            let first = v.first().unwrap();
            let last = v.last().unwrap();
            let num = format!("{first}{last}");
            num.parse::<usize>().unwrap()
        })
        .sum()
}

fn part2(input: &str) -> usize {
    todo!("nah")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(part1(input), 142);
    }

    #[test]
    fn part2_sample() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        assert_eq!(part2(input), 281);
    }
}
