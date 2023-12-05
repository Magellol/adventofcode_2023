use regex::Regex;

fn day3(input: Vec<String>) -> i32 {
    let n_re = Regex::new(r"\d+").unwrap();
    let symbol_re = Regex::new(r"\*+").unwrap();

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

            let default_value = String::new();

            let prev_line = previous_index
                .and_then(|i| input.get(i))
                .unwrap_or(&default_value);
            let next_line = next_index
                .and_then(|i| input.get(i))
                .unwrap_or(&default_value);

            symbol_re
                .find_iter(line)
                .map(|symbol_m| {
                    let current_iter = n_re
                        .find_iter(line)
                        .chain(n_re.find_iter(prev_line))
                        .chain(n_re.find_iter(next_line));

                    let result = current_iter
                        .filter_map(|n_match| {
                            // * Apologies for the following code. I'm 100% certain there is a cleaner way but also I'm 100% tired.
                            // ...123
                            // ..*..
                            if (n_match.start() == symbol_m.end())
                            // 123.
                            // ...*..
                                || (n_match.end() == symbol_m.start())
                            // 123
                            // *..
                                || (n_match.start() == symbol_m.start())
                            
                            // 123.
                            //   *.
                                || (n_match.end() == symbol_m.end())

                            // 123.
                            // .*..
                                || (n_match.start() == symbol_m.start().saturating_sub(1))
                            {
                                n_match.as_str().parse::<i32>().ok()
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<i32>>();

                    if result.len() == 2 {
                        result.iter().fold(1, |acc, n| acc * n)
                    } else {
                        0
                    }
                })
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

    assert_eq!(day3(input), 467835);
}

fn main() {
    let contents = lib::lines_from_file("./day3/input.txt");

    println!("{}", day3(contents));
}
