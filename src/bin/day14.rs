use std::fs;

use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::iter::successors;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Ord, PartialOrd)]
struct Pos2 {
    y: i32,
    x: i32,
}

impl Pos2 {
    fn up(self: &Self) -> Pos2 {
        Pos2 {
            y: self.y - 1,
            x: self.x,
        }
    }
}

#[derive(Hash, Copy, Clone, PartialEq, Eq)]
enum Cell {
    Empty,
    Round,
    Cube,
}

#[derive(Hash, Clone, PartialEq, Eq)]
struct Grid {
    width: i32,
    height: i32,
    contents: Vec<Cell>,
}

impl Grid {
    fn from_input(input: &str) -> Grid {
        let lines: Vec<_> = input.lines().collect();
        let width = lines[0].len() as i32;
        let height = lines.len() as i32;

        let cell = |u8| match u8 {
            b'.' => Cell::Empty,
            b'O' => Cell::Round,
            b'#' => Cell::Cube,
            _ => panic!("unknown cell"),
        };

        Grid {
            width,
            height,
            contents: lines.concat().into_bytes().into_iter().map(cell).collect(),
        }
    }

    fn get(&self, pos: &Pos2) -> Cell {
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

    fn set(&mut self, pos: &Pos2, cell: Cell) {
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
        self.contents[ix as usize] = cell;
    }

    fn tilt_north(&self) -> Grid {
        let mut grid = self.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                let pos = Pos2 { y, x };

                let cell = grid.get(&pos);
                if cell == Cell::Round {
                    let new_pos = successors(Some(pos.clone()), |p| {
                        let p = p.up();
                        if p.y >= 0 && grid.get(&p) == Cell::Empty {
                            Some(p)
                        } else {
                            None
                        }
                    })
                    .last()
                    .expect("could not place round stone");

                    grid.set(&pos, Cell::Empty);
                    grid.set(&new_pos, Cell::Round);
                }
            }
        }

        grid
    }

    fn rotate_right(self: &Self) -> Grid {
        let mut grid = self.clone();
        (grid.height, grid.width) = (grid.width, grid.height);

        for y in 0..self.height {
            for x in 0..self.width {
                let old_pos = Pos2 { y, x };
                let new_pos = Pos2 {
                    y: x,
                    x: self.width - y - 1,
                };
                grid.set(&new_pos, self.get(&old_pos));
            }
        }

        grid
    }

    fn load(self: &Self) -> i32 {
        let mut result = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = Pos2 { y, x };
                if self.get(&pos) == Cell::Round {
                    result += self.height - y;
                }
            }
        }
        result
    }
}

fn main() {
    let contents = fs::read_to_string("inputs/day14.txt").expect("Could not read input");

    let platform = Grid::from_input(&contents).tilt_north();
    println!("Part 1, load after tilt to north: {}", platform.load());

    let mut platform = platform.tilt_north();
    let mut seen = HashMap::new();
    let mut cycle = 0;
    let target_cycle = 1_000_000_000;
    while cycle < target_cycle {
        for _tilt in 0..4 {
            platform = platform.tilt_north().rotate_right();
        }
        let mut hasher = DefaultHasher::new();
        platform.hash(&mut hasher);
        let hash = hasher.finish();
        if let Some(prev) = seen.get(&hash) {
            let diff = cycle - prev;
            let jump = ((target_cycle - cycle) / diff) * diff;
            if jump > 0 {
                println!("dup hash at cycle {} is same as {}", cycle, prev);
                println!("taking shortcut, jumping {} cycles", jump);
                cycle += jump;
            }
        } else {
            seen.insert(hash, cycle);
        }
        cycle += 1;
    }
    println!(
        "Part 2, load at cycle {}: {}",
        target_cycle,
        platform.load()
    );
}
