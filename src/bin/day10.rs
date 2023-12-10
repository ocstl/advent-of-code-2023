use advent_of_code_2023::grid::{Direction, Grid, Position};
use std::str::FromStr;

const INPUT: &str = "./input/day10.txt";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    VerticalPipe,
    HorizontalPipe,
    UpRightBend,
    UpLeftBend,
    DownLeftBend,
    DownRightBend,
    Ground,
    StartingPosition,
}

impl Tile {
    fn directions(self) -> &'static [Direction] {
        use Direction::*;
        use Tile::*;

        match self {
            VerticalPipe => &[Up, Down],
            HorizontalPipe => &[Left, Right],
            UpRightBend => &[Up, Right],
            UpLeftBend => &[Up, Left],
            DownLeftBend => &[Down, Left],
            DownRightBend => &[Down, Right],
            Ground => &[],
            StartingPosition => &[Up, Down, Left, Right],
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '|' => Tile::VerticalPipe,
            '-' => Tile::HorizontalPipe,
            'L' => Tile::UpRightBend,
            'J' => Tile::UpLeftBend,
            '7' => Tile::DownLeftBend,
            'F' => Tile::DownRightBend,
            '.' => Tile::Ground,
            'S' => Tile::StartingPosition,
            _ => Err(value)?,
        })
    }
}

#[derive(Debug, Clone)]
struct Map {
    grid: Grid<Tile>,
    starting_position: Position,
}

impl Map {
    fn connects_to(&self, position: Position) -> impl Iterator<Item = Position> {
        self.grid
            .get(position)
            .unwrap_or(&Tile::Ground)
            .directions()
            .iter()
            .filter_map(move |&d| position + d)
    }

    fn generate_path(&self) -> Vec<Position> {
        use Direction::*;
        use Tile::*;

        // Since the pipe is one continuous loop, we need only generate one path (and there should
        // be at most one).
        let mut path = vec![self.starting_position];
        let first_step = StartingPosition
            .directions()
            .iter()
            .find(|&&d| {
                let next_tile = (self.starting_position + d)
                    .and_then(|p| self.grid.get(p))
                    .unwrap_or(&Ground);
                match d {
                    Up => [VerticalPipe, DownRightBend, DownLeftBend].contains(next_tile),
                    Down => [VerticalPipe, UpRightBend, UpLeftBend].contains(next_tile),
                    Left => [HorizontalPipe, UpLeftBend, DownLeftBend].contains(next_tile),
                    Right => [HorizontalPipe, UpRightBend, DownRightBend].contains(next_tile),
                    _ => unreachable!(),
                }
            })
            .expect("Starting position does not connect to any other tile.");

        let mut next_position = (self.starting_position + *first_step).unwrap();

        while next_position != self.starting_position {
            let previous_position = *path.last().unwrap();
            path.push(next_position);

            next_position = self
                .connects_to(next_position)
                .find(|&p| p != previous_position)
                .unwrap();
        }

        path
    }
}

impl FromStr for Map {
    type Err = char;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height = s.lines().count();
        let width = s.lines().next().unwrap_or_default().len();

        let tiles = s
            .lines()
            .flat_map(|line| line.chars().map(Tile::try_from))
            .collect::<Result<Vec<Tile>, _>>()?;

        let idx = tiles
            .iter()
            .position(|&tile| tile == Tile::StartingPosition)
            .expect("No starting position!");
        let row = idx / width;
        let col = idx % width;

        Ok(Self {
            grid: Grid::new(height, width, tiles).unwrap(),
            starting_position: Position::new(col, row),
        })
    }
}

fn part1(map: &Map) -> usize {
    // `len` will give us the number of steps to get back to the starting point. The farthest
    // point will be at half the distance.
    map.generate_path().len() / 2
}

fn part2(map: &Map) -> usize {
    // Pick's theorem:
    //      A = i + b / 2 - 1
    // where A is the area of the polygon, i the number of interior points and b the number of
    // boundary points.
    // The area can be calculated using the Shoelace formula. Every point in the closed loop is a
    // boundary point, which leaves 'i', which is the number of tiles we are looking for.
    let path = map.generate_path();

    let double_area = path
        .iter()
        .zip(path.iter().cycle().skip(1))
        .map(|(p1, p2)| (p1.y() as isize + p2.y() as isize) * (p1.x() as isize - p2.x() as isize))
        .sum::<isize>()
        .unsigned_abs();

    (double_area - path.len()) / 2 + 1
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT)?;
    let map = input.parse().unwrap();

    println!("The first answer is: {}", part1(&map));
    println!("The second answer is: {}", part2(&map));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "\
        .....\n\
        .S-7.\n\
        .|.|.\n\
        .L-J.\n\
        .....\n\
    ";

    const EXAMPLE_2: &str = "\
        ..F7.\n\
        .FJ|.\n\
        SJ.L7\n\
        |F--J\n\
        LJ...\n\
    ";

    const EXAMPLE_3: &str = "\
        ...........\n\
        .S-------7.\n\
        .|F-----7|.\n\
        .||.....||.\n\
        .||.....||.\n\
        .|L-7.F-J|.\n\
        .|..|.|..|.\n\
        .L--J.L--J.\n\
        ...........\n\
    ";

    const EXAMPLE_4: &str = "\
        .F----7F7F7F7F-7....\n\
        .|F--7||||||||FJ....\n\
        .||.FJ||||||||L7....\n\
        FJL7L7LJLJ||LJ.L-7..\n\
        L--J.L7...LJS7F-7L7.\n\
        ....F-J..F7FJ|L7L7L7\n\
        ....L7.F7||L7|.L7L7|\n\
        .....|FJLJ|FJ|F7|.LJ\n\
        ....FJL-7.||.||||...\n\
        ....L---J.LJ.LJLJ...\n\
    ";

    const EXAMPLE_5: &str = "\
        FF7FSF7F7F7F7F7F---7\n\
        L|LJ||||||||||||F--J\n\
        FL-7LJLJ||||||LJL-77\n\
        F--JF--7||LJLJ7F7FJ-\n\
        L---JF-JLJ.||-FJLJJ7\n\
        |F|F-JF---7F7-L7L|7|\n\
        |FFJF7L7F-JF7|JL---7\n\
        7-L-JL7||F7|L7F-7F7|\n\
        L.L7LFJ|||||FJL7||LJ\n\
        L7JLJL-JLJLJL--JLJ.L\n\
    ";

    #[test]
    fn test_part1_ex1() {
        let map = EXAMPLE_1.parse().unwrap();
        let actual = part1(&map);
        let expected = 4;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part1_ex2() {
        let map = EXAMPLE_2.parse().unwrap();
        let actual = part1(&map);
        let expected = 8;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2_ex3() {
        let map = EXAMPLE_3.parse().unwrap();
        let actual = part2(&map);
        let expected = 4;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2_ex4() {
        let map = EXAMPLE_4.parse().unwrap();
        let actual = part2(&map);
        let expected = 8;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2_ex5() {
        let map = EXAMPLE_5.parse().unwrap();
        let actual = part2(&map);
        let expected = 10;

        assert_eq!(expected, actual);
    }
}
