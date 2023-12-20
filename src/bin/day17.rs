use priority_queue::PriorityQueue;
use std::collections::HashMap;
use std::fs;
use std::hash::Hash;

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
struct Pos2 {
    y: i32,
    x: i32,
}

impl Pos2 {
    fn forward(&self, dir: Dir) -> Pos2 {
        let Pos2 { y, x } = dir.to_pos();
        Pos2 {
            y: self.y + y,
            x: self.x + x,
        }
    }

    fn manhattan(&self, other: &Self) -> i32 {
        (self.x.abs_diff(other.x) + self.y.abs_diff(other.y)) as i32
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn to_pos(&self) -> Pos2 {
        match *self {
            Dir::Up => Pos2 { y: -1, x: 0 },
            Dir::Down => Pos2 { y: 1, x: 0 },
            Dir::Left => Pos2 { y: 0, x: -1 },
            Dir::Right => Pos2 { y: 0, x: 1 },
        }
    }

    fn is_opposite(&self, dir: Dir) -> bool {
        match *self {
            Dir::Up => dir == Dir::Down,
            Dir::Down => dir == Dir::Up,
            Dir::Left => dir == Dir::Right,
            Dir::Right => dir == Dir::Left,
        }
    }
}

#[derive(Clone)]
struct Grid {
    height: i32,
    width: i32,
    contents: Vec<u8>,
}

impl Grid {
    fn from_input(input: &str) -> Grid {
        let lines: Vec<_> = input.lines().collect();

        Grid {
            height: lines.len() as i32,
            width: lines[0].len() as i32,
            contents: lines.concat().into_bytes(),
        }
    }
}

impl Grid {
    fn get(&self, pos: &Pos2) -> u8 {
        debug_assert!(
            pos.x >= 0 && pos.x < self.width,
            "x={} out of bounds 0..{}",
            pos.x,
            self.width
        );
        debug_assert!(
            pos.y >= 0 && pos.y < self.height,
            "y={} out of bounds 0..{}",
            pos.y,
            self.height
        );

        let ix = pos.y * self.width + pos.x;
        self.contents[ix as usize]
    }
}

type HeatLoss = i32;

#[derive(Debug, Hash, PartialEq, Eq)]
struct Node {
    pos: Pos2,
    dir: Dir,
    moves_in_dir: u8,
    heat_loss: HeatLoss,
}

struct Search<'a, T>
where
    T: SearchNode + Hash + Eq,
{
    grid: &'a Grid,
    visited: HashMap<T::Rep, HeatLoss>,
    queue: PriorityQueue<T, HeatLoss>,
}

impl<'a, T: SearchNode + Hash + Eq> Search<'a, T> {
    fn new(grid: &'a Grid, init: T) -> Self {
        let mut search = Search {
            grid: &grid,
            visited: HashMap::new(),
            queue: PriorityQueue::new(),
        };
        search.queue.push(init, 0);
        search
    }
    fn search(mut self) -> Option<T> {
        let target_pos = Pos2 {
            x: self.grid.width - 1,
            y: self.grid.height - 1,
        };

        let mut processed = 0;
        let mut pruned = 0;

        while let Some((node, _)) = self.queue.pop() {
            processed += 1;
            if node.is_legally_at_goal(&target_pos) {
                println!(
                    "{} processed, {} pruned, {} still in queue",
                    processed,
                    pruned,
                    self.queue.len()
                );
                return Some(node);
            }

            if let Some(prior_heat_loss) = self.visited.get(&node.rep()) {
                if *prior_heat_loss < node.heat_loss() {
                    pruned += 1;
                    continue;
                }
            }

            self.visited.insert(node.rep(), node.heat_loss());

            for node in node.next(&self.grid) {
                let cost = node.cost(&target_pos);
                self.queue.push(node, -cost);
            }
        }
        None
    }
}

trait SearchNode {
    type Rep: Hash + PartialEq + Eq;

