use crate::common::*;
use std::iter;
pub fn solve(part: Part) ->u32 {
    let input = read_input(5);
    let input_copy = read_input(5);

    match part{
        Part::One =>{
            let mut base= input.clone();
            let mut cont = true;
            'main: loop{
                let first = base.clone();
                let  offset = first.chars().skip(1);
                let combo = first.chars().zip(offset);
                let mut is_finished = true;
                for (i,element) in combo.enumerate(){
                    if is_pair(element){
                       // println!("Found pair {}, {}", element.0, element.1);
                        base.remove(i);
                        base.remove(i);
                        is_finished = false;
                        break;
                    }
                }
                if is_finished{
                    break 'main;
                }
            }
            println!("{}", base);
            base.len() as u32

        },
        Part::Two=>{

            let mut results = Vec::new();
            for c in ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'].iter(){
                let mut base= input.clone();
                if !base.contains(&(*c).to_string()){
                    continue;
                };
                base = base.replace(&(c.to_string()), "").replace(&(c.to_ascii_lowercase().to_string()), "");
                let mut cont = true;
                'main1: loop{
                    let first = base.clone();
                    let  offset = first.chars().skip(1);
                    let combo = first.chars().zip(offset);
                    let mut is_finished = true;
                    for (i,element) in combo.enumerate(){
                        if is_pair(element){
                            // println!("Found pair {}, {}", element.0, element.1);
                            base.remove(i);
                            base.remove(i);
                            is_finished = false;
                            break;
                        }
                    }
                    if is_finished{
                        break 'main1;
                    }
                }
                println!("base {}", base);
                results.push(base.len() as u32);
            }
            println!("{:?}", results);
            *results.iter().min().unwrap()
        }
    }


}

fn is_pair(target:(char, char)) -> bool{
    (target.0.is_uppercase() && target.1.is_lowercase() && target.0 == target.1.to_ascii_uppercase())  ||
        (target.0.is_lowercase()  && target.1.is_uppercase() && target.0 == target.1.to_ascii_lowercase())
}