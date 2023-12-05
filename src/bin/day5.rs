use advent_of_code_2023::range_extension::RangeExtension;
use rangemap::RangeSet;
use std::collections::BTreeMap;
use std::num::ParseIntError;
use std::ops::Range;
use std::str::FromStr;

const INPUT: &str = "./input/day5.txt";

type Id = i64;

#[derive(Debug, Default, Clone)]
struct Map {
    conversions: Vec<(Range<Id>, Id)>,
}

impl FromStr for Map {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut conversions = Vec::default();

        for line in s.lines().skip(1) {
            let mut iter = line.split_whitespace();
            let destination: Id = iter.next().unwrap_or_default().parse()?;
            let source: Id = iter.next().unwrap_or_default().parse()?;
            let range: Id = iter.next().unwrap_or_default().parse()?;

            let offset = destination - source;
            conversions.push((
                Range {
                    start: source,
                    end: source + range,
                },
                offset,
            ));
        }

        Ok(Self { conversions })
    }
}

fn parse_input(input: &str) -> (Vec<Id>, Vec<Map>) {
    let mut sections = input.split("\n\n");

    let seeds = sections
        .next()
        .expect("Bad format.")
        .split_whitespace()
        .filter_map(|substring| substring.parse().ok())
        .collect();

    let maps = sections
        .map(str::parse)
        .collect::<Result<Vec<Map>, _>>()
        .expect("Bad format.");
    (seeds, maps)
}

fn part1(seeds: &[Id], maps: &[Map]) -> Id {
    let mut numbers: BTreeMap<Id, Id> = seeds.iter().map(|&n| (n, n)).collect();

    for map in maps {
        for (range, offset) in &map.conversions {
            for (_, destination) in numbers.range_mut(range.clone()) {
                *destination += offset;
            }
        }

        numbers = numbers.into_values().map(|n| (n, n)).collect();
    }

    numbers.into_keys().min().unwrap_or_default()
}

fn part2(seeds: &[Id], maps: &[Map]) -> Id {
    let mut sources: RangeSet<Id> = seeds
        .chunks_exact(2)
        .map(|pair| pair[0]..pair[0] + pair[1])
        .collect();

    for map in maps {
        let mut destinations = RangeSet::default();
        for (range, offset) in &map.conversions {
            for overlapping in sources.overlapping(range) {
                let common = range.intersection(overlapping);
                destinations.insert(common.start + offset..common.end + offset);
            }
            sources.remove(range.clone());
        }

        sources.extend(destinations);
    }

    sources.iter().next().expect("No seeds.").start
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT)?;
    let (seeds, maps) = parse_input(&input);

    println!("The first answer is: {}", part1(&seeds, &maps));
    println!("The second answer is: {}", part2(&seeds, &maps));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        seeds: 79 14 55 13\n\
        \n\
        seed-to-soil map:\n\
        50 98 2\n\
        52 50 48\n\
        \n\
        soil-to-fertilizer map:\n\
        0 15 37\n\
        37 52 2\n\
        39 0 15\n\
        \n\
        fertilizer-to-water map:\n\
        49 53 8\n\
        0 11 42\n\
        42 0 7\n\
        57 7 4\n\
        \n\
        water-to-light map:\n\
        88 18 7\n\
        18 25 70\n\
        \n\
        light-to-temperature map:\n\
        45 77 23\n\
        81 45 19\n\
        68 64 13\n\
        \n\
        temperature-to-humidity map:\n\
        0 69 1\n\
        1 0 69\n\
        \n\
        humidity-to-location map:\n\
        60 56 37\n\
        56 93 4\n\
    ";

    #[test]
    fn test_part1() {
        let (seeds, maps) = parse_input(EXAMPLE);
        let actual = part1(&seeds, &maps);
        let expected = 35;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2() {
        let (seeds, maps) = parse_input(EXAMPLE);
        let actual = part2(&seeds, &maps);
        let expected = 46;

        assert_eq!(expected, actual);
    }
}
