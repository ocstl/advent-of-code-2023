const INPUT: &str = "./input/day15.txt";

#[derive(Debug, Clone)]
struct HashMap<'label> {
    boxes: [Vec<(&'label str, u8)>; 256],
}

impl<'label> HashMap<'label> {
    fn hash(s: &str) -> u8 {
        s.trim()
            .bytes()
            .fold(0, |acc, c| acc.wrapping_add(c).wrapping_mul(17))
    }

    fn step(&mut self, s: &'label str) {
        if let Some((label, lens)) = s.split_once('=') {
            let lens = lens.bytes().next().expect("Missing lens number.") - b'0';

            // If already there, replace it. Otherwise, insert it.
            let b = &mut self.boxes[usize::from(Self::hash(label))];
            if let Some(l) = b.iter_mut().find(|(l, _)| l == &label) {
                l.1 = lens;
            } else {
                b.push((label, lens));
            }
        } else if let Some((label, _)) = s.split_once('-') {
            self.boxes[usize::from(Self::hash(label))].retain(|(l, _)| l != &label);
        }
    }

    fn focusing_power(&self) -> usize {
        self.boxes
            .iter()
            .enumerate()
            .map(|(box_number, lenses)| -> usize {
                (box_number + 1)
                    * lenses
                        .iter()
                        .enumerate()
                        .map(|(slot_number, focal_length)| {
                            (slot_number + 1) * usize::from(focal_length.1)
                        })
                        .sum::<usize>()
            })
            .sum()
    }
}

impl Default for HashMap<'_> {
    fn default() -> Self {
        Self {
            boxes: std::array::from_fn(|_| Vec::new()),
        }
    }
}

fn part1(input: &str) -> usize {
    input
        .split(',')
        .map(|step| usize::from(HashMap::hash(step)))
        .sum()
}

fn part2(input: &str) -> usize {
    input
        .trim()
        .split(',')
        .fold(HashMap::default(), |mut acc, step| {
            acc.step(step);
            acc
        })
        .focusing_power()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT)?;

    println!("The first answer is: {}", part1(input.as_str()));
    println!("The second answer is: {}", part2(input.as_str()));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    const EXPECTED: [u8; 11] = [30, 253, 97, 47, 14, 180, 9, 197, 48, 214, 231];

    #[test]
    fn test_hash_algorithm() {
        let actual = EXAMPLE.split(',').map(HashMap::hash);
        for (a, e) in actual.zip(EXPECTED) {
            assert_eq!(a, e);
        }
    }

    #[test]
    fn test_part1() {
        let actual = part1(EXAMPLE);
        let expected = 1320;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2() {
        let actual = part2(EXAMPLE);
        let expected = 145;

        assert_eq!(expected, actual);
    }
}
