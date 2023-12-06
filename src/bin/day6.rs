const INPUT: &str = "./input/day6.txt";

fn ways_to_beat_record(time: u64, distance: u64) -> u64 {
    // We can use the quadratic equation (to yield the first and last time we equate the
    // record).
    let time = time as f64;
    let distance = distance as f64;
    let discriminant = (time.powi(2) - 4.0 * distance).sqrt();
    let first_time = 0.5 * (time - discriminant);
    let last_time = 0.5 * (time + discriminant);

    // If the times (first and last) are exact integers, we haven't actually beat the record.
    let first_time = (first_time.ceil() + if first_time.fract() == 0.0 { 1.0 } else { 0.0 }) as u64;
    let last_time = (last_time.floor() - if last_time.fract() == 0.0 { 1.0 } else { 0.0 }) as u64;

    // As a quick sanity check, if `first_time` == `last_time`, we should have 1, not 0.
    1 + last_time - first_time
}

fn part1(input: &str) -> u64 {
    let mut iter = input.lines();
    let times = iter.next().unwrap_or_default().split_whitespace().skip(1);
    let distances = iter.next().unwrap_or_default().split_whitespace().skip(1);

    times
        .zip(distances)
        .map(|(t, d)| {
            let time = t.parse().expect("Invalid time.");
            let distance = d.parse().expect("Invalid distance.");
            ways_to_beat_record(time, distance)
        })
        .product()
}

fn part2(input: &str) -> u64 {
    let parse_number = |line: &str| -> u64 {
        line.bytes().fold(0, |acc, d| {
            if d.is_ascii_digit() {
                acc * 10 + u64::from(d - b'0')
            } else {
                acc
            }
        })
    };

    let mut iter = input.lines();
    let time = parse_number(iter.next().unwrap_or_default());
    let distance = parse_number(iter.next().unwrap_or_default());

    ways_to_beat_record(time, distance)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT)?;

    println!("The first answer is: {}", part1(&input));
    println!("The second answer is: {}", part2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        Time:      7  15   30\n\
        Distance:  9  40  200\n\
    ";

    #[test]
    fn test_part1() {
        let actual = part1(EXAMPLE);
        let expected = 288;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2() {
        let actual = part2(EXAMPLE);
        let expected = 71503;

        assert_eq!(expected, actual);
    }
}
