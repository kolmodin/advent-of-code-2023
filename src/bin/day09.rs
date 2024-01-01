use std::fs;

use itertools::izip;

fn parse_line(s: &str) -> Vec<i32> {
    s.split(' ')
        .map(|num| num.parse::<i32>().unwrap())
        .collect()
}

fn next_history(seq: &[i32]) -> i32 {
    let mut seqs = vec![];
    seqs.push(Vec::from(seq));

    while !seqs.last().unwrap().iter().all(|n| *n == 0) {
        let last = seqs.last().unwrap();

        let next = izip!(last.iter(), last.iter().skip(1))
            .map(|(a, b)| b - a)
            .collect::<Vec<_>>();

        seqs.push(next);
    }

    seqs.last_mut().unwrap().push(0);

    let mut last_diff = 0;
    for row in (0..=seqs.len()-2).rev() {
        last_diff += seqs[row].last().unwrap();
        seqs[row].push(last_diff);
    }

    return *seqs.first().unwrap().last().unwrap();
}

fn first_history(seq: &[i32]) -> i32 {
    let rev = seq.iter().rev().cloned().collect::<Vec<_>>();
    next_history(&rev)
}

fn main() {
    let contents =
        fs::read_to_string("inputs/day09.txt").expect("Something went wrong reading the file");

    let histories = contents.lines().map(parse_line).collect::<Vec<_>>();

    let nexts = histories.iter().map(|h| next_history(h)).sum::<i32>();
    let firsts = histories.iter().map(|h| first_history(h)).sum::<i32>();

    println!("Part 1: {}", nexts);
    println!("Part 2: {}", firsts);
}
