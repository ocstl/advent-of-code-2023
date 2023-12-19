use advent_of_code_2023::range_extension::RangeExtension;
use std::collections::HashMap;
use std::ops::Range;
use std::str::FromStr;

const INPUT: &str = "./input/day19.txt";

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl Part {
    fn rating(self) -> u64 {
        self.x + self.m + self.a + self.s
    }
}

impl FromStr for Part {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut part = Self::default();
        let iter = s.trim_matches(|c| c == '{' || c == '}').split(',');

        for rating in iter {
            if let Some((name, value)) = rating.split_once('=') {
                let value = value.parse().unwrap();
                match name {
                    "x" => part.x = value,
                    "m" => part.m = value,
                    "a" => part.a = value,
                    "s" => part.s = value,
                    _ => (),
                }
            }
        }

        Ok(part)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct PartRange {
    x: Range<u64>,
    m: Range<u64>,
    a: Range<u64>,
    s: Range<u64>,
}

impl PartRange {
    fn is_empty(&self) -> bool {
        self.x.start == self.x.end
            || self.m.start == self.m.end
            || self.a.start == self.a.end
            || self.s.start == self.s.end
    }
    fn possible_combinations(&self) -> u64 {
        (self.x.end - self.x.start)
            * (self.m.end - self.m.start)
            * (self.a.end - self.a.start)
            * (self.s.end - self.s.start)
    }
}

impl Default for PartRange {
    fn default() -> Self {
        Self {
            x: 1..4001,
            m: 1..4001,
            a: 1..4001,
            s: 1..4001,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Category {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Outcome<'outcome> {
    Accept,
    Reject,
    Workflow(&'outcome str),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Rule<'rule> {
    category: Category,
    range: Range<u64>,
    outcome: Outcome<'rule>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Condition<'condition> {
    Condition(Rule<'condition>),
    None(Outcome<'condition>),
}

impl<'rule> Condition<'rule> {
    fn new(s: &'rule str) -> Self {
        if let Some((condition, outcome)) = s.split_once(':') {
            let category = match &condition[0..1] {
                "x" => Category::X,
                "m" => Category::M,
                "a" => Category::A,
                "s" => Category::S,
                _ => unreachable!(),
            };
            let value = u64::from_str(&condition[2..]).unwrap();
            let range = match &condition[1..2] {
                "<" => 0..value,
                ">" => value + 1..u64::MAX,
                _ => unreachable!(),
            };
            let outcome = match outcome {
                "A" => Outcome::Accept,
                "R" => Outcome::Reject,
                workflow => Outcome::Workflow(workflow),
            };
            Self::Condition(Rule {
                category,
                range,
                outcome,
            })
        } else {
            let outcome = match s {
                "A" => Outcome::Accept,
                "R" => Outcome::Reject,
                workflow => Outcome::Workflow(workflow),
            };
            Self::None(outcome)
        }
    }

    fn apply(&self, part: Part) -> Option<&Outcome<'rule>> {
        match self {
            Condition::Condition(rule) => {
                let test = match rule.category {
                    Category::X => part.x,
                    Category::M => part.m,
                    Category::A => part.a,
                    Category::S => part.s,
                };

                if rule.range.contains(&test) {
                    Some(&rule.outcome)
                } else {
                    None
                }
            }
            Condition::None(outcome) => Some(outcome),
        }
    }

    fn apply_range(&self, part_range: &mut PartRange) -> Option<(PartRange, &Outcome)> {
        match self {
            Condition::Condition(rule) => {
                let mut new_range = part_range.clone();
                let (current, new) = match rule.category {
                    Category::X => (&mut part_range.x, &mut new_range.x),
                    Category::M => (&mut part_range.m, &mut new_range.m),
                    Category::A => (&mut part_range.a, &mut new_range.a),
                    Category::S => (&mut part_range.s, &mut new_range.s),
                };

                let intersection = rule.range.intersection(current);
                if intersection.is_empty() {
                    None
                } else {
                    if intersection.start == current.start {
                        *current = intersection.end..current.end;
                    } else {
                        *current = current.start..intersection.start;
                    }
                    *new = intersection;
                    Some((new_range, &rule.outcome))
                }
            }
            Condition::None(outcome) => {
                let new_range = part_range.clone();
                part_range.x = 0..0;
                Some((new_range, outcome))
            }
        }
    }
}

#[derive(Default, Debug)]
#[allow(clippy::type_complexity)]
struct Workflows<'workflows>(HashMap<&'workflows str, Vec<Condition<'workflows>>>);

impl<'workflows> Workflows<'workflows> {
    fn new(s: &'workflows str) -> Self {
        let mut h = HashMap::new();
        for line in s.lines() {
            let (name, workflow) = line.split_once('{').unwrap();
            let rules = workflow
                .trim_end_matches('}')
                .split(',')
                .map(Condition::new)
                .collect();
            h.insert(name, rules);
        }

        Self(h)
    }

    fn apply(&self, workflow: &str, part: Part) -> Option<&Outcome> {
        match self
            .0
            .get(workflow)
            .unwrap()
            .iter()
            .find_map(|rule| rule.apply(part))
        {
            Some(Outcome::Workflow(workflow)) => self.apply(workflow, part),
            result => result,
        }
    }

    fn accept_part(&self, part: Part) -> bool {
        match self.apply("in", part) {
            Some(Outcome::Accept) => true,
            Some(Outcome::Reject) => false,
            outcome => unreachable!("{:?}", outcome),
        }
    }

    fn apply_range(&self, workflow: &str, mut part_range: PartRange) -> Vec<(PartRange, &Outcome)> {
        let mut ranges = Vec::new();
        for rule in self.0.get(workflow).unwrap() {
            if let Some(result) = rule.apply_range(&mut part_range) {
                ranges.push(result);
            }

            if part_range.is_empty() {
                break;
            }
        }

        ranges
    }
}

fn part1(input: &str) -> u64 {
    let (rules, parts) = input.split_once("\n\n").unwrap();
    let workflows = Workflows::new(rules);

    let parts = parts
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<Part>, _>>()
        .unwrap();
    parts
        .into_iter()
        .filter(|&part| workflows.accept_part(part))
        .map(Part::rating)
        .sum::<u64>()
}

fn part2(input: &str) -> u64 {
    let (rules, _) = input.split_once("\n\n").unwrap();
    let workflows = Workflows::new(rules);

    let mut accumulator = 0;
    let mut to_do = vec![(PartRange::default(), &Outcome::Workflow("in"))];
    while let Some((part_range, outcome)) = to_do.pop() {
        match outcome {
            Outcome::Accept => accumulator += part_range.possible_combinations(),
            Outcome::Reject => (),
            Outcome::Workflow(workflow) => {
                to_do.extend(workflows.apply_range(workflow, part_range))
            }
        }
    }

    accumulator
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
        px{a<2006:qkq,m>2090:A,rfg}\n\
        pv{a>1716:R,A}\n\
        lnx{m>1548:A,A}\n\
        rfg{s<537:gd,x>2440:R,A}\n\
        qs{s>3448:A,lnx}\n\
        qkq{x<1416:A,crn}\n\
        crn{x>2662:A,R}\n\
        in{s<1351:px,qqz}\n\
        qqz{s>2770:qs,m<1801:hdj,R}\n\
        gd{a>3333:R,R}\n\
        hdj{m>838:A,pv}\n\
        \n\
        {x=787,m=2655,a=1222,s=2876}\n\
        {x=1679,m=44,a=2067,s=496}\n\
        {x=2036,m=264,a=79,s=2244}\n\
        {x=2461,m=1339,a=466,s=291}\n\
        {x=2127,m=1623,a=2188,s=1013}\n\
    ";

    #[test]
    fn test_part1() {
        let actual = part1(EXAMPLE);
        let expected = 19114;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2() {
        let actual = part2(EXAMPLE);
        let expected = 167409079868000;

        assert_eq!(expected, actual);
    }
}
