mod common;
mod day1;
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

    day!(day1);
}