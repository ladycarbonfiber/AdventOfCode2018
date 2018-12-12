use crate::common:: *;
use rayon::prelude::*;
use std::collections::HashMap;
use std::iter;

pub fn solve(part: Part) -> String {
    let serial = 9110;
    //let serial = 42;
    let mut grid = HashMap::new();

    for x in 1.. = 300 {
        for y in 1.. = 300 {
            grid.insert((x, y), calc_power(x, y, serial));
        }
    }
    match part {
        Part::One => {
            let mut cell = (-1, -1);
            let mut power = 0;
            for x in 1..298 {
                for y in 1..298 {
                    let temp = calc_block((x, y), 3, &grid);
                    if temp > power {
                        cell = (x, y);
                        power = temp;
                    }
                }
            }

            format!("{},{}", cell.0, cell.1)
        }
        Part::Two => {
            let sizes: Vec<((i32, i32), i32, i32)> = (1..).into_par_iter()
                .map(|size| {
                    println!("Checking size {}", size);
                    let mut power = 0;
                    let mut cell = (-1, -1);
                    for x in 1..300 - (size - 1) {
                        for y in 1..300 - (size - 1) {
                            let temp = calc_block((x, y), size, &grid);
                            if temp > power {
                                cell = (x, y);
                                power = temp;
                            }
                        }
                    }
                    (cell, power, size)
                })
                .collect();
            let best = sizes.iter().max_by_key(|x| x.1).unwrap();
            format!("{},{},{}", (best.0).0, (best.0).1, best.2)
        }
    }
}

fn calc_power(x: i32, y: i32, serial: i32) -> i32 {
    let rack_id = x + 10;
    ((((rack_id * y + serial) * rack_id) / 100) % 10) - 5
}

fn calc_block(cell: (i32, i32), size: i32, grid: &HashMap<(i32, i32), i32>) -> i32 {
    let points: Vec<(i32, i32)> = (cell.0..cell.0 + size)
        .flat_map(|i| iter::repeat(i).zip(cell.1..cell.1 + size))
        .collect();
    let sum = points
        .into_par_iter()
        .map(|tuple| *grid.get(&tuple).unwrap())
        .sum();

    sum
}
