use advent_of_code_2023::grid::{Grid, Position};
use std::collections::{HashMap, HashSet};

const INPUT: &str = "./input/day3.txt";
const EMPTY: u8 = b'.';
const GEAR: u8 = b'*';

type Schematic = Grid<u8>;

fn part1(schematic: &Schematic) -> u32 {
    let mut result = 0;

    for (idy, row) in schematic.rows().enumerate() {
        let mut accumulator = 0;
        let mut to_add = false;

        for (idx, &byte) in row.enumerate() {
            // If we have a digit, we're still dealing with a number.
            // Otherwise, include the number if required (`to_add` flag), then reset.
            if byte.is_ascii_digit() {
                accumulator = accumulator * 10 + u32::from(byte - b'0');

                // Check for a symbol and set the flag for inclusion.
                if !to_add {
                    to_add = Position::new(idx, idy).extended_neighbours().any(|p| {
                        schematic
                            .get(p)
                            .map(|&v| !v.is_ascii_digit() && v != EMPTY)
                            .unwrap_or_default()
                    });
                }
            } else {
                if to_add {
                    result += accumulator;
                }
                accumulator = 0;
                to_add = false;
            }
        }

        // Make sure to check at the end of the line as well.
        if to_add {
            result += accumulator;
        }
    }

    result
}

fn part2(schematic: &Schematic) -> u32 {
    let mut gear_locations: HashMap<Position, Vec<u32>> = HashMap::new();

    for (idy, row) in schematic.rows().enumerate() {
        let mut accumulator = 0;
        let mut gear_positions = HashSet::new();

        for (idx, &byte) in row.enumerate() {
            // If we have a digit, we're still dealing with a number.
            // Otherwise, check whether we're dealing with a potential gear, then note its position.
            if byte.is_ascii_digit() {
                accumulator = accumulator * 10 + u32::from(byte - b'0');
                gear_positions.extend(
                    Position::new(idx, idy)
                        .extended_neighbours()
                        .filter(|&p| schematic.get(p).map(|&v| v == GEAR).unwrap_or_default()),
                );
            } else {
                gear_positions
                    .drain()
                    .for_each(|p| gear_locations.entry(p).or_default().push(accumulator));
                accumulator = 0;
            }
        }

        // Make sure to check at the end of the line as well.
        gear_positions
            .drain()
            .for_each(|p| gear_locations.entry(p).or_default().push(accumulator));
    }

    // Only use "real" gears, i.e. those having two numbers associated with them.
    gear_locations
        .into_iter()
        .filter_map(|(_, numbers)| {
            if numbers.len() == 2 {
                Some(numbers[0] * numbers[1])
            } else {
                None
            }
        })
        .sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT)?;
    let height = input.lines().count();
    let width = input.lines().next().unwrap_or_default().len();
    let schematic = Grid::new(height, width, input.lines().flat_map(str::bytes).collect()).unwrap();

    println!("The first answer is: {}", part1(&schematic));
    println!("The second answer is: {}", part2(&schematic));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SCHEMATIC: &str = "\
            467..114..\n\
            ...*......\n\
            ..35..633.\n\
            ......#...\n\
            617*......\n\
            .....+.58.\n\
            ..592.....\n\
            ......755.\n\
            ...$.*....\n\
            .664.598..";

    #[test]
    fn test_part1() {
        let schematic =
            Grid::new(10, 10, SCHEMATIC.lines().flat_map(str::bytes).collect()).unwrap();
        let actual = part1(&schematic);
        let expected = 4361;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2() {
        let schematic =
            Grid::new(10, 10, SCHEMATIC.lines().flat_map(str::bytes).collect()).unwrap();
        let actual = part2(&schematic);
        let expected = 467835;

        assert_eq!(expected, actual);
    }
}
