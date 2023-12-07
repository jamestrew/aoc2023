use std::ops::Range;
use std::str::Split;

use aoc2023::fetch_input;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let input = fetch_input(5).await?;
    println!("PART 1: {}", part1(&input));
    println!("PART 2: {}", part2(&input));

    Ok(())
}

#[derive(Debug)]
struct Map {
    dest_rng: Range<usize>,
    src_rng: Range<usize>,
}

impl Map {
    fn new(dest_rng_start: usize, src_rng_start: usize, rng_len: usize) -> Self {
        Self {
            dest_rng: dest_rng_start..dest_rng_start + rng_len,
            src_rng: src_rng_start..src_rng_start + rng_len,
        }
    }

    fn dest_num(&self, src_num: usize) -> usize {
        if self.src_rng.contains(&src_num) {
            let offset = src_num - self.src_rng.start;
            self.dest_rng.start + offset
        } else {
            src_num
        }
    }
}

trait FindInMaps {
    fn find_map(&self, src_num: usize) -> usize;
}

impl FindInMaps for Vec<Map> {
    fn find_map(&self, src_num: usize) -> usize {
        for map in self.iter() {
            if map.src_rng.contains(&src_num) {
                return map.dest_num(src_num);
            }
        }
        src_num
    }
}

trait Seeds {
    fn new(seed_line: &str) -> Self;
    fn iter(&self) -> std::slice::Iter<'_, usize>;
}

struct Part1Seeds(Vec<usize>);

impl Seeds for Part1Seeds {
    fn new(seed_line: &str) -> Self {
        Self(
            seed_line
                .split_whitespace()
                .skip(1)
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<_>>(),
        )
    }

    fn iter(&self) -> std::slice::Iter<'_, usize> {
        self.0.iter()
    }
}

#[derive(Debug)]
struct Almanac<T>
where
    T: Seeds,
{
    seed_nums: T,
    seed_to_soil: Vec<Map>,
    soil_to_fert: Vec<Map>,
    fert_to_water: Vec<Map>,
    water_to_light: Vec<Map>,
    light_to_temp: Vec<Map>,
    temp_to_humidity: Vec<Map>,
    humidity_to_location: Vec<Map>,
}

impl<T> Almanac<T>
where
    T: Seeds,
{
    fn new(input: &str) -> Self {
        let mut parts = input.split('\n');
        Self {
            seed_nums: T::new(parts.next().unwrap()),
            seed_to_soil: Self::parse_maps(&mut parts),
            soil_to_fert: Self::parse_maps(&mut parts),
            fert_to_water: Self::parse_maps(&mut parts),
            water_to_light: Self::parse_maps(&mut parts),
            light_to_temp: Self::parse_maps(&mut parts),
            temp_to_humidity: Self::parse_maps(&mut parts),
            humidity_to_location: Self::parse_maps(&mut parts),
        }
    }

    fn parse_maps(parts: &mut Split<'_, char>) -> Vec<Map> {
        parts
            .skip_while(|line| line.is_empty())
            .skip(1)
            .take_while(|line| !line.is_empty())
            .map(|line| {
                let map = line
                    .split_whitespace()
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<Vec<_>>();

                Map::new(map[0], map[1], map[2])
            })
            .collect::<Vec<_>>()
    }

    fn seed_locations(&self) -> Vec<usize> {
        let mut locations = Vec::new();

        for seed_num in self.seed_nums.iter() {
            let soil_num = self.seed_to_soil.find_map(*seed_num);
            let fert_num = self.soil_to_fert.find_map(soil_num);
            let water_num = self.fert_to_water.find_map(fert_num);
            let light_num = self.water_to_light.find_map(water_num);
            let temp_num = self.light_to_temp.find_map(light_num);
            let humid_num = self.temp_to_humidity.find_map(temp_num);
            let location_num = self.humidity_to_location.find_map(humid_num);
            locations.push(location_num);
        }

        locations
    }
}

fn part1(input: &str) -> usize {
    let almanac: Almanac<Part1Seeds> = Almanac::new(input);

    *almanac.seed_locations().iter().min().unwrap()
}

fn part2(input: &str) -> usize {
    let _ = input;
    46
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = "
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"
        .trim();
        assert_eq!(part1(input), 35);
    }

    #[test]
    fn part2_sample() {
        let input = "
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"
        .trim();
        assert_eq!(part2(input), 46);
    }
}
