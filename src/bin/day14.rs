use advent_of_code_2023::grid::Grid;
use std::collections::{HashMap, VecDeque};
use std::fmt::Formatter;
use std::str::FromStr;

const INPUT: &str = "./input/day14.txt";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Rock {
    Round,
    Cube,
    Empty,
}

impl TryFrom<char> for Rock {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'O' => Rock::Round,
            '#' => Rock::Cube,
            '.' => Rock::Empty,
            _ => Err(format!("Invalid input: {value}."))?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Platform(Grid<Rock>);

impl Platform {
    fn total_load(&self) -> usize {
        let height = self.0.height();
        self.0
            .columns()
            .flat_map(|column| {
                column.enumerate().filter_map(|(idx, &rock)| {
                    if rock == Rock::Round {
                        Some(height - idx)
                    } else {
                        None
                    }
                })
            })
            .sum()
    }

    fn tilt_north(&mut self) -> &mut Self {
        let width = self.0.width();

        for column in 0..width {
            // Keep track of the empties.
            let mut empties = VecDeque::new();

            for rock in self.0.column_mut(column) {
                match rock {
                    // If an empty spot is available, fill it with our round rock.
                    Rock::Round => {
                        if let Some(empty) = empties.pop_front() {
                            std::mem::swap(rock, empty);
                            empties.push_back(rock);
                        }
                    }
                    Rock::Cube => empties.clear(),
                    Rock::Empty => empties.push_back(rock),
                }
            }
        }

        self
    }

    fn tilt_south(&mut self) -> &mut Self {
        let width = self.0.width();

        for column in 0..width {
            // Keep track of the empties.
            let mut empties = VecDeque::new();

            for rock in self.0.column_mut(column).rev() {
                match rock {
                    // If an empty spot is available, fill it with our round rock.
                    Rock::Round => {
                        if let Some(empty) = empties.pop_front() {
                            std::mem::swap(rock, empty);
                            empties.push_back(rock);
                        }
                    }
                    Rock::Cube => empties.clear(),
                    Rock::Empty => empties.push_back(rock),
                }
            }
        }

        self
    }

    fn tilt_west(&mut self) -> &mut Self {
        let height = self.0.height();

        for row in 0..height {
            // Keep track of the empties.
            let mut empties = VecDeque::new();

            for rock in self.0.row_mut(row) {
                match rock {
                    // If an empty spot is available, fill it with our round rock.
                    Rock::Round => {
                        if let Some(empty) = empties.pop_front() {
                            std::mem::swap(rock, empty);
                            empties.push_back(rock);
                        }
                    }
                    Rock::Cube => empties.clear(),
                    Rock::Empty => empties.push_back(rock),
                }
            }
        }

        self
    }

    fn tilt_east(&mut self) -> &mut Self {
        let height = self.0.height();

        for row in 0..height {
            // Keep track of the empties.
            let mut empties = VecDeque::new();

            for rock in self.0.row_mut(row).rev() {
                match rock {
                    // If an empty spot is available, fill it with our round rock.
                    Rock::Round => {
                        if let Some(empty) = empties.pop_front() {
                            std::mem::swap(rock, empty);
                            empties.push_back(rock);
                        }
                    }
                    Rock::Cube => empties.clear(),
                    Rock::Empty => empties.push_back(rock),
                }
            }
        }

        self
    }

    fn spin_cycle(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }
}

impl FromStr for Platform {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height = s.lines().count();
        let width = s.lines().next().unwrap_or_default().len();
        let rocks = s
            .lines()
            .flat_map(|line| line.chars().map(Rock::try_from))
            .collect::<Result<Vec<Rock>, _>>()?;

        Ok(Self(Grid::new(height, width, rocks).unwrap()))
    }
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for row in self.0.rows() {
            for rock in row {
                s.push(match rock {
                    Rock::Round => 'O',
                    Rock::Cube => '#',
                    Rock::Empty => '.',
                });
            }

            s.push('\n');
        }

        write!(f, "{s}")
    }
}

fn part2(mut platform: Platform) -> usize {
    let mut h = HashMap::new();
    const CYCLES: usize = 1000000000;

    if let Some((current, previous)) =
        (0..=CYCLES).find_map(|cycle| match h.insert(platform.clone(), cycle) {
            Some(previous) => Some((cycle, previous)),
            None => {
                platform.spin_cycle();
                None
            }
        })
    {
        // If we have found a recurring pattern, we can skip ahead.
        let period_length = current - previous;
        let remainder = (CYCLES - current) % period_length;
        for _ in 0..remainder {
            platform.spin_cycle();
        }
    };

    platform.total_load()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT)?;
    let platform: Platform = input.parse()?;

    println!(
        "The first answer is: {}",
        platform.clone().tilt_north().total_load()
    );
    println!("The second answer is: {}", part2(platform));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        O....#....\n\
        O.OO#....#\n\
        .....##...\n\
        OO.#O....O\n\
        .O.....O#.\n\
        O.#..O.#.#\n\
        ..O..#O..O\n\
        .......O..\n\
        #....###..\n\
        #OO..#....\n\
    ";

    #[test]
    fn test_part1() {
        let mut platform = EXAMPLE.parse::<Platform>().unwrap();
        let actual = platform.tilt_north().total_load();
        let expected = 136;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2() {
        let platform = EXAMPLE.parse::<Platform>().unwrap();
        let actual = part2(platform);
        let expected = 64;

        assert_eq!(expected, actual);
    }
}
