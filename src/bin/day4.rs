use std::collections::btree_set::Intersection;
use std::collections::BTreeSet;
use std::num::ParseIntError;
use std::str::FromStr;

const INPUT: &str = "./input/day4.txt";

#[derive(Debug, Default, Clone)]
struct ScratchCard {
    winning_numbers: BTreeSet<u32>,
    numbers: BTreeSet<u32>,
}

impl ScratchCard {
    fn count_matching_numbers(&self) -> usize {
        self.matching_numbers().count()
    }

    fn matching_numbers(&self) -> Intersection<u32> {
        self.winning_numbers.intersection(&self.numbers)
    }
}

impl FromStr for ScratchCard {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (winning_numbers, numbers) = s
            .trim()
            .split_once(':')
            .unwrap_or_default()
            .1
            .trim()
            .split_once('|')
            .unwrap_or_default();
        let winning_numbers = winning_numbers
            .split_whitespace()
            .map(u32::from_str)
            .collect::<Result<BTreeSet<u32>, _>>()?;
        let numbers = numbers
            .split_whitespace()
            .map(u32::from_str)
            .collect::<Result<BTreeSet<u32>, _>>()?;

        Ok(Self {
            winning_numbers,
            numbers,
        })
    }
}

fn part1(cards: &[ScratchCard]) -> u32 {
    cards
        .iter()
        .map(|card| (1_u32 << (card.count_matching_numbers() as u32)) >> 1)
        .sum()
}

fn part2(cards: &[ScratchCard]) -> usize {
    let mut counter = vec![1; cards.len()];

    for (idx, matches) in cards
        .iter()
        .map(ScratchCard::count_matching_numbers)
        .enumerate()
    {
        let current = counter[idx];
        for item in counter[idx + 1..idx + 1 + matches].iter_mut() {
            *item += current;
        }
    }

    counter.into_iter().sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT)?;
    let cards = input
        .lines()
        .map(ScratchCard::from_str)
        .collect::<Result<Vec<ScratchCard>, _>>()?;

    println!("The first answer is: {}", part1(&cards));
    println!("The second answer is: {}", part2(&cards));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11\n\
    ";

    #[test]
    fn test_part1() {
        let cards = EXAMPLE
            .lines()
            .map(ScratchCard::from_str)
            .collect::<Result<Vec<ScratchCard>, _>>()
            .unwrap();
        let actual = part1(&cards);
        let expected = 13;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2() {
        let cards = EXAMPLE
            .lines()
            .map(ScratchCard::from_str)
            .collect::<Result<Vec<ScratchCard>, _>>()
            .unwrap();
        let actual = part2(&cards);
        let expected = 30;

        assert_eq!(expected, actual);
    }
}
