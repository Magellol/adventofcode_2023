use std::cmp;

use regex::Regex;

fn has_symbol(s: &str) -> bool {
    let re = Regex::new(r"([^\d+.]|\+)+").unwrap();

    re.is_match(s)
}

fn day3(input: Vec<String>) -> i32 {
    let n_re = Regex::new(r"\d+").unwrap();

    input
        .iter()
        .enumerate()
        .map(|(index, line)| {
            let previous_index = if index > 0 { Some(index - 1) } else { None };
            let next_index = if index < input.len() - 1 {
                Some(index + 1)
            } else {
                None
            };

            let prev_line = previous_index.and_then(|i| input.get(i));
            let next_line = next_index.and_then(|i| input.get(i));

            n_re.find_iter(line)
                .filter_map(|m| {
                    let _str = m.as_str();
                    let start = m.start().saturating_sub(1);
                    let end = cmp::min(line.len() - 1, m.end() + 1);

                    if has_symbol(&line[start..end]) {
                        Some(_str)
                    } else {
                        prev_line
                            .filter(|x| has_symbol(&x[start..end]))
                            .or_else(|| next_line.filter(|x| has_symbol(&x[start..end])))
                            .map(|_| _str)
                    }
                })
                .map(|x| x.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .sum::<i32>()
}

#[test]
fn day3_test() {
    let input = vec![
        "467..114..".to_string(),
        "...*......".to_string(),
        "..35..633.".to_string(),
        "......#...".to_string(),
        "617*......".to_string(),
        ".....+.58.".to_string(),
        "..592.....".to_string(),
        "......755.".to_string(),
        "...$.*....".to_string(),
        ".664.598..".to_string(),
    ];

    assert_eq!(day3(input), 4361);
}

fn main() {
    let contents = lib::lines_from_file("./day3/input.txt");

    println!("{}", day3(contents));
}
