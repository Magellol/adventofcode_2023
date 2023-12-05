use lib::lines_from_file;
use num_traits::pow;

fn day4(input: Vec<String>) -> i32 {
    input.iter().fold(0, |acc, line| {
        let result = line.find(":").map(|i| {
            let split = &line[(i + 1)..];
            split
                .trim()
                .splitn(2, "|")
                .map(|x| x.split_whitespace().map(|x| x.parse().unwrap()).collect())
                .collect::<Vec<Vec<i32>>>()
        });

        let result = if let Some([left, right]) = result.map(|x| x.as_slice().to_owned()).as_deref()
        {
            let matches = right.iter().filter(|x| left.contains(x)).count();

            if matches > 0 {
                // e.g 1 * 2^3
                1 * pow(2, matches - 1)
            } else {
                0
            }
        } else {
            0
        };

        acc + result
    })
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

    assert_eq!(day4(input), 13);
}

fn main() {
    let contents = lines_from_file("./day4/input.txt");

    println!("{}", day4(contents))
}
