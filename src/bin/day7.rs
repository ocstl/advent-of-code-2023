use std::cmp::Ordering;
use std::str::FromStr;

const INPUT: &str = "./input/day7.txt";
const HAND_SIZE: usize = 5;

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
enum Card {
    Joker = 0,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

impl TryFrom<char> for Card {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '2' => Ok(Card::Two),
            '3' => Ok(Card::Three),
            '4' => Ok(Card::Four),
            '5' => Ok(Card::Five),
            '6' => Ok(Card::Six),
            '7' => Ok(Card::Seven),
            '8' => Ok(Card::Eight),
            '9' => Ok(Card::Nine),
            'T' => Ok(Card::Ten),
            'J' => Ok(Card::Jack),
            'Q' => Ok(Card::Queen),
            'K' => Ok(Card::King),
            'A' => Ok(Card::Ace),
            _ => Err(value),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl From<&[Card; HAND_SIZE]> for HandType {
    fn from(value: &[Card; HAND_SIZE]) -> Self {
        // Consider the jokers separately.
        let mut counter = [0; 15];
        let mut jokers = 0;
        for &card in value {
            if card == Card::Joker {
                jokers += 1;
            } else {
                counter[card as usize] += 1;
            }
        }

        counter.sort_unstable();
        // We can add the jokers to the largest count, since three of a kind is better than
        // two pairs, and so on.
        counter[14] += jokers;

        match counter {
            [.., 5] => HandType::FiveOfAKind,
            [.., 4] => HandType::FourOfAKind,
            [.., 2, 3] => HandType::FullHouse,
            [.., 3] => HandType::ThreeOfAKind,
            [.., 2, 2] => HandType::TwoPairs,
            [.., 2] => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Hand {
    hand_type: HandType,
    hand: [Card; HAND_SIZE],
    bid: u32,
}

impl Hand {
    fn jacks_to_jokers(mut self) -> Self {
        for card in self.hand.iter_mut() {
            if *card == Card::Jack {
                *card = Card::Joker;
            }
        }

        self.hand_type = HandType::from(&self.hand);
        self
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand_type
            .cmp(&other.hand_type)
            .then(self.hand.cmp(&other.hand))
    }
}

impl FromStr for Hand {
    type Err = char;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s.trim().split_once(' ').expect("Bad format.");
        let mut hand = [Card::Ace; HAND_SIZE];
        for (idx, card) in cards.char_indices() {
            hand[idx] = Card::try_from(card)?;
        }

        let hand_type = HandType::from(&hand);
        let bid = bid.parse().expect("Not a number.");

        Ok(Hand {
            hand_type,
            hand,
            bid,
        })
    }
}

fn part1(hands: &[Hand]) -> u32 {
    let mut hands = hands.to_vec();
    hands.sort_unstable();

    hands
        .into_iter()
        .enumerate()
        .map(|(idx, hand)| ((idx + 1) as u32) * hand.bid)
        .sum()
}

fn part2(hands: &[Hand]) -> u32 {
    // Simply replace the jacks with jokers.
    let hands: Vec<Hand> = hands.iter().map(|hand| hand.jacks_to_jokers()).collect();
    part1(&hands)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT)?;
    let hands = input
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<Hand>, _>>()
        .expect("Bad format.");

    println!("The first answer is: {}", part1(&hands));
    println!("The second answer is: {}", part2(&hands));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        32T3K 765\n\
        T55J5 684\n\
        KK677 28\n\
        KTJJT 220\n\
        QQQJA 483\n\
    ";

    #[test]
    fn test_part1() {
        let hands = EXAMPLE
            .lines()
            .map(str::parse)
            .collect::<Result<Vec<Hand>, _>>()
            .unwrap();
        let actual = part1(&hands);
        let expected = 6440;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2() {
        let hands = EXAMPLE
            .lines()
            .map(str::parse)
            .collect::<Result<Vec<Hand>, _>>()
            .unwrap();

        let actual = part2(&hands);
        let expected = 5905;

        assert_eq!(expected, actual);
    }
}
