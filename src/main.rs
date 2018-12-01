mod common;
macro_rules! days {
    ( $( $x:ident ),* ) => {
        $(
            mod $x;
        )*
        fn print_all_solutions() {
            $(
                  println!("{}: Part One Solution: {}", stringify!($x), $x::solve(Part::One));
                  println!("{}: Part Two Solution: {}", stringify!($x), $x::solve(Part::Two));
            )*
        }
    };
}
use common::Part;
days!(day1);
fn main(){
    println!("Advent of Code 2018");
    print_all_solutions();
}