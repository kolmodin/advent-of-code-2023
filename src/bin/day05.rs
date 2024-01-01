use std::fs;

#[derive(Debug)]
struct Input {
    seeds: Vec<i64>,
    seed_ranges: Vec<(i64, i64)>,
    maps: Vec<Vec<SrcDestEntry>>,
}

#[derive(Debug, Clone)]
struct SrcDestEntry {
    src: i64,
    src_end: i64,
    dest: i64,
    dest_end: i64,
}

impl Input {
    fn from_line(input: &str) -> Input {
        let mut groups = input.split("\n\n");

        let seeds = groups
            .next()
            .unwrap()
            .split_once("seeds: ")
            .unwrap()
            .1
            .split(' ')
            .map(|n| n.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        let seed_ranges = seeds
            .chunks(2)
            .map(|c| (c[0], c[0] + c[1]))
            .collect::<Vec<_>>();

        let mut maps = vec![];
        for map_input in groups {
            let mut map = vec![];
            for ln in map_input.lines().skip(1) {
                match ln
                    .split(' ')
                    .map(|n| n.parse::<i64>().unwrap())
                    .collect::<Vec<_>>()[..]
                {
                    [dest, src, len] => map.push(SrcDestEntry {
                        src,
                        src_end: src + len,
                        dest,
                        dest_end: dest + len,
                    }),
                    _ => panic!("could not parse map line: {ln}"),
                }
            }
            maps.push(map);
        }

        Input {
            seeds,
            seed_ranges,
            maps,
        }
    }

    fn reverse_maps(&self) -> Input {
        Input {
            seeds: self.seeds.clone(),
            seed_ranges: self.seed_ranges.clone(),
            maps: self
                .maps
                .iter()
                .cloned()
                .rev()
                .map(|v| {
                    v.into_iter()
                        .map(|sde| SrcDestEntry {
                            src: sde.dest,
                            src_end: sde.dest_end,
                            dest: sde.src,
                            dest_end: sde.src_end,
                        })
                        .collect::<Vec<_>>()
                })
                .collect(),
        }
    }

    fn solve(&self, seed: i64) -> i64 {
        let mut seed = seed;
        'outer: for map in &self.maps {
            for mapping in map {
                if seed >= mapping.src && seed <= mapping.src_end {
                    seed = seed - mapping.src + mapping.dest;
                    continue 'outer;
                }
            }
        }
        seed
    }
}

fn main() {
    let contents = fs::read_to_string("inputs/day05.txt").expect("Could not read input");

    let input = Input::from_line(&contents);

    let part1 = input
        .seeds
        .iter()
        .map(|seed| input.solve(*seed))
        .min()
        .unwrap();

    println!("Part 1: {part1}");

    let reversed = input.reverse_maps();
    for seed in 0_i64.. {
        let mapped_seed = reversed.solve(seed);
        for r in &reversed.seed_ranges {
            if mapped_seed >= r.0 && mapped_seed <= r.1 {
                println!("{} -> {}", seed, mapped_seed);
                println!("Part 2: {}", seed);
                return;
            }
        }

        if seed % 10000000 == 0 {
            println!("{} -> {}", seed, mapped_seed);
        }
    }
}
