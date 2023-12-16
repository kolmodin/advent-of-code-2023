use std::fs;

fn hash_str(inp: &str) -> usize {
    let mut current_value = 0;
    for u8 in inp.as_bytes() {
        current_value += *u8 as usize;
        current_value *= 17;
        current_value %= 256;
    }
    current_value
}

fn split_label(input: &str) -> (&str, &str) {
    let bytes = input.as_bytes();
    for ix in 0..input.len() {
        if !bytes[ix].is_ascii_alphabetic() {
            return (&input[0..ix], &input[ix..]);
        }
    }
    (input, "")
}

fn main() {
    let contents = fs::read_to_string("inputs/day15.txt").expect("Could not read input");

    let inputs: Vec<_> = contents.lines().next().unwrap().split(',').collect();

    let part1 = inputs.iter().map(|inp| hash_str(inp)).sum::<usize>();
    println!("Part 1: {}", part1);

    const ARRAY_REPEAT_VALUE: Vec<(&str, i32)> = Vec::new();
    let mut boxes: [Vec<(&str, i32)>; 256] = [ARRAY_REPEAT_VALUE; 256];

    for input in &inputs {
        let (label, rest) = split_label(input);
        let label_hash = hash_str(&label);

        if rest.as_bytes()[0] == b'-' {
            boxes[label_hash].retain(|(slabel, _)| *slabel != label);
        } else if rest.as_bytes()[0] == b'=' {
            let new_lens = (label, rest[1..].parse::<i32>().unwrap());
            let lenses = &mut boxes[label_hash];
            if let Some(lens) = lenses.iter_mut().find(|(slabel, _)| *slabel == label) {
                *lens = new_lens;
            } else {
                lenses.push(new_lens);
            }
        }
    }

    let score = boxes
        .into_iter()
        .enumerate()
        .flat_map(|(box_id, lenses)| {
            lenses
                .into_iter()
                .enumerate()
                .map(move |(lens_id, (_, focal_strength))| {
                    (box_id as i32 + 1) * (lens_id as i32 + 1) * focal_strength
                })
        })
        .sum::<i32>();

    println!("Part 2: {}", score);
}
