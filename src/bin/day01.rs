use std::fs;

static NUM_WORDS: [(&str, u32); 20] = [
    ("0", 0),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
    ("zero", 0),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn do_line2(s: &str, dict: &[(&str, u32)]) -> u32 {
    let mut nums = None;

    'outer: for i in 0..s.len() {
        let ss = &s[i..];

        for (snum, n) in dict {
            if ss.starts_with(snum) {
                nums = match nums {
                    None => Some((n, n)),
                    Some((fst, _)) => Some((fst, n)),
                };
                continue 'outer;
            }
        }
    }
    let (fst, last) = nums.unwrap();
    return fst * 10 + last;
}

fn main() {
    let contents =
        fs::read_to_string("inputs/day01.txt").expect("Something went wrong reading the file");

    let sum = contents
        .lines()
        .map(|s| do_line2(s, &NUM_WORDS[0..10]))
        .sum::<u32>();
    let sum2 = contents
        .lines()
        .map(|s| do_line2(s, &NUM_WORDS))
        .sum::<u32>();

    println!("Part 1: {}", sum);
    println!("Part 2: {}", sum2);
}
