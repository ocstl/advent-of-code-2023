const INPUT: &str = "./input/day1.txt";

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let first = line
                .bytes()
                .find(u8::is_ascii_digit)
                .expect("There should be at least one digit.")
                - b'0';
            let last = line
                .bytes()
                .rfind(u8::is_ascii_digit)
                .expect("There should be at least one digit.")
                - b'0';
            u32::from(first * 10 + last)
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    let match_convert = |s: &str| -> Option<u32> {
        if s.starts_with('0') || s.starts_with("zero") {
            Some(0)
        } else if s.starts_with('1') || s.starts_with("one") {
            Some(1)
        } else if s.starts_with('2') || s.starts_with("two") {
            Some(2)
        } else if s.starts_with('3') || s.starts_with("three") {
            Some(3)
        } else if s.starts_with('4') || s.starts_with("four") {
            Some(4)
        } else if s.starts_with('5') || s.starts_with("five") {
            Some(5)
        } else if s.starts_with('6') || s.starts_with("six") {
            Some(6)
        } else if s.starts_with('7') || s.starts_with("seven") {
            Some(7)
        } else if s.starts_with('8') || s.starts_with("eight") {
            Some(8)
        } else if s.starts_with('9') || s.starts_with("nine") {
            Some(9)
        } else {
            None
        }
    };

    input
        .lines()
        .map(|line| {
            let first = (0..line.len())
                .find_map(|idx| match_convert(&line[idx..]))
                .expect("There should have been at least one match.");
            let last = (0..line.len())
                .rev()
                .find_map(|idx| match_convert(&line[idx..]))
                .expect("There should have been at least one match.");
            first * 10 + last
        })
        .sum()
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

    #[test]
    fn test_part1() {
        let s = "1abc2
                      pqr3stu8vwx
                      a1b2c3d4e5f
                      treb7uchet";
        let actual = part1(s);
        let expected = 142;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2() {
        let s = "two1nine
                      eightwothree
                      abcone2threexyz
                      xtwone3four
                      4nineeightseven2
                      zoneight234
                      7pqrstsixteen";
        let actual = part2(s);
        let expected = 281;
        assert_eq!(expected, actual);
    }
}
