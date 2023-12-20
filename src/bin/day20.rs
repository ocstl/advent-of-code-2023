use num_integer::lcm;
use std::cell::Cell;
use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};

const INPUT: &str = "./input/day20.txt";

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
enum Pulse {
    #[default]
    Low,
    High,
}

type Name<'name> = &'name str;
type Destinations<'name> = Vec<Name<'name>>;
type Inputs<'name> = BTreeMap<Name<'name>, Cell<Pulse>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Transmission<'name> {
    pulse: Pulse,
    origin: Name<'name>,
    destination: Name<'name>,
}

impl Default for Transmission<'_> {
    fn default() -> Self {
        Self {
            pulse: Pulse::Low,
            origin: "button",
            destination: "broadcaster",
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
struct Broadcaster<'name> {
    name: Name<'name>,
    destinations: Destinations<'name>,
}

impl<'name> Broadcaster<'name> {
    fn recv(&self, transmission: Transmission) -> impl Iterator<Item = Transmission> {
        self.destinations.iter().map(move |d| Transmission {
            pulse: transmission.pulse,
            origin: self.name,
            destination: d,
        })
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
enum FlipFlopState {
    #[default]
    Off,
    On,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct FlipFlop<'name> {
    name: Name<'name>,
    state: Cell<FlipFlopState>,
    destinations: Destinations<'name>,
}

impl<'name> FlipFlop<'name> {
    fn recv(
        &'name self,
        transmission: Transmission,
    ) -> Box<dyn Iterator<Item = Transmission> + 'name> {
        match transmission.pulse {
            Pulse::High => Box::new(std::iter::empty()),
            Pulse::Low => {
                let pulse = match self.state.get() {
                    FlipFlopState::Off => {
                        self.state.replace(FlipFlopState::On);
                        Pulse::High
                    }
                    FlipFlopState::On => {
                        self.state.replace(FlipFlopState::Off);
                        Pulse::Low
                    }
                };

                Box::new(self.destinations.iter().map(move |d| Transmission {
                    pulse,
                    origin: self.name,
                    destination: d,
                }))
            }
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Conjunction<'name> {
    name: Name<'name>,
    inputs: Inputs<'name>,
    destinations: Destinations<'name>,
}

impl<'name> Conjunction<'name> {
    fn recv(&self, transmission: Transmission) -> impl Iterator<Item = Transmission> {
        self.inputs
            .get(transmission.origin)
            .unwrap()
            .replace(transmission.pulse);

        let pulse = if self.inputs.values().all(|pulse| pulse.get() == Pulse::High) {
            Pulse::Low
        } else {
            Pulse::High
        };

        self.destinations.iter().map(move |d| Transmission {
            pulse,
            origin: self.name,
            destination: d,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Module<'name> {
    Broadcaster(Broadcaster<'name>),
    FlipFlop(FlipFlop<'name>),
    Conjunction(Conjunction<'name>),
}

impl<'name> Module<'name> {
    fn new(s: &'name str) -> Self {
        let (name, destination) = s.split_once(" -> ").unwrap();
        let destinations = destination.trim().split(',').map(|d| d.trim()).collect();
        if name == "broadcaster" {
            Module::Broadcaster(Broadcaster { name, destinations })
        } else if let Some(name) = name.strip_prefix('%') {
            Module::FlipFlop(FlipFlop {
                name,
                state: Cell::new(FlipFlopState::Off),
                destinations,
            })
        } else if let Some(name) = name.strip_prefix('&') {
            Module::Conjunction(Conjunction {
                name,
                inputs: Inputs::default(),
                destinations,
            })
        } else {
            unreachable!("Invalid input: {s}");
        }
    }

    fn name(&self) -> Name<'name> {
        match self {
            Module::Broadcaster(m) => m.name,
            Module::FlipFlop(m) => m.name,
            Module::Conjunction(m) => m.name,
        }
    }

    fn destinations(&self) -> impl Iterator<Item = &Name<'name>> {
        match self {
            Module::Broadcaster(m) => m.destinations.iter(),
            Module::FlipFlop(m) => m.destinations.iter(),
            Module::Conjunction(m) => m.destinations.iter(),
        }
    }

    fn recv(&self, transmission: Transmission) -> Box<dyn Iterator<Item = Transmission> + '_> {
        match self {
            Module::Broadcaster(module) => Box::new(module.recv(transmission)),
            Module::FlipFlop(module) => module.recv(transmission),
            Module::Conjunction(module) => Box::new(module.recv(transmission)),
        }
    }
}

type ModuleConfiguration<'modules> = HashMap<&'modules str, Module<'modules>>;

fn parse_input(input: &str) -> ModuleConfiguration<'_> {
    let mut inputs: BTreeMap<Name, Inputs> = BTreeMap::new();
    let mut modules = ModuleConfiguration::new();

    for line in input.lines() {
        let module = Module::new(line);
        let name = module.name();
        for destination in module.destinations() {
            inputs
                .entry(destination)
                .or_default()
                .insert(name, Cell::new(Pulse::Low));
        }
        modules.insert(name, module);
    }

    // We need to add the input to the conjunction modules.
    for (name, others) in inputs {
        if let Some(Module::Conjunction(module)) = modules.get_mut(name) {
            module.inputs = others;
        }
    }

    modules
}

fn part1(module_configuration: ModuleConfiguration, button_presses: u64) -> u64 {
    let mut low_pulses = 0;
    let mut high_pulses = 0;

    for _ in 0..button_presses {
        let mut transmissions = VecDeque::from([Transmission::default()]);

        while let Some(transmission) = transmissions.pop_front() {
            match transmission.pulse {
                Pulse::Low => low_pulses += 1,
                Pulse::High => high_pulses += 1,
            }

            if let Some(module) = module_configuration.get(transmission.destination) {
                transmissions.extend(module.recv(transmission));
            }
        }
    }

    low_pulses * high_pulses
}

fn part2(module_configuration: ModuleConfiguration) -> u64 {
    // Hard-coding it a bit, but we do what we have to do.
    // Looking at the input data, for 'rx' to receive a low pulse, the conjunction that has it as a
    // destination module needs to receive high pulses on each of its inputs. Assuming these are on
    // a cycle, let's find how many cycles for each to return to sending a high pulse to the
    // conjunction module, then find the lowest common multiple of these periods.
    if let Some(Module::Conjunction(module)) = module_configuration
        .values()
        .find(|module| module.destinations().any(|d| d == &"rx"))
    {
        let targets: BTreeSet<Name> = module.inputs.keys().copied().collect();
        let mut periods = BTreeMap::new();
        let mut transmissions = VecDeque::new();

        for counter in 1.. {
            transmissions.push_back(Transmission::default());

            while let Some(transmission) = transmissions.pop_front() {
                if transmission.pulse == Pulse::High && targets.contains(transmission.origin) {
                    periods.entry(transmission.origin).or_insert(counter);

                    // We have found all necessary periods.
                    if periods.len() == targets.len() {
                        return periods.into_values().fold(1, lcm);
                    }
                }

                if let Some(module) = module_configuration.get(transmission.destination) {
                    transmissions.extend(module.recv(transmission));
                }
            }
        }
    }

    0
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT)?;
    let module_configuration = parse_input(&input);

    println!(
        "The first answer is: {}",
        part1(module_configuration.clone(), 1000)
    );
    println!("The second answer is: {}", part2(module_configuration));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "\
        broadcaster -> a, b, c\n\
        %a -> b\n\
        %b -> c\n\
        %c -> inv\n\
        &inv -> a\n\
    ";

    const EXAMPLE_2: &str = "\
        broadcaster -> a\n\
        %a -> inv, con\n\
        &inv -> b\n\
        %b -> con\n\
        &con -> output\n\
    ";

    #[test]
    fn test_part1_ex1() {
        let configuration = parse_input(EXAMPLE_1);
        let actual = part1(configuration, 1000);
        let expected = 32000000;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part1_ex2() {
        let configuration = parse_input(EXAMPLE_2);
        let actual = part1(configuration, 1000);
        let expected = 11687500;

        assert_eq!(expected, actual);
    }
}
