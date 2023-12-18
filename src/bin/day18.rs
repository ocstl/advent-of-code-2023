use advent_of_code_2023::position::{Direction, Position, DOWN, LEFT, RIGHT, UP};
use std::num::ParseIntError;
use std::str::FromStr;

const INPUT: &str = "./input/day18.txt";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Instruction {
    direction: Direction,
    steps: isize,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();
        let direction = Direction::try_from(
            iter.next()
                .ok_or(String::from("Missing direction."))?
                .chars()
                .next()
                .unwrap(),
        )
        .map_err(|c| format!("Invalid direction: {c}."))?;

        let steps = iter
            .next()
            .ok_or(String::from("Missing number of steps."))?
            .parse()
            .map_err(|e: ParseIntError| e.to_string())?;

        Ok(Self { direction, steps })
    }
}

#[derive(Debug, Clone)]
struct DigPlan(Vec<Instruction>);

impl FromStr for DigPlan {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut instructions = Vec::new();
        for line in s.lines() {
            instructions.push(line.parse()?);
        }

        Ok(Self(instructions))
    }
}

fn part1(dig_plan: &DigPlan) -> isize {
    // We can use the shoelace formula and Pick's theorem.
    // Note that the "points" do not represent a dimensionless (or size 0) thingamajig, but are
    // actual 1 meter cube holes:
    //  - for non-corners, half is inside and half out.
    //  - for corners:
    //      - interior corners (3/4 in, 1/4 out) are paired with exterior corners (1/4 in, 3/4 out),
    //        which allows us to ignore the difference, as they average out.
    //      - except for 4 outside corners (1/4 in, 3/4 out). Since we're accounting for only 1/2
    //        out for these, we need to add 4 * (3/4 - 1/2) or 1.
    let mut position = Position::default();
    let mut accumulator = 0;
    let mut total_steps = 0;

    for &Instruction { direction, steps } in dig_plan.0.iter() {
        let next_position = Position::new(
            position.x + direction.dx * steps,
            position.y + direction.dy * steps,
        );
        accumulator += (position.x * next_position.y) - (position.y * next_position.x);
        total_steps += steps;
        position = next_position;
    }

    (total_steps + accumulator.abs()) / 2 + 1
}

fn part2(input: &str) -> isize {
    let mut instructions: Vec<Instruction> = Vec::new();
    for line in input.lines() {
        let mut instruction = line.split_whitespace().last().unwrap().chars().skip(2);

        let steps =
            isize::from_str_radix(&instruction.by_ref().take(5).collect::<String>(), 16).unwrap();

        let direction = match instruction.next().unwrap() {
            '0' => RIGHT,
            '1' => DOWN,
            '2' => LEFT,
            '3' => UP,
            c => panic!("{}", c),
        };

        instructions.push(Instruction { direction, steps })
    }

    part1(&DigPlan(instructions))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT)?;
    let dig_plan: DigPlan = input.parse()?;

    println!("The first answer is: {}", part1(&dig_plan));
    println!("The second answer is: {}", part2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        R 6 (#70c710)\n\
        D 5 (#0dc571)\n\
        L 2 (#5713f0)\n\
        D 2 (#d2c081)\n\
        R 2 (#59c680)\n\
        D 2 (#411b91)\n\
        L 5 (#8ceee2)\n\
        U 2 (#caa173)\n\
        L 1 (#1b58a2)\n\
        U 2 (#caa171)\n\
        R 2 (#7807d2)\n\
        U 3 (#a77fa3)\n\
        L 2 (#015232)\n\
        U 2 (#7a21e3)\n\
    ";

    #[test]
    fn test_part1() {
        let dig_plan: DigPlan = EXAMPLE.parse().unwrap();
        let actual = part1(&dig_plan);
        let expected = 62;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2() {
        let actual = part2(EXAMPLE);
        let expected = 952408144115;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_line_dig() {
        let dig_plan: DigPlan = "L 2\nR 2".parse().unwrap();
        let actual = part1(&dig_plan);
        let expected = 3;

        assert_eq!(expected, actual);
    }
}
