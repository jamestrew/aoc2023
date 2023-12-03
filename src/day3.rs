use std::ops::Range;

use aoc2023::fetch_input;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let input = fetch_input(3).await?;
    println!("PART 1: {}", part1(&input));
    println!("PART 2: {}", part2(&input));

    Ok(())
}

// row, col_rng -> num
// symbol neighors (row, col)

#[derive(Debug, PartialEq)]
struct Pos(usize, usize); // row, col

#[derive(Debug, PartialEq)]
struct NumPos {
    row: usize,
    col_rng: Range<usize>,
    num: usize,
    counted: bool,
}

impl NumPos {
    fn new(row: usize, col_start: usize, nums: &[char]) -> Self {
        let num_str = nums.iter().collect::<String>();
        println!("{num_str}");
        Self {
            row,
            col_rng: col_start..col_start + num_str.len(),
            num: num_str.parse().unwrap(),
            counted: false,
        }
    }
}

fn get_num_pos(schematic: &[Vec<char>]) -> Vec<NumPos> {
    let mut nums = Vec::new();

    for (row_num, row) in schematic.iter().enumerate() {
        let mut num_start = None;
        for (col_num, ch) in row.iter().enumerate() {
            match (ch.is_numeric(), num_start) {
                (true, None) => num_start = Some(col_num),
                (false, Some(start)) => {
                    nums.push(NumPos::new(row_num, start, &row[start..col_num]));
                    num_start = None;
                }
                _ => {}
            }
        }
        if let Some(start) = num_start {
            nums.push(NumPos::new(row_num, start, &row[start..]));
        }
    }

    nums
}

fn symbol_pos(schematic: &[Vec<char>]) -> Vec<Vec<Pos>> {
    let mut symbols = Vec::new();

    let rows = schematic.len();
    let cols = schematic.first().map_or(0, Vec::len);

    for (row_num, row) in schematic.iter().enumerate() {
        for (col_num, &ch) in row.iter().enumerate() {
            if ch.is_numeric() || ch == '.' {
                continue;
            }
            let mut symbol_neighbors = Vec::new();
            for dx in -1..=1 {
                for dy in -1..=1 {
                    let new_row = row_num as isize + dx;
                    let new_col = col_num as isize + dy;

                    if new_row >= 0
                        && new_row < rows as isize
                        && new_col >= 0
                        && new_col < cols as isize
                    {
                        symbol_neighbors.push(Pos(new_row as usize, new_col as usize));
                    }
                }
            }
            symbols.push(symbol_neighbors)
        }
    }

    symbols
}

fn create_schematic(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn part1(input: &str) -> usize {
    let schematic = create_schematic(input);
    let mut nums = get_num_pos(&schematic);
    let symbols = symbol_pos(&schematic);

    let mut sum = 0;


    for symbol_neighbors in &symbols {
        for num in &mut nums {
            if num.counted {
                continue;
            }
            for sym_neighbor in symbol_neighbors {
                if sym_neighbor.0 == num.row
                    && num.col_rng.contains(&sym_neighbor.1)
                    && !num.counted
                {
                    sum += num.num;
                    num.counted = true;
                }
            }
        }
    }
    sum
}

fn part2(input: &str) -> usize {
    let _ = input;
    3
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            .trim();
        assert_eq!(part1(input), 4361);
    }

    #[test]
    fn part1_1() {
        let input = "
........
.24..4..
......*.
"
        .trim();
        assert_eq!(part1(input), 4);
    }

    #[test]
    fn part1_2() {
        let input = "
........
.24$-4..
......*.
"
        .trim();
        assert_eq!(part1(input), 28);
    }
    #[test]
    fn part1_3() {
        let input = "
11....11
..$..$..
11....11
"
        .trim();
        assert_eq!(part1(input), 44);
    }
    #[test]
    fn part1_4() {
        let input = "
$......$
.1....1.
.1....1.
$......$
"
        .trim();
        assert_eq!(part1(input), 4);
    }
    #[test]
    fn part1_5() {
        let input = "
$......$
.11..11.
.11..11.
$......$
"
        .trim();
        assert_eq!(part1(input), 44);
    }
    #[test]
    fn part1_6() {
        let input = "
$11
...
11$
...
"
        .trim();
        assert_eq!(part1(input), 22);
    }
    #[test]
    fn part1_7() {
        let input = "
$..
.11
.11
$..
..$
11.
11.
..$
"
        .trim();
        assert_eq!(part1(input), 44);
    }
    #[test]
    fn part1_8() {
        let input = "
11.$.
"
        .trim();
        assert_eq!(part1(input), 0);
    }

    #[test]
    fn part2_sample() {
        let input = "";
        assert_eq!(part2(input), 3);
    }
}
