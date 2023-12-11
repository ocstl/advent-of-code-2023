use advent_of_code_2023::grid::Position;
use std::str::FromStr;

const INPUT: &str = "./input/day11.txt";
const GALAXY: char = '#';

#[derive(Debug, Default, Clone)]
struct Image(Vec<Position>);

impl Image {
    fn expand(&self, age: usize) -> Self {
        // Precompute the expansion factors, by checking whether a row/column is empty.
        let (max_row, max_col) = self.0.iter().fold((0, 0), |(x, y), galaxy| {
            (x.max(galaxy.x()), y.max(galaxy.y()))
        });

        let row_incrementer: Vec<usize> = (0..=max_row)
            .scan(0, |expansion, row| {
                if self.0.iter().all(|galaxy| galaxy.y() != row) {
                    *expansion += age;
                }
                Some(*expansion)
            })
            .collect();
        let col_incrementer: Vec<usize> = (0..=max_col)
            .scan(0, |expansion, col| {
                if self.0.iter().all(|galaxy| galaxy.x() != col) {
                    *expansion += age;
                }
                Some(*expansion)
            })
            .collect();

        // Then, move the coordinates down/right by the appropriate amount.
        Self(
            self.0
                .iter()
                .map(|galaxy| {
                    Position::new(
                        galaxy.x() + col_incrementer[galaxy.x()],
                        galaxy.y() + row_incrementer[galaxy.y()],
                    )
                })
                .collect(),
        )
    }
    fn sum_shortest_paths(&self) -> usize {
        self.0
            .iter()
            .enumerate()
            .flat_map(|(idx, first)| {
                self.0[idx + 1..]
                    .iter()
                    .map(|&second| first.manhattan_distance(second))
            })
            .sum()
    }
}

impl FromStr for Image {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .enumerate()
                .flat_map(|(row, line)| {
                    line.char_indices().filter_map(move |(col, c)| {
                        if c == GALAXY {
                            Some(Position::new(col, row))
                        } else {
                            None
                        }
                    })
                })
                .collect(),
        ))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT)?;
    let image: Image = input.parse().unwrap();

    println!(
        "The first answer is: {}",
        image.expand(1).sum_shortest_paths()
    );
    println!(
        "The second answer is: {}",
        image.expand(999_999).sum_shortest_paths()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        ...#......\n\
        .......#..\n\
        #.........\n\
        ..........\n\
        ......#...\n\
        .#........\n\
        .........#\n\
        ..........\n\
        .......#..\n\
        #...#.....\n\
    ";

    #[test]
    fn test_part1() {
        let image: Image = EXAMPLE.parse().unwrap();
        let actual = image.expand(1).sum_shortest_paths();
        let expected = 374;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2_ex1() {
        let image: Image = EXAMPLE.parse().unwrap();
        let actual = image.expand(9).sum_shortest_paths();
        let expected = 1030;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2_ex2() {
        let image: Image = EXAMPLE.parse().unwrap();
        let actual = image.expand(99).sum_shortest_paths();
        let expected = 8410;

        assert_eq!(expected, actual);
    }
}
