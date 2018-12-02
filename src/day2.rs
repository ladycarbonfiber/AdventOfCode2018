use common::*;
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
                if has2 {
                    twos = twos + 1;
                }
                if has3 {
                    threes = threes + 1;
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
    let mut mismatch = 0;
    let mut same = String::new();
    let mut a_iter = a.chars();
    let mut b_iter = b.chars();
    for _ in 0..a.to_string().len() {
        let aa = a_iter.next().unwrap();
        if !aa.eq(&b_iter.next().unwrap()) {
            mismatch += 1;
        } else {
            same.push(aa);
        }
    }
    return (mismatch == 1, same);
}