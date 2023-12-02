use aoc2023::fetch_input;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let input = fetch_input(2).await?;
    println!("PART 1: {}", part1(&input));
    println!("PART 2: {}", part2(&input));

    Ok(())
}

// 12 red
// 13 green
// 14 blue

#[derive(Debug, Default)]
struct Rgb {
    red: usize,
    green: usize,
    blue: usize,
}

impl Rgb {
    fn legal(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }

    fn add_color(&mut self, color: &str, count: usize) {
        match color {
            "red" => self.red += count,
            "green" => self.green += count,
            "blue" => self.blue += count,
            _ => unreachable!(),
        }
    }

    fn count_max(&mut self, other: &Rgb) {
        if other.red > self.red {
            self.red = other.red
        }
        if other.green > self.green {
            self.green = other.green
        }
        if other.blue > self.blue {
            self.blue = other.blue
        }
    }
}

fn game_subsets(line: &str) -> (usize, Vec<&str>) {
    let mut split = line.split(':');
    let game = game_num(split.next().unwrap());
    let subsets = split.last().unwrap().split(';').collect::<Vec<_>>();
    (game, subsets)
}

fn game_num(game_str: &str) -> usize {
    game_str
        .chars()
        .skip(5)
        .collect::<String>()
        .parse::<usize>()
        .unwrap()
}

fn subset_colors(subset: &str) -> Rgb {
    let mut ret = Rgb::default();

    let colors = subset.trim().split(',');
    for color in colors {
        let mut split = color.trim().split(' ');
        let count = split.next().unwrap().parse::<usize>().unwrap();
        let color = split.next().unwrap();
        ret.add_color(color, count);
    }

    ret
}

fn part1(input: &str) -> usize {
    let mut valid_games: Vec<usize> = vec![];

    for game in input.lines() {
        let (game, subsets) = game_subsets(game);
        if subsets
            .iter()
            .map(|subset| subset_colors(subset).legal())
            .all(|legal| legal)
        {
            valid_games.push(game);
        }
    }

    valid_games.iter().sum()
}

fn part2(input: &str) -> usize {
    let mut ans = 0;

    for game in input.lines() {
        let mut rgb = Rgb::default();
        let (_, subsets) = game_subsets(game);
        for subset in subsets {
            rgb.count_max(&subset_colors(subset));
        }

        ans += rgb.red * rgb.green * rgb.blue;
    }
    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_game_num() {
        let input = "Game 1123: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let mut split = input.split(':');
        let game = game_num(split.next().unwrap());
        assert_eq!(game, 1123)
    }

    #[test]
    fn part1_sample() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(part1(input), 8);
    }

    #[test]
    fn part2_sample() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(part2(input), 2286);
    }
}
