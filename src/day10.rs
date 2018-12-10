use crate::common::Part;
use crate::common::read_input;
use std::fmt;
use std::io;
use std::collections::HashSet;
use regex::Regex;
use lazy_static::lazy_static;
pub fn solve(part:Part) ->String{
    let input = read_input(10);
    let mut seconds = 0;
    let mut points:Vec<Point> = input.lines()
        .map(|s| parse_line(s))
        .collect();
    loop{
        seconds +=1;
        points.iter_mut().for_each(|mut p| p.update());
        let point_set:HashSet<(i32,i32)> = points.iter().map(|p| (p.x, p.y)).collect();
        if display_ready(-200, 200, -200, 200, &point_set){
            break;
        }
    }
    println!("iterate");
    loop {
        display(&points);
        points.iter_mut().for_each(|mut p| p.update());
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.eq(&"q\n".to_string()){
                    break;
                }else{
                    seconds+=1;
                }
            }
            Err(error) => println!("error: {}", error),
        }

    }
    match part {
        Part::One =>{
            "Look at stdout".to_string()
        },
        Part::Two =>{
            seconds.to_string()
        }
    }



}
#[derive(Debug)]
struct Point{
    x:i32,
    y:i32,
    dx:i32,
    dy:i32
}
impl Point {
    fn update(&mut self){
        self.x += self.dx;
        self.y += self.dy;
    }
}
impl fmt::Display for Point{
    fn fmt(&self, f: &mut fmt::Formatter)-> fmt::Result{
        write!(f, "(x:{}, y:{}, dx:{}, dy:{})", self.x, self.y, self.dx, self.dy)
    }
}
fn parse_line(line:&str)->Point {
    lazy_static! {
        static ref line_regex:Regex = Regex::new(r"position=< *(-*\d+), *(-*\d+)> velocity=< *(-*\d+), *(-*\d+)>").unwrap();
    }
    match line_regex.captures(line) {
        Some(t) => {
            let x = t.get(1).map_or(0, |x| x.as_str().parse::<i32>().unwrap());
            let y = t.get(2).map_or(0, |x| x.as_str().parse::<i32>().unwrap());
            let dx = t.get(3).map_or(0, |x| x.as_str().parse::<i32>().unwrap());
            let dy = t.get(4).map_or(0, |x| x.as_str().parse::<i32>().unwrap());
            Point { x, y, dx, dy }
        },
        None => {
            println!("Error parsing {}", line);
            Point { x: 0, y: 0, dx: 0, dy: 0 }
        }
    }
}
fn display_ready(lower_x:i32, upper_x:i32, lower_y:i32, upper_y:i32, point_set:&HashSet<(i32,i32)> ) -> bool{
    point_set.iter().fold(true, |x, y| x && (y.0 > lower_x && y.0 < upper_x
        && y.1 > lower_y && y.1 < upper_y))
}
fn bounds(points:&Vec<Point>)->(i32, i32, i32,i32){
    let mut bounds = (points[0].x, points[0].x, points[0].y, points[0].y);
    for point in points.iter().skip(1){
        if point.x < bounds.0{
            bounds.0 = point.x -1;
        }
        if point.x> bounds.1 {
            bounds.1 = point.x +1;
        }

        if point.y < bounds.2 {
            bounds.2 = point.y -1;
        }

        if point.y > bounds.3 {
            bounds.3 = point.y +1;
        }
    }
    bounds
}
fn display(points:&Vec<Point>){
    let point_set:HashSet<(i32,i32)> = points.iter().map(|p| (p.x, p.y)).collect();
    let (lower_x, upper_x, lower_y, upper_y) = bounds(points);
    for x in 0 .. 15{
        print!("*");
    }
    print!("\n");
    println!("x_low{}, x_high{}, y_low{}, y_hi{}", lower_x, upper_x, lower_y, upper_y);
    let point_set:HashSet<(i32,i32)> = points.iter().map(|p| (p.x, p.y)).collect();
    for i in lower_y .. upper_y{
        for j in lower_x.. upper_x{
            if point_set.contains(&(j,i)){
                print!("#")
            }else{
                print!(".")
            }
        }
        print!("\n")
    }
}
