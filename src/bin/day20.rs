use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::fmt::Display;
use std::fs;

#[derive(Debug, Clone)]
struct ParseError(String);
impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "parse error: {}", &self.0)
    }
}
impl Error for ParseError {}

#[derive(Debug, PartialEq, Eq)]
enum Kind {
    FlipFlop,
    Conjunction,
    BroadCaster,
}

type Label = str;

fn parse_line(s: &str) -> Result<(Kind, &Label, Vec<&Label>), ParseError> {
    let s = s.trim();
    if let Some((label, cont)) = s.trim().split_once(" -> ") {
        let mut label = label.chars();
        let (kind, label) = match label.next() {
            Some('%') => (Kind::FlipFlop, label.as_str()),
            Some('&') => (Kind::Conjunction, label.as_str()),
            Some('b') if label.as_str() == "roadcaster" => (Kind::BroadCaster, "broadcaster"),
            _ => return Err(ParseError(format!("unparsable first char on line: {}", s))),
        };

        return Ok((kind, label, cont.split(", ").collect_vec()));
    }

    Err(ParseError(format!("unparsable line: {}", s)))
}

#[derive(Debug)]
enum State<'a> {
    BroadCaster,
    FlipFlop(bool),
    Conjunction(Vec<(&'a Label, bool)>),
}

impl<'a> State<'a> {
    fn signal(&mut self, sender: &'a Label, pulse: bool) -> Option<bool> {
        match self {
            State::BroadCaster => Some(pulse),
            State::FlipFlop(ref mut state) => {
                if pulse {
                    return None;
                }
                *state = !*state;
                Some(*state)
            }
            State::Conjunction(ref mut inputs) => {
                for (label, ref mut state) in inputs.iter_mut() {
                    if sender == *label {
                        *state = pulse;
                    }
                }

                Some(!inputs.iter().all(|(_, s)| *s))
            }
        }
    }
}

struct Game<'a> {
    queue: VecDeque<(&'a Label, &'a Label, bool)>,
    modules: HashMap<&'a Label, (&'a [&'a Label], State<'a>)>,
    high_pulses_sent: i32,
    low_pulses_sent: i32,
    iterations: i64,
    remaining_modules: Vec<&'static str>,
    modules_iteration: Vec<i64>,
}

impl<'a> Game<'a> {
    fn new(
        input: &'a [(Kind, &str, Vec<&str>)],
        input_map: &'a HashMap<&Label, Vec<&Label>>,
    ) -> Self {
        let modules = input
            .iter()
            .map(|(kind, label, outputs)| {
                (
                    *label,
                    (
                        &outputs[..],
                        match kind {
                            Kind::BroadCaster => State::BroadCaster,
                            Kind::FlipFlop => State::FlipFlop(false),
                            Kind::Conjunction => State::Conjunction(
                                input_map
                                    .get(*label)
                                    .unwrap()
                                    .iter()
                                    .map(|input| (*input, false))
                                    .collect(),
                            ),
                        },
                    ),
                )
            })
            .collect();

        Game {
            modules,
            queue: VecDeque::new(),
            high_pulses_sent: 0,
            low_pulses_sent: 0,
            remaining_modules: vec!["dh", "qd", "bb", "dp"],
            modules_iteration: vec![],
            iterations: 0,
        }
    }

    fn press_button(&mut self) {
        self.iterations += 1;
        self.queue.push_back(("button", "broadcaster", false));
    }

    fn process(&mut self) {
        while let Some((sender, receiver, pulse)) = self.queue.pop_front() {
            if pulse {
                self.high_pulses_sent += 1;
            } else {
                self.low_pulses_sent += 1;
            }

            if !pulse && self.remaining_modules.contains(&receiver) {
                println!("found key {} at iteration {}", receiver, self.iterations);
                self.remaining_modules.retain(|l| *l != receiver);
                self.modules_iteration.push(self.iterations);
            }

            if let Some((outputs, module)) = self.modules.get_mut(receiver) {
                if let Some(out_pulse) = module.signal(sender, pulse) {
                    for output in outputs.iter() {
                        self.queue.push_back((receiver, output, out_pulse));
                    }
                }
            }
        }
    }

    fn solve_part_2(&mut self) {
        while !self.remaining_modules.is_empty() {
            self.press_button();
            self.process();
        }
    }
}

fn input_map<'a>(modules: &'a [(Kind, &Label, Vec<&Label>)]) -> HashMap<&'a Label, Vec<&'a Label>> {
    let mut result = HashMap::new();

    for (_, input, outputs) in modules {
        for &output in outputs {
            result.entry(output).or_insert(Vec::new()).push(*input);
        }
    }

    result
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("inputs/day20.txt").expect("Could not read input");

    let input = contents
        .lines()
        .map(parse_line)
        .collect::<Result<Vec<_>, ParseError>>()?;

    let input_map = input_map(&input);

    let mut game = Game::new(&input, &input_map);

    for _ in 0..1000 {
        game.press_button();
        game.process();
    }
    println!("Part 1: {}", game.high_pulses_sent * game.low_pulses_sent);

    game.solve_part_2();
    println!(
        "Part 2: {}",
        game.modules_iteration.iter().product::<i64>()
    );

    Ok(())
}
