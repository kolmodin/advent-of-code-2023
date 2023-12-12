use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

#[derive(Debug)]
struct Card {
    id: i32,
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
            id: card.split(' ').last().unwrap().parse::<i32>().unwrap(),
            winners: parse(winners),
            my_numbers: parse(mine),
        }
    }

    fn count_matches(self: &Self) -> i32 {
        let w = self.winners.clone().into_iter().collect::<HashSet<_>>();
        let m = self.my_numbers.clone().into_iter().collect();
        w.intersection(&m).collect::<Vec<_>>().len() as i32
    }

    fn points(self: &Self) -> i32 {
        let w = self.winners.clone().into_iter().collect::<HashSet<_>>();
        let m = self.my_numbers.clone().into_iter().collect();
        let matches: Vec<_> = w.intersection(&m).collect();

        if matches.is_empty() {
            return 0;
        }
        2_i32.pow((matches.len() - 1) as u32) as i32
    }
}

struct ScoreCounter<'a> {
    cards: &'a [Card],
    visited: HashMap<i32, i32>,
}

impl ScoreCounter<'_> {
    fn new<'a>(cards: &'a[Card]) -> ScoreCounter<'a> {
        ScoreCounter { cards: cards, visited: HashMap::new() }
    }

    fn visit_all(self: &mut Self) -> i32 {
        self.cards.iter().map(|card|self.visit(card)).sum()
    }

    fn visit(self: &mut Self, card: &Card) -> i32 {
        if let Some(score) = self.visited.get(&card.id) {
            return *score;
        }

        let matches = card.count_matches();
        if matches == 0 {
            self.visited.insert(card.id, 1);
            return 1;
        }

        let range = (card.id + 1)..=(card.id + matches);
        let mut sum = 1;
        for id in range {
            let card = &self.cards[(id - 1) as usize];
            sum += self.visit(card);
        }

        self.visited.insert(card.id, sum);
        return sum;
    }
}

fn count_copies(cards: &[Card]) -> i32 {
    ScoreCounter::new(cards).visit_all()
}

fn main() {
    let contents =
        fs::read_to_string("inputs/day04.txt").expect("Something went wrong reading the file");

    let cards = contents.lines().map(Card::from_line).collect::<Vec<_>>();

    println!("Part 1: {}", cards.iter().map(Card::points).sum::<i32>());

    println!("Part 2: {}", count_copies(&cards));
}
