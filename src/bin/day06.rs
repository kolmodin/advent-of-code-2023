use aoc2023::string;
use itertools::Itertools;
use std::fs;

fn solve(time: i64, dist: i64) -> i64 {
    let mut count = 0;

    for hold in 0..=time {
        let sum = hold * (time - hold);
        if sum > dist {
            count += 1;
        }
    }

    count
}

fn main() {
    let contents = fs::read_to_string("inputs/day06.txt").expect("Could not read input");

    println!("{contents}");

    if let Some((time, distance)) = contents
        .lines()
        .map(|s| {
            string::words(s)
                .skip(1)
                .map(|s| s.parse::<i64>().unwrap())
                .collect_vec()
        })
        .collect_tuple()
    {
        println!(
            "Part 1: {}",
            std::iter::zip(time, distance)
                .map(|(t, d)| solve(t, d))
                .product::<i64>()
        );
    }

    if let Some((time, distance)) = contents
        .lines()
        .map(|ln| {
            String::from_iter(ln.chars().filter(|c| c.is_numeric()))
                .parse::<i64>()
                .unwrap()
        })
        .collect_tuple()
    {
        println!("Part 2: {}", solve(time, distance));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(solve(7, 9), 4);
    }
}
