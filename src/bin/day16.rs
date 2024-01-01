use std::{collections::HashSet, fs};

use itertools::Itertools;

#[derive(Hash, PartialEq, Eq, Copy, Clone)]
struct Pos2 {
    y: i32,
    x: i32,
}

impl Pos2 {
    fn forward(&self, dir: Dir) -> (Pos2, Dir) {
        let Pos2 { y, x } = dir.to_diff();
        (
            Pos2 {
                y: self.y + y,
                x: self.x + x,
            },
            dir,
        )
    }

    fn step(&self, dir: Dir, u8: u8) -> ((Pos2, Dir), Option<(Pos2, Dir)>) {
        match (u8, dir) {
            (b'.', dir) => (self.forward(dir), None),
            (b'|', dir) if dir == Dir::Left || dir == Dir::Right => {
                (self.forward(Dir::Up), Some(self.forward(Dir::Down)))
            }
            (b'|', dir) => (self.forward(dir), None),
            (b'-', dir) if dir == Dir::Up || dir == Dir::Down => {
                (self.forward(Dir::Left), Some(self.forward(Dir::Right)))
            }
            (b'-', dir) => (self.forward(dir), None),
            (b'\\', Dir::Right) => (self.forward(Dir::Down), None),
            (b'\\', Dir::Down) => (self.forward(Dir::Right), None),
            (b'\\', Dir::Up) => (self.forward(Dir::Left), None),
            (b'\\', Dir::Left) => (self.forward(Dir::Up), None),

            (b'/', Dir::Right) => (self.forward(Dir::Up), None),
            (b'/', Dir::Down) => (self.forward(Dir::Left), None),
            (b'/', Dir::Up) => (self.forward(Dir::Right), None),
            (b'/', Dir::Left) => (self.forward(Dir::Down), None),

            _ => panic!("{}", u8),
        }
    }
}

#[derive(Hash, PartialEq, Eq, Copy, Clone)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn to_diff(&self) -> Pos2 {
        match *self {
            Dir::Up => Pos2 { y: -1, x: 0 },
            Dir::Down => Pos2 { y: 1, x: 0 },
            Dir::Left => Pos2 { y: 0, x: -1 },
            Dir::Right => Pos2 { y: 0, x: 1 },
        }
    }
}

struct Grid<T> {
    height: i32,
    width: i32,
    contents: Vec<T>,
}

impl Grid<u8> {
    fn from_input(input: &str) -> Grid<u8> {
        let lines: Vec<_> = input.lines().collect();

        Grid {
            height: lines.len() as i32,
            width: lines[0].len() as i32,
            contents: lines.concat().into_bytes(),
        }
    }
}

impl<T> Grid<T> {
    fn get(&self, pos: &Pos2) -> &T {
        let ix = pos.y * self.width + pos.x;
        &self.contents[ix as usize]
    }
}

struct Beam<'a> {
    grid: &'a Grid<u8>,
    beams: Vec<(Pos2, Dir)>,
    visited: HashSet<(Pos2, Dir)>,
}

impl<'a> Beam<'a> {
    fn new(grid: &'a Grid<u8>) -> Beam<'a> {
        Beam {
            grid,
            beams: vec![],
            visited: HashSet::new(),
        }
    }

    fn run(&mut self, pos: Pos2, dir: Dir) {
        self.beams.push((pos, dir));

        while let Some((pos, dir)) = self.beams.pop() {
            if pos.x < 0
                || pos.x >= self.grid.width
                || pos.y < 0
                || pos.y >= self.grid.height
                || self.visited.contains(&(pos, dir))
            {
                continue;
            }
            self.visited.insert((pos, dir));
            let c = self.grid.get(&pos);
            let (next, opt_next) = pos.step(dir, *c);
            self.beams.push(next);
            opt_next.into_iter().for_each(|x| self.beams.push(x));
        }
    }

    fn energized_tile_count(&self) -> i32 {
        self.visited.iter().map(|(p, _)| *p).unique().count() as i32
    }
}

fn main() {
    let contents = fs::read_to_string("inputs/day16.txt").expect("Could not read input");

    let grid = Grid::from_input(&contents);

    let mut beam = Beam::new(&grid);

    beam.run(Pos2 { x: 0, y: 0 }, Dir::Right);

    println!("Part 1: {}", beam.energized_tile_count());

    let mut max_energy = 0;
    for (col, dir) in [(0, Dir::Right), (grid.width - 1, Dir::Left)] {
        for row in 0..grid.height {
            let mut beam = Beam::new(&grid);
            beam.run(Pos2 { x: col, y: row }, dir);
            max_energy = i32::max(max_energy, beam.energized_tile_count());
        }
    }
    for (row, dir) in [(0, Dir::Down), (grid.height - 1, Dir::Up)] {
        for col in 0..grid.width {
            let mut beam = Beam::new(&grid);
            beam.run(Pos2 { x: col, y: row }, dir);
            max_energy = i32::max(max_energy, beam.energized_tile_count());
        }
    }

    println!("Part 2: {}", max_energy);
}
