use advent_of_code_2023::grid::Position;
use advent_of_code_2023::range_extension::RangeExtension;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::ops::Range;
use std::str::FromStr;

const INPUT: &str = "./input/day22.txt";
const FLOOR: usize = 0;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Brick {
    x: Range<usize>,
    y: Range<usize>,
    z: Range<usize>,
}

impl Brick {
    fn move_down_to(&mut self, z: usize) {
        self.z = z..z + self.z.end - self.z.start;
    }

    fn surface(&self) -> impl Iterator<Item = Position> + '_ {
        (self.x.start..self.x.end)
            .flat_map(|x| (self.y.start..self.y.end).map(move |y| Position::new(x, y)))
    }

    fn supports(&self, other: &Self) -> bool {
        self != other
            && self.z.end == other.z.start
            && self.x.overlaps(&other.x)
            && self.y.overlaps(&other.y)
    }

    fn is_supported(&self, other: &Self) -> bool {
        self != other
            && self.z.start == other.z.end
            && self.x.overlaps(&other.x)
            && self.y.overlaps(&other.y)
    }
}

impl FromStr for Brick {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('~').unwrap();
        let mut starts = start
            .split(',')
            .map(|value| value.parse::<usize>().unwrap());
        let mut ends = end.split(',').map(|value| value.parse::<usize>().unwrap());
        let x = starts.next().unwrap()..ends.next().unwrap() + 1;
        let y = starts.next().unwrap()..ends.next().unwrap() + 1;
        let z = starts.next().unwrap()..ends.next().unwrap() + 1;

        Ok(Self { x, y, z })
    }
}

impl Display for Brick {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{},{}~{},{},{}",
            self.x.start,
            self.y.start,
            self.z.start,
            self.x.end - 1,
            self.y.end - 1,
            self.z.end - 1
        )
    }
}

trait Bricks {
    fn settle(&mut self);
    fn supported_by(&self) -> HashMap<&Brick, HashSet<&Brick>>;
    fn supports(&self) -> HashMap<&Brick, HashSet<&Brick>>;
}

impl Bricks for Vec<Brick> {
    fn settle(&mut self) {
        let mut heights = HashMap::new();

        // Settle the bricks, starting with the lowest brick.
        self.sort_unstable_by_key(|brick| brick.z.start);
        for brick in self.iter_mut() {
            let minimum_z = *brick
                .surface()
                .filter_map(|p| heights.get(&p))
                .max()
                .unwrap_or(&(FLOOR + 1));
            brick.move_down_to(minimum_z);
            for p in brick.surface() {
                heights.insert(p, brick.z.end);
            }
        }
    }

    fn supported_by(&self) -> HashMap<&Brick, HashSet<&Brick>> {
        self.iter()
            .map(|brick| {
                (
                    brick,
                    self.iter().filter(|other| other.supports(brick)).collect(),
                )
            })
            .collect()
    }

    fn supports(&self) -> HashMap<&Brick, HashSet<&Brick>> {
        self.iter()
            .map(|brick| {
                (
                    brick,
                    self.iter()
                        .filter(|other| other.is_supported(brick))
                        .collect(),
                )
            })
            .collect()
    }
}

fn part1(bricks: &Vec<Brick>) -> usize {
    // Find the bricks that have a single supporting brick.
    let single_support: HashSet<&Brick> = bricks
        .supported_by()
        .into_iter()
        .filter_map(|(brick, supports)| {
            if supports.len() == 1 {
                Some(brick)
            } else {
                None
            }
        })
        .collect();

    // Now, find the bricks that are not supporting these.
    bricks
        .iter()
        .filter(|brick| !single_support.iter().any(|other| brick.supports(other)))
        .count()
}

fn part2(bricks: &Vec<Brick>) -> usize {
    // Get the bricks supported by other bricks. Remove those that are not supported, as it would
    // complicate matters when filtering.
    let mut supported_by = bricks.supported_by();
    supported_by.retain(|_, others| !others.is_empty());

    // Get the bricks that support other bricks. We can remove the bricks that do not support
    // any other, as they would not cause a chain reaction.
    let mut supports = bricks.supports();
    supports.retain(|_, others| !others.is_empty());

    let mut result = 0;
    for &initial in supports.keys() {
        let mut fallen = HashSet::new();
        let mut to_fall = HashSet::from([initial]);

        // Keep adding falling bricks until no more fall.
        while !to_fall.is_empty() {
            fallen.extend(to_fall.iter());

            to_fall = to_fall
                .into_iter()
                .filter_map(|falling| supports.get(falling))
                .flat_map(|candidates| {
                    candidates.iter().filter(|&&candidate| {
                        !fallen.contains(candidate)
                            && supported_by
                                .get(candidate)
                                .map_or(false, |others| others.is_subset(&fallen))
                    })
                })
                .copied()
                .collect();
        }

        // Remove the initial brick (it doesn't count).
        result += fallen.len() - 1;
    }

    result
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT)?;
    let mut bricks = input
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<Brick>, _>>()?;

    bricks.settle();

    println!("The first answer is: {}", part1(&bricks));
    println!("The second answer is: {}", part2(&bricks));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        1,0,1~1,2,1\n\
        0,0,2~2,0,2\n\
        0,2,3~2,2,3\n\
        0,0,4~0,2,4\n\
        2,0,5~2,2,5\n\
        0,1,6~2,1,6\n\
        1,1,8~1,1,9\n\
    ";

    #[test]
    fn test_part1() {
        let mut bricks = EXAMPLE
            .lines()
            .map(str::parse)
            .collect::<Result<Vec<Brick>, _>>()
            .unwrap();

        bricks.settle();

        let actual = part1(&bricks);
        let expected = 5;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2() {
        let mut bricks = EXAMPLE
            .lines()
            .map(str::parse)
            .collect::<Result<Vec<Brick>, _>>()
            .unwrap();

        bricks.settle();

        let actual = part2(&bricks);
        let expected = 7;

        assert_eq!(expected, actual);
    }
}
