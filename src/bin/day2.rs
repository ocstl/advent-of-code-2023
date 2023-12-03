use std::cmp::Ordering;
use std::num::ParseIntError;
use std::str::FromStr;

const INPUT: &str = "./input/day2.txt";

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}

impl CubeSet {
    fn power(self) -> u32 {
        self.red * self.green * self.blue
    }
}

impl PartialOrd for CubeSet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (
            self.red.cmp(&other.red),
            self.green.cmp(&other.green),
            self.blue.cmp(&other.blue),
        ) {
            (Ordering::Greater, Ordering::Greater, Ordering::Greater) => Some(Ordering::Greater),
            (Ordering::Greater, Ordering::Greater, Ordering::Equal) => Some(Ordering::Greater),
            (Ordering::Greater, Ordering::Greater, Ordering::Less) => None,
            (Ordering::Greater, Ordering::Equal, Ordering::Greater) => Some(Ordering::Greater),
            (Ordering::Greater, Ordering::Equal, Ordering::Equal) => Some(Ordering::Greater),
            (Ordering::Greater, Ordering::Equal, Ordering::Less) => None,
            (Ordering::Greater, Ordering::Less, Ordering::Greater) => None,
            (Ordering::Greater, Ordering::Less, Ordering::Equal) => None,
            (Ordering::Greater, Ordering::Less, Ordering::Less) => None,
            (Ordering::Equal, Ordering::Greater, Ordering::Greater) => Some(Ordering::Greater),
            (Ordering::Equal, Ordering::Greater, Ordering::Equal) => Some(Ordering::Greater),
            (Ordering::Equal, Ordering::Greater, Ordering::Less) => None,
            (Ordering::Equal, Ordering::Equal, Ordering::Greater) => Some(Ordering::Greater),
            (Ordering::Equal, Ordering::Equal, Ordering::Equal) => Some(Ordering::Equal),
            (Ordering::Equal, Ordering::Equal, Ordering::Less) => Some(Ordering::Less),
            (Ordering::Equal, Ordering::Less, Ordering::Greater) => None,
            (Ordering::Equal, Ordering::Less, Ordering::Equal) => Some(Ordering::Less),
            (Ordering::Equal, Ordering::Less, Ordering::Less) => Some(Ordering::Less),
            (Ordering::Less, Ordering::Greater, Ordering::Greater) => None,
            (Ordering::Less, Ordering::Greater, Ordering::Equal) => None,
            (Ordering::Less, Ordering::Greater, Ordering::Less) => None,
            (Ordering::Less, Ordering::Equal, Ordering::Greater) => None,
            (Ordering::Less, Ordering::Equal, Ordering::Equal) => Some(Ordering::Less),
            (Ordering::Less, Ordering::Equal, Ordering::Less) => Some(Ordering::Less),
            (Ordering::Less, Ordering::Less, Ordering::Greater) => None,
            (Ordering::Less, Ordering::Less, Ordering::Equal) => Some(Ordering::Less),
            (Ordering::Less, Ordering::Less, Ordering::Less) => Some(Ordering::Less),
        }
    }
}

impl std::ops::BitOr for CubeSet {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        CubeSet {
            red: self.red.max(rhs.red),
            green: self.green.max(rhs.green),
            blue: self.blue.max(rhs.blue),
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Game {
    id: u32,
    sets: Vec<CubeSet>,
}

impl FromStr for Game {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game, sets) = s.trim().split_once(':').expect("Bad format.");
        let (_, id) = game.split_once(' ').expect("Bad format.");
        let id = u32::from_str(id.trim())?;

        let sets = sets
            .split(';')
            .map(|set| {
                let mut cubes = CubeSet::default();
                for count in set.trim().split(',') {
                    let (count, color) = count.trim().split_once(' ').expect("Bad format.");
                    let count = u32::from_str(count.trim()).expect("Bad format.");
                    match color {
                        "red" => cubes.red = count,
                        "green" => cubes.green = count,
                        "blue" => cubes.blue = count,
                        _ => unreachable!("Invalid color."),
                    }
                }

                cubes
            })
            .collect();

        Ok(Game { id, sets })
    }
}

fn part1(games: &[Game]) -> u32 {
    const CUBES: CubeSet = CubeSet {
        red: 12,
        green: 13,
        blue: 14,
    };

    games
        .iter()
        .filter_map(|game| {
            if game.sets.iter().all(|&set| set <= CUBES) {
                Some(game.id)
            } else {
                None
            }
        })
        .sum()
}

fn part2(games: &[Game]) -> u32 {
    games
        .iter()
        .map(|game| {
            game.sets
                .iter()
                .fold(CubeSet::default(), |acc, &s| acc | s)
                .power()
        })
        .sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT)?;
    let games = input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<Game>, _>>()?;

    println!("The first answer is: {}", part1(&games));
    println!("The second answer is: {}", part2(&games));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let s = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let games = s
            .lines()
            .map(|line| line.parse())
            .collect::<Result<Vec<Game>, _>>()
            .unwrap();
        let actual = part1(&games);
        let expected = 8;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2() {
        let s = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let games = s
            .lines()
            .map(|line| line.parse())
            .collect::<Result<Vec<Game>, _>>()
            .unwrap();
        let actual = part2(&games);
        let expected = 2286;
        assert_eq!(expected, actual);
    }
}
