use std::num::ParseIntError;
use std::str::FromStr;

const INPUT: &str = "./input/day9.txt";

type Reading = i64;

#[derive(Debug, Clone)]
struct Sequence(Vec<Reading>);

impl Sequence {
    fn coefficients(&self) -> impl Iterator<Item = Reading> {
        let l = self.0.len() as i64;
        (0..=l).scan(0, move |acc, col| {
            *acc = (*acc * (l + 1 - col)).checked_div(col).unwrap_or(1);
            Some(*acc)
        })
    }
    fn next_value(&self) -> Reading {
        // We can keep going all the way down to a single element, as, once we reach a line of 0's,
        // all the lines below will remain 0's.
        // Noting that the first line is differences (x_2 - x_1), the second line is a difference of
        // two differences (x_2 - 2 * x_1 + x_0), etc. This looks like Pascal's triangle!
        // Starting at the bottom, the new value is simply the accumulation of the last elements
        // of each row.
        // In the example above, x_3 = x_2 + (x_2 - x_1) + (x_2 - 2 * x_1 + x_0), or:
        //      x_3 = 3 * x_2 - 3 * x_1 + x_0
        // The coefficients are simply the number of combinations, with alternating signs.
        [1, -1]
            .into_iter()
            .cycle()
            .zip(self.coefficients().skip(1))
            .zip(self.0.iter().rev())
            .fold(0, |acc, ((sign, coefficient), reading)| {
                acc + sign * coefficient * reading
            })
    }

    fn prev_value(&self) -> Reading {
        // See `next_value` for some explanation, but start at the beginning instead of the end.
        [1, -1]
            .into_iter()
            .cycle()
            .zip(self.coefficients().skip(1))
            .zip(self.0.iter())
            .fold(0, |acc, ((sign, coefficient), reading)| {
                acc + sign * coefficient * reading
            })
    }
}

impl FromStr for Sequence {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.split_whitespace()
                .map(Reading::from_str)
                .collect::<Result<Vec<Reading>, _>>()?,
        ))
    }
}

fn part1(sequences: &[Sequence]) -> Reading {
    sequences.iter().map(Sequence::next_value).sum()
}

fn part2(sequences: &[Sequence]) -> Reading {
    sequences.iter().map(Sequence::prev_value).sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT)?;
    let sequences = input
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<Sequence>, _>>()?;

    println!("The first answer is: {}", part1(&sequences));
    println!("The second answer is: {}", part2(&sequences));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        0 3 6 9 12 15\n\
        1 3 6 10 15 21\n\
        10 13 16 21 30 45\n\
    ";

    #[test]
    fn test_part1() {
        let sequences = EXAMPLE
            .lines()
            .map(str::parse)
            .collect::<Result<Vec<Sequence>, _>>()
            .unwrap();
        let actual = part1(&sequences);
        let expected = 114;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2() {
        let sequences = EXAMPLE
            .lines()
            .map(str::parse)
            .collect::<Result<Vec<Sequence>, _>>()
            .unwrap();
        let actual = part2(&sequences);
        let expected = 2;

        assert_eq!(expected, actual);
    }
}
