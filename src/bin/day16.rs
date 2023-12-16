use advent_of_code_2023::grid::{Direction, Grid, Position};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

const INPUT: &str = "./input/day16.txt";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    UpRightMirror,
    DownRightMirror,
    VerticalSplitter,
    HorizontalSplitter,
}

impl TryFrom<char> for Tile {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Tile::Empty,
            '/' => Tile::UpRightMirror,
            '\\' => Tile::DownRightMirror,
            '|' => Tile::VerticalSplitter,
            '-' => Tile::HorizontalSplitter,
            _ => Err(format!("Invalid character: {}", value))?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Contraption(Grid<Tile>);

impl Contraption {
    fn energize(&self, initial_position: Position, initial_direction: Direction) -> usize {
        // Keep track of the energized tiles. By keeping track of the direction of the ray, we
        // can avoid infinite loops.
        let mut energized_tiles: HashMap<Position, HashSet<Direction>> = HashMap::new();
        let mut rays = vec![(initial_position, initial_direction)];

        while let Some((position, mut direction)) = rays.pop() {
            if let Some(tile) = self.0.get(position) {
                if !energized_tiles
                    .entry(position)
                    .or_default()
                    .insert(direction)
                {
                    continue;
                }

                // Modify the direction if encountering mirrors. For the splitters, we manually
                // push a second direction.
                match (tile, direction) {
                    (Tile::VerticalSplitter, Direction::Right | Direction::Left) => {
                        if let Some(p) = position + Direction::Up {
                            rays.push((p, Direction::Up));
                        }
                        direction = Direction::Down;
                    }
                    (Tile::HorizontalSplitter, Direction::Up | Direction::Down) => {
                        if let Some(p) = position + Direction::Left {
                            rays.push((p, Direction::Left));
                        }
                        direction = Direction::Right;
                    }
                    (Tile::UpRightMirror, Direction::Up) => direction = Direction::Right,
                    (Tile::UpRightMirror, Direction::Down) => direction = Direction::Left,
                    (Tile::UpRightMirror, Direction::Right) => direction = Direction::Up,
                    (Tile::UpRightMirror, Direction::Left) => direction = Direction::Down,
                    (Tile::DownRightMirror, Direction::Up) => direction = Direction::Left,
                    (Tile::DownRightMirror, Direction::Down) => direction = Direction::Right,
                    (Tile::DownRightMirror, Direction::Right) => direction = Direction::Down,
                    (Tile::DownRightMirror, Direction::Left) => direction = Direction::Up,
                    _ => (),
                }

                if let Some(p) = position + direction {
                    rays.push((p, direction));
                }
            }
        }

        energized_tiles.len()
    }
}

impl FromStr for Contraption {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height = s.lines().count();
        let width = s.lines().next().unwrap_or_default().trim().len();
        let tiles = s
            .lines()
            .flat_map(|line| line.chars().map(Tile::try_from))
            .collect::<Result<Vec<Tile>, _>>()?;

        Ok(Self(Grid::new(height, width, tiles).unwrap()))
    }
}

fn part1(contraption: &Contraption) -> usize {
    contraption.energize(Position::default(), Direction::Right)
}

fn part2(contraption: &Contraption) -> usize {
    let height = contraption.0.height();
    let width = contraption.0.width();

    let top_row = (0..width).map(|col| (Position::new(col, 0), Direction::Down));
    let bottom_row = (0..width).map(|col| (Position::new(col, height - 1), Direction::Up));
    let left_column = (0..height).map(|row| (Position::new(0, row), Direction::Right));
    let right_column = (0..height).map(|row| (Position::new(width - 1, row), Direction::Left));

    top_row
        .chain(bottom_row)
        .chain(left_column)
        .chain(right_column)
        .map(|(p, d)| contraption.energize(p, d))
        .max()
        .unwrap_or_default()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT)?;
    let contraption: Contraption = input.parse()?;

    println!("The first answer is: {}", part1(&contraption));
    println!("The second answer is: {}", part2(&contraption));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        .|...\\....\n\
        |.-.\\.....\n\
        .....|-...\n\
        ........|.\n\
        ..........\n\
        .........\\\n\
        ..../.\\\\..\n\
        .-.-/..|..\n\
        .|....-|.\\\n\
        ..//.|....\n\
    ";

    #[test]
    fn test_part1() {
        let contraption = EXAMPLE.parse::<Contraption>().unwrap();
        let actual = part1(&contraption);
        let expected = 46;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2() {
        let contraption = EXAMPLE.parse::<Contraption>().unwrap();
        let actual = part2(&contraption);
        let expected = 51;

        assert_eq!(expected, actual);
    }
}
