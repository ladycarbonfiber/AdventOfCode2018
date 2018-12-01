use common::*;
use std::collections::HashSet;
pub fn solve(part:Part)->i32{
    let input = read_input(1);
    match part {
        Part::One => {
            input.lines()
                .map(|s| parse_row(s))
                .fold(0, |a,b| a +b)
        },
        Part::Two => {
            let mut freqs = HashSet::new();
            freqs.insert(0);
            let vals:Vec<i32> = input.lines().map(|s| parse_row(s)).collect();
            let mut cycle_it = vals.into_iter().cycle();
            let mut sum = 0;
            loop{
                sum+= cycle_it.next().unwrap();
                if freqs.contains(&sum){
                    break;
                }else{
                    freqs.insert(sum);
                }
            }

            sum
        }
    }



}
fn parse_row(row:&str) -> i32{
    match row.chars().next() {
        Some(c) => {
            let (sign, value) = row.split_at(c.len_utf8());
            match sign{
                "-"=> -1 * value.parse::<i32>().unwrap(),
                "+"=> value.parse::<i32>().unwrap(),
                _ => 0
            }
        },
        None => 0,
    }
}