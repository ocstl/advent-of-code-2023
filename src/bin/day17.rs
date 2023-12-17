use advent_of_code_2023::grid::{Direction, Grid, Position};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

const INPUT: &str = "./input/day17.txt";

type CityMap = Grid<u32>;

fn parse_input(input: &str) -> CityMap {
    let height = input.lines().count();
    let width = input.lines().next().unwrap_or_default().len();
    let blocks = input
        .lines()
        .flat_map(|line| line.bytes().map(|block| u32::from(block - b'0')))
        .collect();

    CityMap::new(height, width, blocks).unwrap()
}

fn minimize_heat_loss<const MINIMAL_STRAIGHT_LINE: u32, const MAXIMAL_STRAIGHT_LINE: u32>(
    city_map: &CityMap,
) -> u32 {
    let destination = Position::new(city_map.width() - 1, city_map.height() - 1);

    let mut to_visit = BinaryHeap::new();
    to_visit.push((Reverse(0), Position::default(), Direction::Right, 0));
    to_visit.push((Reverse(0), Position::default(), Direction::Down, 0));
    let mut visited = HashSet::new();

    while let Some((Reverse(heat_loss), position, direction, straight)) = to_visit.pop() {
        if position == destination {
            return heat_loss;
        }

        if !visited.insert((position, direction, straight)) {
            continue;
        }

        // We can only turn once we have travelled far enough on a straight line.
        if straight >= MINIMAL_STRAIGHT_LINE {
            let left = direction.rotate_left();
            if let Some(new_position) = position + left {
                if let Some(cost) = city_map.get(new_position) {
                    to_visit.push((Reverse(heat_loss + cost), new_position, left, 1))
                }
            }

            let right = direction.rotate_right();
            if let Some(new_position) = position + right {
                if let Some(cost) = city_map.get(new_position) {
                    to_visit.push((Reverse(heat_loss + cost), new_position, right, 1))
                }
            }
        }

        // We can only go on a straight line for so long, before we need to turn.
        if straight < MAXIMAL_STRAIGHT_LINE {
            if let Some(new_position) = position + direction {
                if let Some(cost) = city_map.get(new_position) {
                    to_visit.push((
                        Reverse(heat_loss + cost),
                        new_position,
                        direction,
                        straight + 1,
                    ));
                }
            }
        }
    }

    0
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT)?;
    let city_map = parse_input(&input);

    println!(
        "The first answer is: {}",
        minimize_heat_loss::<1, 3>(&city_map)
    );
    println!(
        "The second answer is: {}",
        minimize_heat_loss::<4, 10>(&city_map)
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        2413432311323\n\
        3215453535623\n\
        3255245654254\n\
        3446585845452\n\
        4546657867536\n\
        1438598798454\n\
        4457876987766\n\
        3637877979653\n\
        4654967986887\n\
        4564679986453\n\
        1224686865563\n\
        2546548887735\n\
        4322674655533\n\
    ";

    #[test]
    fn test_part1() {
        let city_map = parse_input(EXAMPLE);
        let actual = minimize_heat_loss::<1, 3>(&city_map);
        let expected = 102;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2() {
        let city_map = parse_input(EXAMPLE);
        let actual = minimize_heat_loss::<4, 10>(&city_map);
        let expected = 94;

        assert_eq!(expected, actual);
    }
}
