use std::num::ParseIntError;
use std::ops::{RangeInclusive, Sub};
use std::str::FromStr;
use z3::ast::{Ast, Int};
use z3::{Config, Context, Solver};

const INPUT: &str = "./input/day24.txt";

type Value = i64;
type TestArea = RangeInclusive<Value>;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: Value,
    y: Value,
    z: Value,
}

impl Position {
    fn new(x: Value, y: Value, z: Value) -> Self {
        Self { x, y, z }
    }
}

impl Sub for Position {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Position::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl FromStr for Position {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.trim().split(',');
        let x = iter.next().unwrap_or_default().trim().parse()?;
        let y = iter.next().unwrap_or_default().trim().parse()?;
        let z = iter.next().unwrap_or_default().trim().parse()?;

        Ok(Self::new(x, y, z))
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Velocity {
    x: Value,
    y: Value,
    z: Value,
}

impl Velocity {
    fn new(x: Value, y: Value, z: Value) -> Self {
        Self { x, y, z }
    }
}

impl Sub for Velocity {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Velocity::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl FromStr for Velocity {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.trim().split(',');
        let x = iter.next().unwrap_or_default().trim().parse()?;
        let y = iter.next().unwrap_or_default().trim().parse()?;
        let z = iter.next().unwrap_or_default().trim().parse()?;

        Ok(Self::new(x, y, z))
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Hailstone {
    position: Position,
    velocity: Velocity,
}

impl Hailstone {
    fn new(position: Position, velocity: Velocity) -> Self {
        Self { position, velocity }
    }

    fn crosses_path_xy(self, other: Self, test_area: &TestArea) -> bool {
        // Convert them into lines.
        let slope1 = (self.velocity.y as f64) / (self.velocity.x as f64);
        let intercept1 = (self.position.y as f64) - slope1 * (self.position.x as f64);
        let slope2 = (other.velocity.y as f64) / (other.velocity.x as f64);
        let intercept2 = (other.position.y as f64) - slope2 * (other.position.x as f64);

        // Find the interception point.
        let x = (intercept2 - intercept1) / (slope1 - slope2);
        let y = x * slope1 + intercept1;
        let min_coordinate = *test_area.start() as f64;
        let max_coordinate = *test_area.end() as f64;

        // Check that we are moving forward in time and that we are within the test area.
        ((x - self.position.x as f64) / (self.velocity.x as f64)).is_sign_positive()
            && ((x - other.position.x as f64) / (other.velocity.x as f64)).is_sign_positive()
            && x >= min_coordinate
            && x <= max_coordinate
            && y >= min_coordinate
            && y <= max_coordinate
    }
}

impl FromStr for Hailstone {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (position, velocity) = s.split_once('@').unwrap();
        let position = position.parse()?;
        let velocity = velocity.parse()?;

        Ok(Self::new(position, velocity))
    }
}

fn part1(hailstones: &[Hailstone], test_area: &TestArea) -> usize {
    hailstones
        .iter()
        .enumerate()
        .flat_map(|(idx, first)| {
            // Avoid comparison twice (and also with itself).
            hailstones[idx + 1..]
                .iter()
                .map(move |second| (first, second))
        })
        .filter(|(a, b)| a.crosses_path_xy(**b, test_area))
        .count()
}

fn part2(hailstones: &[Hailstone]) -> Value {
    // Easier to use z3... :(
    let context = Context::new(&Config::new());
    let solver = Solver::new(&context);

    // Define our unknowns.
    let px = Int::new_const(&context, "px");
    let py = Int::new_const(&context, "py");
    let pz = Int::new_const(&context, "pz");
    let vx = Int::new_const(&context, "vx");
    let vy = Int::new_const(&context, "vy");
    let vz = Int::new_const(&context, "vz");

    // Fill in our constraints. We only need a few hailstones, not all of them.
    for hailstone in &hailstones[..4] {
        let hailstone_px = Int::from_i64(&context, hailstone.position.x);
        let hailstone_py = Int::from_i64(&context, hailstone.position.y);
        let hailstone_pz = Int::from_i64(&context, hailstone.position.z);
        let hailstone_vx = Int::from_i64(&context, hailstone.velocity.x);
        let hailstone_vy = Int::from_i64(&context, hailstone.velocity.y);
        let hailstone_vz = Int::from_i64(&context, hailstone.velocity.z);
        let hailstone_t = Int::fresh_const(&context, "hailstone_t");

        solver.assert(
            &(&hailstone_px + &hailstone_vx * &hailstone_t)._eq(&(&px + &vx * &hailstone_t)),
        );
        solver.assert(
            &(&hailstone_py + &hailstone_vy * &hailstone_t)._eq(&(&py + &vy * &hailstone_t)),
        );
        solver.assert(
            &(&hailstone_pz + &hailstone_vz * &hailstone_t)._eq(&(&pz + &vz * &hailstone_t)),
        );
    }

    solver.check();
    let model = solver.get_model().unwrap();
    let px = model.get_const_interp(&px).unwrap().as_i64().unwrap();
    let py = model.get_const_interp(&py).unwrap().as_i64().unwrap();
    let pz = model.get_const_interp(&pz).unwrap().as_i64().unwrap();

    px + py + pz
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT)?;
    let hailstones = input
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<Hailstone>, _>>()?;

    println!(
        "The first answer is: {}",
        part1(
            &hailstones,
            &RangeInclusive::new(200000000000000, 400000000000000)
        )
    );
    println!("The second answer is: {}", part2(&hailstones));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        19, 13, 30 @ -2,  1, -2\n\
        18, 19, 22 @ -1, -1, -2\n\
        20, 25, 34 @ -2, -2, -4\n\
        12, 31, 28 @ -1, -2, -1\n\
        20, 19, 15 @  1, -5, -3\n\
    ";

    const TEST_AREA: TestArea = TestArea::new(7, 27);

    #[test]
    fn test_part1() {
        let hailstones = EXAMPLE
            .lines()
            .map(str::parse)
            .collect::<Result<Vec<Hailstone>, _>>()
            .unwrap();
        let actual = part1(&hailstones, &TEST_AREA);
        let expected = 2;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2() {
        let hailstones = EXAMPLE
            .lines()
            .map(str::parse)
            .collect::<Result<Vec<Hailstone>, _>>()
            .unwrap();
        let actual = part2(&hailstones);
        let expected = 47;

        assert_eq!(expected, actual);
    }
}
