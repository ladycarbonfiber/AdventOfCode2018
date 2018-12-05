extern crate chrono;
extern crate regex;
extern crate time;
extern crate itertools;
mod common;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
use common::Part;
macro_rules! day {
    ( $( $x:ident ),* ) => {
        $(
            println!("{}: Part One Solution: {}", stringify!($x), $x::solve(Part::One));
            println!("{}: Part Two Solution: {}", stringify!($x), $x::solve(Part::Two));
        )*
    };
}

fn main(){
    println!("Advent of Code 2018");

    day!(day5);
}