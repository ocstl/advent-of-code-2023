use advent_of_code_2023::grid::{Grid, Position};
use std::collections::HashSet;

const INPUT: &str = "./input/day21.txt";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Plot {
    Garden,
    Rock,
}

trait Garden {
    fn reachable_plots(
        &self,
        starting_points: HashSet<Position>,
        steps: usize,
    ) -> HashSet<Position>;
}

impl Garden for Grid<Plot> {
    fn reachable_plots(
        &self,
        mut starting_points: HashSet<Position>,
        steps: usize,
    ) -> HashSet<Position> {
        for _ in 1..=steps {
            starting_points = starting_points
                .into_iter()
                .flat_map(Position::neighbours)
                .filter(|position| self.get(*position) == Some(&Plot::Garden))
                .collect();
        }

        starting_points
    }
}

fn parse_input(input: &str) -> (Position, Grid<Plot>) {
    let height = input.lines().count();
    let width = input.lines().next().unwrap_or_default().len();

    let mut plots = Vec::with_capacity(height * width);
    let mut starting_point = Position::default();

    for (row, line) in input.lines().enumerate() {
        for (col, plot) in line.char_indices() {
            match plot {
                '.' => plots.push(Plot::Garden),
                '#' => plots.push(Plot::Rock),
                'S' => {
                    starting_point = Position::new(col, row);
                    plots.push(Plot::Garden)
                }
                _ => unreachable!("Invalid plot type: {plot}."),
            }
        }
    }

    (starting_point, Grid::new(height, width, plots).unwrap())
}

fn part2(starting_point: Position, map: &Grid<Plot>, steps: usize) -> f64 {
    // Eventually, every copy of the map will saturate, and will alternate between values for the
    // odd and even steps.
    // In our input, the row and column of the starting are empty of rocks. Since our map is a
    // square (131 by 131), we reach the end of the map after 65 steps. On the next step, we would
    // start on a new map.
    // Notice also that the first and last rows and columns are also empty of rocks. This means
    // that we need only consider the first 'entry' from the same side, as it will propagate at
    // least as fast as the next possible entry point.
    // BUT we do need to consider what happens for the "corner" maps (diagonally from the starting
    // point), as the first starting points happen at both the row and column levels.
    // Interestingly, after another 131 steps, we'll have crossed the new maps, essentially
    // repeating the initial pattern (65 steps).
    // After another 131 steps, we now have covered the diagonal maps.
    // At this point, we will have a pattern: every 131 steps, we get a new diamond whose interior
    // are saturated maps, the exterior are diagonally-filled maps, except the four cardinal points.
    // The side of the diamond grows linearly, which means the diamond's surface (hence the number
    // of reachable plots) grows quadratically. We just need to fit a quadratic equation.
    let size = map.width();

    let starting_point =
        Position::new(starting_point.x() + 2 * size, starting_point.y() + 2 * size);
    let new_height = size * 5;
    let new_width = size * 5;
    let tiles: Vec<Plot> = (0..new_height)
        .flat_map(|row| map.row(row % size).copied().cycle().take(new_width))
        .collect();
    let map: Grid<Plot> = Grid::new(new_height, new_width, tiles).unwrap();

    let starting_point = HashSet::from([starting_point]);
    let starting_point = map.reachable_plots(starting_point, size / 2);
    let x1 = (size / 2) as f64;
    let y1 = starting_point.len() as f64;
    let starting_point = map.reachable_plots(starting_point, size);
    let x2 = x1 + (size as f64);
    let y2 = starting_point.len() as f64;
    let starting_point = map.reachable_plots(starting_point, size);
    let x3 = x2 + (size as f64);
    let y3 = starting_point.len() as f64;
    let steps = steps as f64;

    // Use Lagrange interpolating polynomials (and f64 to avoid overflow).
    y1 * ((steps - x2) * (steps - x3)) / ((x1 - x2) * (x1 - x3))
        + y2 * ((steps - x1) * (steps - x3)) / ((x2 - x1) * (x2 - x3))
        + y3 * ((steps - x1) * (steps - x2)) / ((x3 - x1) * (x3 - x2))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT)?;
    let (starting_point, map) = parse_input(&input);

    println!(
        "The first answer is: {}",
        map.reachable_plots(HashSet::from([starting_point]), 64)
            .len()
    );
    println!(
        "The second answer is: {}",
        part2(starting_point, &map, 26501365)
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        ...........\n\
        .....###.#.\n\
        .###.##..#.\n\
        ..#.#...#..\n\
        ....#.#....\n\
        .##..S####.\n\
        .##..#...#.\n\
        .......##..\n\
        .##.#.####.\n\
        .##..##.##.\n\
        ...........\n\
    ";

    #[test]
    fn test_part1() {
        let (starting_point, map) = parse_input(EXAMPLE);
        let starting_point = HashSet::from([starting_point]);

        // One step.
        let actual = map.reachable_plots(starting_point.clone(), 1).len();
        let expected = 2;
        assert_eq!(expected, actual);

        // Two steps.
        let actual = map.reachable_plots(starting_point.clone(), 2).len();
        let expected = 4;
        assert_eq!(expected, actual);

        // Three steps.
        let actual = map.reachable_plots(starting_point.clone(), 3).len();
        let expected = 6;
        assert_eq!(expected, actual);

        // Six steps.
        let actual = map.reachable_plots(starting_point.clone(), 6).len();
        let expected = 16;
        assert_eq!(expected, actual);
    }
}
