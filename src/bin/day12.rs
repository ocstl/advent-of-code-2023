use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

const INPUT: &str = "./input/day12.txt";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Condition {
    Operational,
    Broken,
    Unknown,
}

impl From<char> for Condition {
    fn from(value: char) -> Self {
        match value {
            '.' => Condition::Operational,
            '#' => Condition::Broken,
            '?' => Condition::Unknown,
            _ => unreachable!("Invalid condition."),
        }
    }
}

#[derive(Debug, Clone)]
struct Row {
    springs: Vec<Condition>,
    contiguous_groups: Vec<usize>,
}

impl Row {
    fn count_possible_arrangements(&self) -> usize {
        let mut cache: HashMap<(usize, usize, usize), usize> = HashMap::new();
        self.recursive(0, 0, 0, &mut cache)
    }

    fn recursive(
        &self,
        spring: usize,
        damaged_count: usize,
        group: usize,
        cache: &mut HashMap<(usize, usize, usize), usize>,
    ) -> usize {
        if let Some(count) = cache.get(&(spring, damaged_count, group)) {
            return *count;
        }

        let mut count = 0;
        match self.springs.get(spring) {
            // If the spring is operational:
            //      - if the previous spring was operational, just skip to the next one.
            //      - if we just terminated a group of broken springs, check that the count is
            //        good. If it is, reset the count; if not, terminate this branch.
            Some(Condition::Operational) => {
                if damaged_count == 0 {
                    count += self.recursive(spring + 1, damaged_count, group, cache);
                } else if damaged_count
                    == self
                        .contiguous_groups
                        .get(group)
                        .copied()
                        .unwrap_or_default()
                {
                    count += self.recursive(spring + 1, 0, group + 1, cache);
                }
            }
            // If the spring is broken, simply increment the current count for the group.
            Some(Condition::Broken) => {
                count += self.recursive(spring + 1, damaged_count + 1, group, cache);
            }
            // If the condition is unknown, do both of the above.
            Some(Condition::Unknown) => {
                count += self.recursive(spring + 1, damaged_count + 1, group, cache);

                if damaged_count == 0 {
                    count += self.recursive(spring + 1, damaged_count, group, cache);
                } else if damaged_count
                    == self
                        .contiguous_groups
                        .get(group)
                        .copied()
                        .unwrap_or_default()
                {
                    count += self.recursive(spring + 1, 0, group + 1, cache);
                }
            }
            // If we have reached the end of the row:
            //      - if the previous spring was operational, check that there are no more groups.
            //      - if we were counting broken springs, check that the count is good AND that it
            //        is the last group.
            None => {
                if damaged_count
                    == self
                        .contiguous_groups
                        .get(group)
                        .copied()
                        .unwrap_or_default()
                    && self.contiguous_groups.get(group + 1).is_none()
                {
                    count += 1;
                }
            }
        };

        cache.insert((spring, damaged_count, group), count);
        count
    }

    fn expand(&self) -> Self {
        Self {
            springs: self
                .springs
                .iter()
                .copied()
                .chain(std::iter::once(Condition::Unknown))
                .cycle()
                .take(4 + self.springs.len() * 5)
                .collect(),
            contiguous_groups: self.contiguous_groups.repeat(5),
        }
    }
}

impl FromStr for Row {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (springs, groups) = s.split_once(' ').expect("Invalid format.");
        let springs = springs.chars().map(Condition::from).collect();
        let groups = groups
            .split(',')
            .map(str::parse)
            .collect::<Result<Vec<usize>, _>>()?;

        Ok(Self {
            springs,
            contiguous_groups: groups,
        })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT)?;
    let rows = input
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<Row>, _>>()?;

    println!(
        "The first answer is: {}",
        rows.iter()
            .map(Row::count_possible_arrangements)
            .sum::<usize>()
    );
    println!(
        "The second answer is: {}",
        rows.iter()
            .map(|row| row.expand().count_possible_arrangements())
            .sum::<usize>()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        ???.### 1,1,3\n\
        .??..??...?##. 1,1,3\n\
        ?#?#?#?#?#?#?#? 1,3,1,6\n\
        ????.#...#... 4,1,1\n\
        ????.######..#####. 1,6,5\n\
        ?###???????? 3,2,1\n\
    ";

    #[test]
    fn test_part1() {
        let rows = EXAMPLE
            .lines()
            .map(str::parse)
            .collect::<Result<Vec<Row>, _>>()
            .unwrap();
        let actual = rows.iter().map(Row::count_possible_arrangements).sum();
        let expected: usize = 21;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2() {
        let rows = EXAMPLE
            .lines()
            .map(str::parse)
            .collect::<Result<Vec<Row>, _>>()
            .unwrap();
        let actual = rows
            .iter()
            .map(|row| row.expand().count_possible_arrangements())
            .sum();
        let expected: usize = 525152;

        assert_eq!(expected, actual);
    }
}
