use lib::lines_from_file;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
enum Color {
    R,
    G,
    B,
}

impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Color, Self::Err> {
        match s {
            "red" => Ok(Color::R),
            "green" => Ok(Color::G),
            "blue" => Ok(Color::B),
            _ => Err(()),
        }
    }
}

type Turn = HashMap<Color, i32>;

// only 12 red cubes, 13 green cubes, and 14 blue cubes
fn day2(input: Vec<String>, max: Turn) -> i32 {
    let turn_re = Regex::new(r"(\d+) (\w+)").unwrap();

    input
        .into_iter()
        .filter_map(|s| {
            let (id, turn_str) = sscanf::sscanf!(s, "Game {i32}:{str}").unwrap();

            // Turn: 8 green, 6 blue, 20 red
            let impossible_turns: Vec<(i32, Color)> = turn_re
                .find_iter(turn_str)
                .filter_map(|m| {
                    let match_ = turn_re.captures(m.as_str()).unwrap();
                    let n = match_.get(1).and_then(|x| x.as_str().parse::<i32>().ok());
                    let color = match_.get(2).and_then(|x| Color::from_str(x.as_str()).ok());

                    n.zip(color).filter(|(n, color)| n > &max[color])
                })
                .collect();

            if impossible_turns.len() > 0 {
                None
            } else {
                Some(id)
            }
        })
        .sum()
}

#[test]
fn day2_test() {
    let max: Turn = HashMap::from([(Color::R, 12), (Color::G, 13), (Color::B, 14)]);
    let input = vec![
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string(),
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string(),
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string(),
        "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string(),
    ];

    assert_eq!(day2(input, max), 8);
}

fn main() {
    let max: Turn = HashMap::from([(Color::R, 12), (Color::G, 13), (Color::B, 14)]);
    let contents = lines_from_file("./day2/input.txt");

    println!("{}", day2(contents, max))
}
