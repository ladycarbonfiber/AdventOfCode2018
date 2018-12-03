use common::read_input;
use common::Part;
use std::collections::HashMap;

pub fn solve(part: Part) -> u32 {
    let input = read_input(3);
    let mut fabric = HashMap::new();
    let mut claims = Vec::new();
    for claim in input.lines() {
        let c = parse_claim(&claim);
        for i in c.left_offset..c.left_offset + c.width {
            for j in c.top_offset..c.top_offset + c.height {
                *fabric.entry((i, j)).or_insert(0) += 1;
            }
        }
        claims.push(c);
    }
    match part {
        Part::One => {
            let mut overlap = 0;
            for (_, counts) in fabric {
                if counts > 1 {
                    overlap += 1;
                }
            }
            overlap
        }
        Part::Two => {
            let mut id = 0;
            for c in claims {
                let mut has_overlap = false;
                for i in c.left_offset..c.left_offset + c.width {
                    for j in c.top_offset..c.top_offset + c.height {
                        if *fabric.get(&(i, j)).unwrap() > 1 {
                            has_overlap = true;
                            break;
                        }
                    }
                }
                if !has_overlap {
                    id = c.id;
                }
            }
            id
        }
    }
}

#[derive(Debug)]
struct Claim {
    id: u32,
    left_offset: u32,
    top_offset: u32,
    width: u32,
    height: u32
}

fn parse_claim(s: &str) -> Claim {
    let tokens: Vec<u32> = s.replace("#", "")
        .replace("@ ", "")
        .replace(":", "")
        .replace(",", " ")
        .replace("x", " ")
        .split(" ")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    Claim { id: tokens[0], left_offset: tokens[1], top_offset: tokens[2], width: tokens[3], height: tokens[4] }
}