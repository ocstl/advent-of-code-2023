use advent_of_code_2023::grid::{Direction, Grid, Position};
use std::collections::BTreeSet;
use std::str::FromStr;

const INPUT: &str = "./input/day23.txt";
const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Path,
    Forest,
    UpSlope,
    DownSlope,
    LeftSlope,
    RightSlope,
}

impl TryFrom<char> for Tile {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Tile::Path,
            '#' => Tile::Forest,
            '^' => Tile::UpSlope,
            'v' => Tile::DownSlope,
            '<' => Tile::LeftSlope,
            '>' => Tile::RightSlope,
            _ => Err(format!("Invalid character: {value}"))?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct HikingTrail(Grid<Tile>);

impl HikingTrail {
    fn starting_point(&self) -> Position {
        self.0
            .row(0)
            .enumerate()
            .find_map(|(col, &tile)| {
                if tile == Tile::Path {
                    Some(Position::new(col, 0))
                } else {
                    None
                }
            })
            .unwrap()
    }

    fn ending_point(&self) -> Position {
        let last_row = self.0.height() - 1;
        self.0
            .row(last_row)
            .enumerate()
            .find_map(|(col, &tile)| {
                if tile == Tile::Path {
                    Some(Position::new(col, last_row))
                } else {
                    None
                }
            })
            .unwrap()
    }

    fn longest_slippery_hike(&self, starting_point: Position, ending_point: Position) -> usize {
        let mut longest_path = 0;
        let mut possible_paths = vec![(starting_point, BTreeSet::from([starting_point]))];

        while let Some((mut current, mut path)) = possible_paths.pop() {
            loop {
                if current == ending_point {
                    longest_path = longest_path.max(path.len() - 1);
                    break;
                }

                match self.0.get(current) {
                    None | Some(Tile::Forest) => break,
                    Some(Tile::UpSlope) => {
                        if let Some(p) = current + Direction::Up {
                            current = p;
                            if !path.insert(current) {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    Some(Tile::DownSlope) => {
                        if let Some(p) = current + Direction::Down {
                            current = p;
                            if !path.insert(current) {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    Some(Tile::LeftSlope) => {
                        if let Some(p) = current + Direction::Left {
                            current = p;
                            if !path.insert(current) {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    Some(Tile::RightSlope) => {
                        if let Some(p) = current + Direction::Right {
                            current = p;
                            if !path.insert(current) {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    Some(Tile::Path) => {
                        let mut possibles: Vec<Position> = DIRECTIONS
                            .into_iter()
                            .filter_map(|d| {
                                (current + d).filter(|p| {
                                    !path.contains(p)
                                        && !matches!(self.0.get(*p), None | Some(Tile::Forest))
                                })
                            })
                            .collect();

                        if let Some(p) = possibles.pop() {
                            // Create alternative paths if necessary.
                            for other in possibles {
                                let mut split_path = path.clone();
                                split_path.insert(other);
                                possible_paths.push((other, split_path));
                            }

                            current = p;
                            path.insert(current);
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        longest_path
    }

    fn longest_hike(&self, starting_point: Position, ending_point: Position) -> usize {
        let mut longest_path = 0;
        let mut possible_paths = vec![(starting_point, BTreeSet::from([starting_point]))];

        while let Some((mut current, mut path)) = possible_paths.pop() {
            loop {
                if current == ending_point {
                    longest_path = longest_path.max(path.len() - 1);
                    break;
                } else if matches!(self.0.get(current), None | Some(Tile::Forest)) {
                    break;
                }

                let mut possibles: Vec<Position> = DIRECTIONS
                    .into_iter()
                    .filter_map(|d| {
                        (current + d).filter(|p| {
                            !path.contains(p)
                                && !matches!(self.0.get(*p), None | Some(Tile::Forest))
                        })
                    })
                    .collect();

                if let Some(p) = possibles.pop() {
                    // Create alternative paths if necessary.
                    for other in possibles {
                        let mut split_path = path.clone();
                        split_path.insert(other);
                        possible_paths.push((other, split_path));
                    }

                    current = p;
                    path.insert(current);
                } else {
                    break;
                }
            }
        }

        longest_path
    }
}

impl FromStr for HikingTrail {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height = s.lines().count();
        let width = s.lines().next().unwrap().len();
        let tiles = s
            .lines()
            .flat_map(|line| line.chars().map(Tile::try_from))
            .collect::<Result<Vec<Tile>, _>>()?;

        Ok(Self(Grid::new(height, width, tiles).unwrap()))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT)?;
    let trail: HikingTrail = input.parse()?;

    println!(
        "The first answer is: {}",
        trail.longest_slippery_hike(trail.starting_point(), trail.ending_point())
    );
    println!(
        "The second answer is: {}",
        trail.longest_hike(trail.starting_point(), trail.ending_point())
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        #.#####################\n\
        #.......#########...###\n\
        #######.#########.#.###\n\
        ###.....#.>.>.###.#.###\n\
        ###v#####.#v#.###.#.###\n\
        ###.>...#.#.#.....#...#\n\
        ###v###.#.#.#########.#\n\
        ###...#.#.#.......#...#\n\
        #####.#.#.#######.#.###\n\
        #.....#.#.#.......#...#\n\
        #.#####.#.#.#########v#\n\
        #.#...#...#...###...>.#\n\
        #.#.#v#######v###.###v#\n\
        #...#.>.#...>.>.#.###.#\n\
        #####v#.#.###v#.#.###.#\n\
        #.....#...#...#.#.#...#\n\
        #.#########.###.#.#.###\n\
        #...###...#...#...#.###\n\
        ###.###.#.###v#####v###\n\
        #...#...#.#.>.>.#.>.###\n\
        #.###.###.#.###.#.#v###\n\
        #.....###...###...#...#\n\
        #####################.#\n\
    ";

    #[test]
    fn test_part1() {
        let trail: HikingTrail = EXAMPLE.parse().unwrap();
        let actual = trail.longest_slippery_hike(trail.starting_point(), trail.ending_point());
        let expected = 94;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2() {
        let trail: HikingTrail = EXAMPLE.parse().unwrap();
        let actual = trail.longest_hike(trail.starting_point(), trail.ending_point());
        let expected = 154;

        assert_eq!(expected, actual);
    }
}
