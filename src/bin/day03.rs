use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Num {
    row: i32,
    start: i32,
    end: i32,
    num: i32,
}

impl Num {
    fn from_line(row: i32, s: &str) -> Vec<Num> {
        let bytes = s.as_bytes();
        let mut nums = vec![];

        let mut ix = 0;
        while ix < bytes.len() {
            if bytes[ix].is_ascii_digit() {
                let start = ix as i32;

                let mut num: i32 = 0;
                while ix < bytes.len() && bytes[ix].is_ascii_digit() {
                    num = num * 10 + Into::<i32>::into(bytes[ix] - b'0');
                    ix += 1;
                }

                nums.push(Num {
                    row,
                    start,
                    end: ix as i32,
                    num,
                });
            }

            ix += 1;
        }

        nums
    }

    fn edge(&self) -> Vec<Pos2> {
        let mut res = vec![];

        for col in (self.start - 1)..=self.end {
            res.push(Pos2 {
                x: col,
                y: self.row - 1,
            });
            if col < self.start || col >= self.end {
                res.push(Pos2 {
                    x: col,
                    y: self.row,
                });
            }
            res.push(Pos2 {
                x: col,
                y: self.row + 1,
            });
        }

        res
    }

    fn is_near_symbol(self: &Num, symbols: &HashMap<Pos2, Symbol>) -> bool {
        self.edge().iter().any(|p| symbols.contains_key(p))
    }
}

struct Symbol {
    pos: Pos2,
    sign: u8,
}

impl Symbol {
    fn from_line(row: i32, s: &str) -> Vec<Symbol> {
        s.as_bytes()
            .iter()
            .enumerate()
            .filter_map(|(pos, symbol)| {
                if *symbol != b'.' && !symbol.is_ascii_digit() {
                    Some(Symbol {
                        pos: Pos2 {
                            x: pos as i32,
                            y: row,
                        },
                        sign: *symbol,
                    })
                } else {
                    None
                }
            })
            .collect()
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Pos2 {
    x: i32,
    y: i32,
}

fn main() {
    let contents =
        fs::read_to_string("inputs/day03.txt").expect("Something went wrong reading the file");

    let nums: Vec<_> = contents
        .lines()
        .enumerate()
        .flat_map(|(row, ln)| Num::from_line(row as i32, ln))
        .collect();

    let symbols: HashMap<Pos2, Symbol> = contents
        .lines()
        .enumerate()
        .flat_map(|(row, ln)| {
            Symbol::from_line(row as i32, ln)
                .into_iter()
                .map(move |s| (s.pos, s))
        })
        .collect();

    let part1 = nums
        .iter()
        .filter_map(|n| {
            if n.is_near_symbol(&symbols) {
                Some(n.num)
            } else {
                None
            }
        })
        .sum::<i32>();

    println!("Part 1: {}", part1);

    let mut nums_map = HashMap::new();
    for n in &nums {
        for e in n.edge() {
            nums_map.entry(e).or_insert(Vec::new()).push(n.num);
        }
    }

    let part2 = symbols
        .iter()
        .filter_map(|(pos, symbol)| {
            if symbol.sign == b'*' {
                if let Some(v) = nums_map.get(pos) {
                    if v.len() == 2 {
                        return Some(v.iter().product::<i32>());
                    }
                }
            }
            None
        })
        .sum::<i32>();

    println!("Part 2: {}", part2);
}
