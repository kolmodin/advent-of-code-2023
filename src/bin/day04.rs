use std::collections::HashSet;
use std::fs;

#[derive(Debug)]
struct Card {
    id: usize,
    winners: Vec<i32>,
    my_numbers: Vec<i32>,
}

impl Card {
    fn from_line(s: &str) -> Card {
        let (card, numbers) = s.split_once(':').unwrap();
        let (winners, mine) = numbers.split_once('|').unwrap();

        let parse = |ln: &str| {
            ln.trim()
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        };

        Card {
            id: card.split(' ').last().unwrap().parse::<usize>().unwrap(),
            winners: parse(winners),
            my_numbers: parse(mine),
        }
    }

    fn count_matches(&self) -> usize {
        let w = self.winners.clone().into_iter().collect::<HashSet<_>>();
        let m = self.my_numbers.clone().into_iter().collect();
        w.intersection(&m).collect::<Vec<_>>().len()
    }

    fn points(&self) -> i32 {
        let w = self.winners.clone().into_iter().collect::<HashSet<_>>();
        let m = self.my_numbers.clone().into_iter().collect();
        let matches: Vec<_> = w.intersection(&m).collect();

        if matches.is_empty() {
            return 0;
        }
        2_i32.pow((matches.len() - 1) as u32) 
    }
}

fn count_copies(cards: &[Card]) -> i32 {
    let mut visited = vec![0; cards.len()];

    for card in cards.iter().rev() {
        visited[card.id - 1] = 1 + &visited[card.id..card.id + card.count_matches()].iter().sum::<i32>();
    }

    visited.iter().sum::<i32>()
}

fn main() {
    let contents =
        fs::read_to_string("inputs/day04.txt").expect("Something went wrong reading the file");

    let cards = contents.lines().map(Card::from_line).collect::<Vec<_>>();

    println!("Part 1: {}", cards.iter().map(Card::points).sum::<i32>());
    println!("Part 2: {}", count_copies(&cards));
}
