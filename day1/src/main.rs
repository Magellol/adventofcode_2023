// https://adventofcode.com/2023/day/1

use fancy_regex::Regex;
use std::{
    collections::HashMap,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn day1(input: Vec<String>) -> i32 {
    let entries = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let re_str = entries
        .iter()
        .fold(vec![], |mut acc, (k, v)| {
            let fmt = format!("{}|{}", k, v);

            acc.push(fmt);
            acc
        })
        .join("|");

    let fmt = format!("(?=({}))", re_str);
    let re = Regex::new(fmt.as_str()).unwrap();

    input.iter().fold(0, |acc, s| {
        let mut start = 0;
        let mut result = vec![];
        while let Some(_match) = re.captures_from_pos(s, start).unwrap() {
            let inner = _match.get(1).unwrap();
            result.push(&s[inner.start()..inner.end()]);

            // Manually iterating to find overlapping matches.
            start = _match.get(0).unwrap().start() + 1;
        }

        let matches: Vec<i32> = result
            .iter()
            .filter_map(|matched| {
                entries
                    .get(matched)
                    .map(|x| x.to_owned())
                    .or(matched.parse::<i32>().ok())
            })
            .collect();

        let (n1, n2) = match matches.as_slice() {
            [] => (&0, &0),
            [x] => (x, x),
            [x, .., y] => (x, y),
        };

        let n = format!("{}{}", n1, n2).parse::<i32>().unwrap();

        acc + n
    })
}

#[test]
fn day1_test() {
    let input1 = vec![
        "1abc2".to_string(),
        "pqr3stu8vwx".to_string(),
        "a1b2c3d4e5f".to_string(),
        "treb7uchet".to_string(),
    ];

    let input2 = vec![
        "two1nine".to_string(),
        "eightwothree".to_string(),
        "abcone2threexyz".to_string(),
        "xtwone3four".to_string(),
        "4nineeightseven2".to_string(),
        "zoneight234".to_string(),
        "7pqrstsixteen".to_string(),
    ];

    let input3 = vec!["2rzvpfpgzxk3863eightoneighttbb".to_string()];

    assert_eq!(day1(input1), 142);
    assert_eq!(day1(input2), 281);
    assert_eq!(day1(input3), 28);
}

fn main() {
    let contents = lines_from_file("./day1/input.txt");

    println!("{}", day1(contents))
}
