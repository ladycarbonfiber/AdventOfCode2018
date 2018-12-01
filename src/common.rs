use std::fs::File;
use std::io::prelude::*;
pub fn read_input(day:u16) -> String{
    let path = format!("day{}.txt", day);
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}
#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub struct Point{
    pub x:i32,
    pub y:i32,
}
pub enum Direction{
    Up,
    Left,
    Down,
    Right
}
#[derive(Debug, PartialEq)]
pub enum Part{
    One,
    Two
}
impl Point{
    pub fn update(&mut self, heading:&Direction){
        match heading {
            &Direction::Right => {
                self.x +=1;
            },
            &Direction::Left => {
                self.x -=1;
            },
            &Direction::Up => {
                self.y+=1;
            },
            &Direction::Down => {
                self.y-=1;
            }
        }
    }
    pub fn print(&self){
        println!("x: {}, y: {}", self.x, self.y);
    }
}

pub fn get_neighbors(point:&Point) -> Vec<Point>{
    let mut neighbors = Vec::new();
    for i in {-1 ..2}{
        for j in {-1..2}{
            if !(i==0 && j==0) {
                neighbors.push(Point { x: point.x +i, y: point.y +j })
            }
        }
    }
    neighbors

}