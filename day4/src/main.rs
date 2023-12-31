use regex::Regex;
use std::collections::HashMap;

use lib::lines_from_file;

fn get_card_id(line: &str) -> i32 {
    let re = Regex::new(r"Card\s+(\d+):").unwrap();

    re.captures(line)
        .and_then(|m| m.get(1))
        .and_then(|x| x.as_str().parse::<i32>().ok())
        .unwrap()
}

fn process(x: &Vec<String>, map: &HashMap<i32, String>) -> i32 {
    x.iter().fold(0, |acc, line| {
        // A bit of duplication here...
        let id = get_card_id(line);
        let ns = line.split(":").nth(1).unwrap();

        let split = ns
            .split("|")
            .map(|x| x.split_whitespace().map(|x| x.parse().unwrap()).collect())
            .collect::<Vec<Vec<i32>>>();

        if let [left, right] = split.as_slice() {
            let matches: Vec<String> = right
                .iter()
                .filter(|x| left.contains(x))
                .enumerate()
                .filter_map(|(i, _x)| {
                    let index = id + i as i32 + 1;
                    map.get(&index).map(|x| x.to_owned())
                })
                .collect();

            if matches.len() == 0 {
                acc
            } else {
                1 + acc + process(&matches, map)
            }
        } else {
            acc
        }
    })
}

fn day4(input: Vec<String>) -> i32 {
    let mut dict: HashMap<i32, String> = HashMap::new();

    for line in input.iter() {
        dict.insert(get_card_id(line), line.to_owned());
    }

    process(&input, &dict)
}

#[test]
fn day4_test() {
    let input = vec![
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string(),
        "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19".to_string(),
        "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1".to_string(),
        "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83".to_string(),
        "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36".to_string(),
        "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string(),
    ];

    assert_eq!(day4(input), 30);
}

fn main() {
    let contents = lines_from_file("./day4/input.txt");

    println!("{}", day4(contents))
}
