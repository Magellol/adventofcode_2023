use lib::lines_from_file;

// TODO I still need to handle parsing
fn day6(input: Vec<String>) -> i64 {
    let races: [(i64, i64); 1] = [(60808676, 601116315591300)];

    races.iter().fold(1, |acc, (time, distance)| {
        let range = 0..*time + 1;

        let result = range.fold(0, |acc2, ms| {
            let remaining = time - ms;
            let current_distance = ms * remaining;

            if &current_distance > distance {
                acc2 + 1
            } else {
                acc2
            }
        });

        acc * result
    })
}

#[test]
fn day6_test() {
    let input = vec![
        "Time:      7  15   30".to_string(),
        "Distance:  9  40  200".to_string(),
    ];

    assert_eq!(day6(input), 288)
}

fn main() {
    let contents = lib::lines_from_file("./day6/input.txt");

    println!("{}", day6(contents));
}
