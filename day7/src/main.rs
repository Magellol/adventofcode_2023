use std::{collections::HashMap, str::FromStr};

use lib::lines_from_file;

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
enum Strengh {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPairs,
    OnePair,
    HighCard,
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Card::A),
            "K" => Ok(Card::K),
            "Q" => Ok(Card::Q),
            "J" => Ok(Card::J),
            "T" => Ok(Card::T),
            "9" => Ok(Card::Nine),
            "8" => Ok(Card::Eight),
            "7" => Ok(Card::Seven),
            "6" => Ok(Card::Six),
            "5" => Ok(Card::Five),
            "4" => Ok(Card::Four),
            "3" => Ok(Card::Three),
            "2" => Ok(Card::Two),
            _ => Err(()),
        }
    }
}

fn process(fa: &Vec<(&char, &i32)>) -> Option<Strengh> {
    fa.split_first().and_then(|(head, tail)| {
        let (_, count) = head;
        let tail_split = tail.split_first();

        match count {
            5 => Some(Strengh::FiveOfAKind),
            4 => Some(Strengh::FourOfAKind),
            3 => {
                tail_split.and_then(|((_, count), _tail)| {
                    // There is probably a type-safe way of dealing with it but it's Thursday morning pre-coffee time and I have a full-time job.
                    match count {
                        2 => Some(Strengh::FullHouse),
                        1 => Some(Strengh::ThreeOfAKind),
                        _ => None,
                    }
                })
            }
            2 => tail_split.and_then(|((_, count), _tail)| match count {
                2 => Some(Strengh::TwoPairs),
                1 => Some(Strengh::OnePair),
                _ => None,
            }),
            _ => Some(Strengh::HighCard),
        }
    })
}

// TODO I still need to handle parsing
fn day7(input: Vec<String>) -> i32 {
    let mut x = input
        .iter()
        .filter_map(|_str| {
            let split = _str.split(" ").collect::<Vec<&str>>();
            let result = match split.as_slice() {
                [a, b] => b.parse::<i32>().ok().map(|x| (a.to_owned(), x)),
                _ => None,
            };

            let strength = result.and_then(|(cards, _)| {
                let mut hash: HashMap<char, i32> = HashMap::new();

                for c in cards.chars() {
                    let prev = hash.get(&c).unwrap_or(&0);

                    hash.insert(c, prev + 1);
                }

                let mut sorted = hash.iter().collect::<Vec<(&char, &i32)>>();
                sorted.sort_by(|a, b| b.1.cmp(&a.1));

                process(&sorted)
            });

            result.zip(strength)
        })
        .collect::<Vec<((&str, i32), Strengh)>>();

    x.sort_by(|((turn_a, _), strength_a), ((turn_b, _), strength_b)| {
        let ord = strength_a.cmp(strength_b);

        if ord.is_eq() {
            for (i, c) in turn_a.chars().enumerate() {
                let a = Card::from_str(&c.to_string()).unwrap();
                let b = Card::from_str(&turn_b.chars().nth(i).unwrap().to_string()).unwrap();
                let ord = a.cmp(&b);

                if ord.is_eq() {
                    continue;
                } else {
                    return ord;
                }
            }

            // Like I said...Thursday morning pre-coffee; gotta work after.
            unreachable!()
        } else {
            ord
        }
    });

    x.iter().enumerate().fold(0, |acc, (i, ((_, bid), _))| {
        let rank = x.len() - i;

        acc + rank as i32 * bid
    })
}

#[test]
fn day7_test() {
    let input = vec![
        "32T3K 765".to_string(),
        "T55J5 684".to_string(),
        "KK677 28".to_string(),
        "KTJJT 220".to_string(),
        "QQQJA 483".to_string(),
    ];

    assert_eq!(day7(input), 6440)
}

fn main() {
    let contents = lines_from_file("./day7/input.txt");

    println!("{}", day7(contents));
}
