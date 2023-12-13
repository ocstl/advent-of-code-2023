use advent_of_code_2023::grid::Grid;
use std::str::FromStr;

const INPUT: &str = "./input/day13.txt";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Location {
    Ash,
    Rock,
}

impl TryFrom<char> for Location {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Location::Ash,
            '#' => Location::Rock,
            _ => Err(format!("Invalid character: {}", value))?,
        })
    }
}

#[derive(Debug, Clone)]
struct Pattern(Grid<Location>);

impl Pattern {
    fn find_vertical_line_of_reflection(&self, smudges: usize) -> Option<usize> {
        let columns: Vec<Vec<&Location>> =
            self.0.columns().map(|column| column.collect()).collect();

        (1..self.0.width()).find(|&column| {
            columns[0..column]
                .iter()
                .rev()
                .zip(columns[column..].iter())
                .flat_map(|(left, right)| left.iter().zip(right.iter()).filter(|(l, r)| l != r))
                .count()
                == smudges
        })
    }

    fn find_horizontal_line_of_reflection(&self, smudges: usize) -> Option<usize> {
        let rows: Vec<Vec<&Location>> = self.0.rows().map(|row| row.collect()).collect();

        (1..self.0.height()).find(|&row| {
            rows[0..row]
                .iter()
                .rev()
                .zip(rows[row..].iter())
                .flat_map(|(left, right)| left.iter().zip(right.iter()).filter(|(l, r)| l != r))
                .count()
                == smudges
        })
    }
    fn summarize(&self, smudges: usize) -> usize {
        self.find_horizontal_line_of_reflection(smudges)
            .map_or(self.find_vertical_line_of_reflection(smudges), |row| {
                Some(row * 100)
            })
            .unwrap()
    }
}

impl FromStr for Pattern {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height = s.lines().count();
        let width = s.lines().next().unwrap_or_default().len();
        let locations = s
            .lines()
            .flat_map(|line| line.chars().map(Location::try_from))
            .collect::<Result<Vec<Location>, _>>()?;

        Ok(Self(Grid::new(height, width, locations).unwrap()))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT)?;
    let patterns = input
        .split("\n\n")
        .map(str::parse)
        .collect::<Result<Vec<Pattern>, _>>()?;

    println!(
        "The first answer is: {}",
        patterns
            .iter()
            .map(|pattern| pattern.summarize(0))
            .sum::<usize>()
    );
    println!(
        "The second answer is: {}",
        patterns
            .iter()
            .map(|pattern| pattern.summarize(1))
            .sum::<usize>()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "\
        #.##..##.\n\
        ..#.##.#.\n\
        ##......#\n\
        ##......#\n\
        ..#.##.#.\n\
        ..##..##.\n\
        #.#.##.#.\n\
    ";

    const EXAMPLE_2: &str = "\
        #...##..#\n\
        #....#..#\n\
        ..##..###\n\
        #####.##.\n\
        #####.##.\n\
        ..##..###\n\
        #....#..#\n\
    ";

    const EXAMPLE_3: &str = "\
        #.##..##.\n\
        ..#.##.#.\n\
        ##......#\n\
        ##......#\n\
        ..#.##.#.\n\
        ..##..##.\n\
        #.#.##.#.\n\
        \n\
        #...##..#\n\
        #....#..#\n\
        ..##..###\n\
        #####.##.\n\
        #####.##.\n\
        ..##..###\n\
        #....#..#\n\
        \n\
        .#.##.#.#\n\
        .##..##..\n\
        .#.##.#..\n\
        #......##\n\
        #......##\n\
        .#.##.#..\n\
        .##..##.#\n\
        \n\
        #..#....#\n\
        ###..##..\n\
        .##.#####\n\
        .##.#####\n\
        ###..##..\n\
        #..#....#\n\
        #..##...#\n\
    ";

    #[test]
    fn test_part1_ex1() {
        let actual = EXAMPLE_1.parse::<Pattern>().unwrap().summarize(0);
        let expected = 5;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part1_ex2() {
        let actual = EXAMPLE_2.parse::<Pattern>().unwrap().summarize(0);
        let expected = 400;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part1_ex3() {
        let actual = EXAMPLE_3
            .split("\n\n")
            .map(|pattern| pattern.parse::<Pattern>().unwrap().summarize(0))
            .sum::<usize>();
        let expected = 709; // 1400 part 2.

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2_ex1() {
        let actual = EXAMPLE_1.parse::<Pattern>().unwrap().summarize(1);
        let expected = 300;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2_ex2() {
        let actual = EXAMPLE_2.parse::<Pattern>().unwrap().summarize(1);
        let expected = 100;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2_ex3() {
        let actual = EXAMPLE_3
            .split("\n\n")
            .map(|pattern| pattern.parse::<Pattern>().unwrap().summarize(1))
            .sum::<usize>();
        let expected = 1400;

        assert_eq!(expected, actual);
    }
}