    fn is_legally_at_goal(&self, pos: &Pos2) -> bool;
    fn rep(&self) -> Self::Rep;
    fn heat_loss(&self) -> i32;
    fn cost(&self, target: &Pos2) -> i32;
    fn next(&self, grid: &Grid) -> Vec<Self>
    where
        Self: Sized;
}

#[derive(Hash, PartialEq, Eq)]
struct NodePart1(Node);

impl SearchNode for NodePart1 {
    type Rep = (Pos2, Dir, u8);

    fn is_legally_at_goal(&self, pos: &Pos2) -> bool {
        self.0.pos == *pos
    }

    fn rep(&self) -> Self::Rep {
        (self.0.pos, self.0.dir, self.0.moves_in_dir)
    }

    fn heat_loss(&self) -> i32 {
        self.0.heat_loss
    }

    fn cost(&self, target: &Pos2) -> i32 {
        self.0.heat_loss + self.0.pos.manhattan(target)
    }

    fn next(&self, grid: &Grid) -> Vec<Self> {
        let mut result = vec![];
        let node = &self.0;

        for dir in [Dir::Up, Dir::Left, Dir::Down, Dir::Right] {
            let new_pos = node.pos.forward(dir);
            let moves_in_dir = if node.dir == dir {
                node.moves_in_dir + 1
            } else {
                1
            };
            if node.dir.is_opposite(dir)
                || new_pos.x < 0
                || new_pos.x >= grid.width
                || new_pos.y < 0
                || new_pos.y >= grid.height
                || moves_in_dir > 3
            {
                continue;
            }

            result.push(NodePart1(Node {
                pos: new_pos,
                dir,
                moves_in_dir,
                heat_loss: node.heat_loss + (grid.get(&new_pos) - b'0') as i32,
            }));
        }
        result
    }
}

#[derive(Hash, PartialEq, Eq)]
struct NodePart2(Node);

impl SearchNode for NodePart2 {
    type Rep = (Pos2, Dir, u8);

    fn is_legally_at_goal(&self, pos: &Pos2) -> bool {
        self.0.pos == *pos && self.0.moves_in_dir >= 4
    }

    fn rep(&self) -> Self::Rep {
        (self.0.pos, self.0.dir, self.0.moves_in_dir)
    }

    fn heat_loss(&self) -> i32 {
        self.0.heat_loss
    }

    fn cost(&self, target: &Pos2) -> i32 {
        self.0.heat_loss + self.0.pos.manhattan(target)
    }

    fn next(&self, grid: &Grid) -> Vec<Self> {
        let mut result = vec![];

        let node = &self.0;

        for dir in [Dir::Up, Dir::Left, Dir::Down, Dir::Right] {
            let pos = node.pos.forward(dir);
            if node.dir != dir && node.moves_in_dir < 4
                || node.dir == dir && node.moves_in_dir >= 10
                || node.dir.is_opposite(dir)
                || pos.x < 0
                || pos.x >= grid.width
                || pos.y < 0
                || pos.y >= grid.height
            {
                continue;
            }
            let heat_loss = node.heat_loss + (grid.get(&pos) - b'0') as i32;
            let moves_in_dir = if node.dir == dir {
                node.moves_in_dir + 1
            } else {
                1
            };
            let node = NodePart2(Node {
                pos,
                dir,
                moves_in_dir,
                heat_loss,
            });
            result.push(node);
        }

        result
    }
}

fn main() {
    let contents = fs::read_to_string("inputs/day17.txt").expect("Could not read input");

    let grid = Grid::from_input(&contents);

    let search = Search::new(
        &grid,
        NodePart1(Node {
            pos: Pos2 { x: 0, y: 0 },
            dir: Dir::Right,
            moves_in_dir: 0,
            heat_loss: 0,
        }),
    );

    println!("Part 1: {}", search.search().unwrap().heat_loss());

    let search = Search::new(
        &grid,
        NodePart2(Node {
            pos: Pos2 { x: 0, y: 0 },
            dir: Dir::Right,
            moves_in_dir: 0,
            heat_loss: 0,
        }),
    );
    println!("Part 2: {}", search.search().unwrap().heat_loss())
}
