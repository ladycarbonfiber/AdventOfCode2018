use crate::common::*;
use std::collections::HashMap;
pub fn solve(part:Part) -> u32{
    let input = read_input(6);
    //let mut grid = [['.';1000];1000];
    let offset = 500;
    let small_offset = 400;
    let mut destinations = Vec::new();
    let mut points = HashMap::new();
    for (i,line) in input.lines().enumerate(){
        //    println!("{}", line);
        let p = to_point(line);
        points.insert(i,p);
    }

    destinations = points.iter()
        .map(|(i,p)| PointRecord{name:*i as u32,val:0, small_val:0, point:*p})
        .collect();
    match part{
        Part::One =>{
            for x in -1*(offset as i32) .. (offset as i32){
                for y in -1*(offset as i32) .. (offset as i32){
                    let target = Point{x,y};
                    let mut distances:Vec<u32> = destinations
                        .iter()
                        .map(|d| d.point.manhattan_distance(&target))
                        .collect();
                    let (mut i, mut min) = (0, offset * 100);
                    let mut contested = true;
                    for (index, d) in distances.iter().enumerate(){
                        if d < &min{
                            min = *d;
                            i = index;
                            contested = false;
                        }
                        else if d == &min {
                            contested = true;
                        }
                    }

                    if !contested{
                        destinations[i].increment(false);

                    }
                }
            }
            for x in -1*(small_offset as i32) .. (small_offset as i32){
                for y in -1*(small_offset as i32) .. (small_offset as i32){
                    let target = Point{x,y};
                    let distances:Vec<u32> = destinations
                        .iter()
                        .map(|d| d.point.manhattan_distance(&target))
                        .collect();
                    let (mut i, mut min) = (0, offset * 100);
                    let mut contested = true;
                    for (index, d) in distances.iter().enumerate(){
                        if d < &min{
                            min = *d;
                            i = index;
                            contested = false;
                        }
                            else if d == &min {
                                contested = true;
                            }
                    }
//                    let (index, _) = distances.iter()
//                        .enumerate()
//                        .min_by_key(|&(_, d)| d)
//                        .unwrap();
                    if !contested{
                        destinations[i].increment(true);

                    }
                }
            }
            let mut valid:Vec<&PointRecord> = Vec::new();
            for pr in destinations.iter() {
                if !(pr.val > pr.small_val){
                    valid.push(pr);
                }
            }
           println!("Valid {}", valid.len());
            valid.iter().for_each(|pr| pr.point.print());
            let candidate = valid.iter()
                .max_by_key(|d| d.val).unwrap();
            println!("{}, {}, {}", candidate.point.x, candidate.point.y, candidate.val);
            candidate.val as u32

        },
        Part::Two => {
            let mut region = 0;
            for x in -1 * (5000 as i32)..(5000 as i32) {
                for y in -1 * (5000 as i32)..(5000 as i32) {
                    let target = Point{x,y};
                    let d:i32 = destinations
                        .iter()
                        .fold(0, |acc, d| acc + d.point.manhattan_distance(&target) as i32);
                    if d < 10000{
                        region +=1;
                    }
                }
            }
            region
        }

    }


}

fn to_point(cords:&str) -> Point{
    let vals:Vec<i32> = cords.split(",").map(|s| s.trim())
        .map(|v| v.parse::<i32>().unwrap())
        .collect();
    Point{x:vals[0],y:vals[1]}
}
struct PointRecord{
    name:u32,
    val:i32,
    small_val:i32,
    point:Point
}
impl PointRecord{
    fn increment(&mut self, is_small:bool){
        if is_small{
            self.small_val +=1;
        }else {
            self.val += 1;
        }
    }
}