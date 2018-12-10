use crate::common::*;
use std::collections::HashMap;

pub fn solve(part: Part) -> String {
    let input = read_input(2);
    match part {
        Part::One => {
            let mut twos = 0;
            let mut threes = 0;
            for row in input.lines() {
                let mut counts: HashMap<char, i32> = HashMap::new();
                for c in row.chars() {
                    *counts.entry(c).or_insert(0) += 1;
                }
                let mut has2 = false;
                let mut has3 = false;
                for (_, count) in &counts {
                    if *count == 2 && !has2 {
                        has2 = true;
                    }
                    if *count == 3 && !has3 {
                        has3 = true;
                    }
                }
                match (has2, has3) {
                    (true, true) => {
                        twos += 1;
                        threes += 1;
                    }
                    (true, false) => {
                        twos += 1;
                    }
                    (false, true) => {
                        threes += 1;
                    }
                    (false, false) => ()
                }
            }
            (twos * threes).to_string()
        }
        Part::Two => {
            let mut vals = String::new();
            for row in input.lines() {
                let base = &row;
                for comp in input.lines() {
                    let (hit, same) = validate(base, &comp);
                    if hit {
                        vals = same;
                    }
                }
            }
            vals
        }
    }
}

fn validate(a: &str, b: &str) -> (bool, String) {
    let diff = a.chars()
        .zip(b.chars())
        .filter(|(a_c, b_c)| a_c != b_c)
        .count() == 1;
    let same = a.chars()
        .zip(b.chars())
        .filter_map(|(x, y)| {
            if x == y {
                return Some(x);
            }
            None
        })
        .collect();
    return (diff, same);
}