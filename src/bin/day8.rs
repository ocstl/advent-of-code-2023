use num_integer::lcm;
use std::collections::HashMap;

type Node<'node> = &'node str;
const INPUT: &str = "./input/day8.txt";
const START: Node = "AAA";
const END: Node = "ZZZ";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Step {
    Left,
    Right,
}

impl TryFrom<char> for Step {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Step::Left),
            'R' => Ok(Step::Right),
            _ => Err(value),
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Network<'network> {
    nodes: HashMap<Node<'network>, (Node<'network>, Node<'network>)>,
}

impl<'network> Network<'network> {
    fn generate_path(
        &'network self,
        steps: &'network [Step],
        start: Node<'network>,
    ) -> impl Iterator<Item = Node<'network>> {
        steps.iter().cycle().scan(start, |state, step| {
            let destinations = self.nodes.get(state).unwrap();
            match step {
                Step::Left => *state = destinations.0,
                Step::Right => *state = destinations.1,
            }
            Some(*state)
        })
    }
    fn count_steps(&self, steps: &[Step], start: Node, end: Node) -> usize {
        self.generate_path(steps, start)
            .take_while(|node| node != &end)
            .count()
            + 1
    }
}

impl<'input> TryFrom<&'input str> for Network<'input> {
    type Error = &'static str;

    fn try_from(value: &'input str) -> Result<Self, Self::Error> {
        let mut network = Network::default();
        for line in value.lines() {
            let (origin, destinations) = line.split_once('=').ok_or("Bad format.")?;
            let (left, right) = destinations.trim().split_once(',').ok_or("Bad format.")?;

            // Remember to remove the parentheses.
            network
                .nodes
                .insert(origin.trim(), (&left.trim()[1..], &right.trim()[..3]));
        }

        Ok(network)
    }
}

fn parse_input(input: &str) -> Result<(Vec<Step>, Network), Box<dyn std::error::Error>> {
    let (steps, nodes) = input.split_once("\n\n").ok_or("Bad format.")?;

    let steps = steps
        .chars()
        .map(Step::try_from)
        .collect::<Result<Vec<Step>, _>>()
        .unwrap();
    let network = Network::try_from(nodes)?;

    Ok((steps, network))
}

fn part2(steps: &[Step], network: &Network) -> usize {
    // We are going to assume that, once we have reached a given end node ("..Z"), the path only
    // loops back to it, and no other (which could yield a shorter time).
    // This allows us to use the Chinese remainder theorem, considering 'a_x' being the number of
    // steps to reach the end node, and 'n_x' the length of the cycle for 'x' starting point.
    // In this case, we cheat a bit after noticing that the cycle length is the same as the number
    // of steps required to reach the start of the cycle (a_x == 0 mod n_x).
    network
        .nodes
        .keys()
        .filter(|node| node.ends_with('A'))
        .map(|start| {
            let mut path = network.generate_path(steps, start);
            let (steps, end) = path
                .by_ref()
                .enumerate()
                .find(|(_, node)| node.ends_with('Z'))
                .unwrap();
            let cycle = path.take_while(|&node| node != end).count() + 1;
            (steps + 1, cycle)
        })
        .fold(1, |acc, (_, cycle)| lcm(acc, cycle))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT)?;
    let (steps, network) = parse_input(&input)?;

    println!(
        "The first answer is: {}",
        network.count_steps(&steps, START, END)
    );
    println!("The second answer is: {}", part2(&steps, &network));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        RL\n\
        \n\
        AAA = (BBB, CCC)\n\
        BBB = (DDD, EEE)\n\
        CCC = (ZZZ, GGG)\n\
        DDD = (DDD, DDD)\n\
        EEE = (EEE, EEE)\n\
        GGG = (GGG, GGG)\n\
        ZZZ = (ZZZ, ZZZ)\n\
    ";

    const EXAMPLE_2: &str = "\
        LLR\n\
        \n\
        AAA = (BBB, BBB)\n\
        BBB = (AAA, ZZZ)\n\
        ZZZ = (ZZZ, ZZZ)\n\
    ";

    const EXAMPLE_3: &str = "\
        LR\n\
        \n\
        11A = (11B, XXX)\n\
        11B = (XXX, 11Z)\n\
        11Z = (11B, XXX)\n\
        22A = (22B, XXX)\n\
        22B = (22C, 22C)\n\
        22C = (22Z, 22Z)\n\
        22Z = (22B, 22B)\n\
        XXX = (XXX, XXX)\n\
    ";

    #[test]
    fn test_part1_ex1() {
        let (steps, network) = parse_input(EXAMPLE).unwrap();
        let actual = network.count_steps(&steps, START, END);
        let expected = 2;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part1_ex2() {
        let (steps, network) = parse_input(EXAMPLE_2).unwrap();
        let actual = network.count_steps(&steps, START, END);
        let expected = 6;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2() {
        let (steps, network) = parse_input(EXAMPLE_3).unwrap();
        let actual = part2(&steps, &network);
        let expected = 6;

        assert_eq!(expected, actual);
    }
}
