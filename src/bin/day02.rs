use std::fs;
use std::vec::Vec;
use std::cmp::max;

#[derive(Debug)]
struct Hand {
    red: u32,
    green: u32,
    blue: u32,
}

impl Hand {
    fn empty() -> Self {
        Hand {red:0, green:0, blue:0}
    }

    fn is_valid(self: &Self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }

    fn minimum(a: Hand, b: &Hand) -> Hand {
        Hand {
            red: max(a.red, b.red),
            green: max(a.green, b.green),
            blue: max(a.blue, b.blue),
        }
    }

    fn power(self: &Self) -> u32 {
        self.red * self.green * self.blue
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    hands: std::vec::Vec<Hand>,
}

impl Game {
    fn from_line(s: &str) -> Game {
        let (game, cont) = s.split_once(':').unwrap();
    
        let (_, id_str) = game.split_once(' ').unwrap();
    
        let mut hands = vec![];
    
        for handful_str in cont.trim().split("; ") {
            let mut hand = Hand {
                red: 0,
                green: 0,
                blue: 0,
            };
            for cube in handful_str.split(", ") {
                let (count_str, color_str) = cube.split_once(' ').unwrap();
                let count = count_str.parse::<u32>().unwrap();
                
                match color_str {
                    "blue" => hand.blue += count,
                    "green" => hand.green += count,
                    "red" => hand.red += count,
                    _ => panic!("could not parse color {}", color_str),
                };
            }
            hands.push(hand);
        }
    
        Game {
            id: id_str.parse().unwrap(),
            hands,
        }
    }

    fn all_hands_valid(self: &Game) -> bool {
        self.hands
            .iter()
            .all(Hand::is_valid)
    }

    fn smallest_hand(self: &Game) -> Hand {
        self.hands.iter().fold(Hand::empty(), Hand::minimum)
    }
}

fn solve_part1(games: &[Game]) -> u32 {
    games
        .iter()
        .filter_map(|g| {
            if g.all_hands_valid() {
                Some(g.id)
            } else {
                None
            }
        })
        .sum::<u32>()
}

fn solve_part2(games: &[Game]) -> u32 {
    games.iter().map(|g| g.smallest_hand().power()).sum::<u32>()
}

fn main() {
    let contents =
        fs::read_to_string("inputs/day02.txt").expect("Something went wrong reading the file");

    let games = contents.lines().map(Game::from_line).collect::<Vec<_>>();

    println!("Part 1: {}", solve_part1(&games));
    println!("Part 2: {}", solve_part2(&games));
}
