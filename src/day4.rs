use common::*;
use std::collections::HashMap;
use chrono::prelude::*;
use regex::Regex;
use time::Duration;
use itertools::Itertools;
pub fn solve(part:Part)-> u32{
    let input = read_input(4);
    match part{
        Part::One => {
            let mut events:Vec<Event> = input.lines().map(|x| parse_ln(x)).collect();
            events.sort_by_key(|k| k.time);
            let mut guards = HashMap::new();
            //let mut times = HashMap::new();
            let  guard_regex = Regex::new(r"#(\d{2})").unwrap();

            for event in events{
                let mut time = event.time;
                if time.hour() != 0 {
                    time = time + Duration::days(1);
                }
                let key = (time.month(), time.day());
                match guard_regex.captures(&event.event){
                    Some(T)=>{
                        let id_str = T.get(1).map_or("", |m| m.as_str());
                        let id = id_str.parse::<u32>().unwrap();
                        println!("id, {:?} key {:?}", id, key);
                        let shifts = [0;60];
                        guards.insert(key, GuardPost{id, shifts });
                    },
                    None=>{
                       let new_val =  if event.event.as_str().contains("asleep"){
                           //let k = key;
                           let minute = event.time.minute() as usize;
                           let  gp = guards.get(&key).unwrap();
                           gp.fall_asleep(minute)
                        } else{
                           let minute = event.time.minute() as usize;
                           let gp = guards.get(&key).unwrap();
                           gp.wake_up(minute)
                       };
                        guards.insert(key, new_val);
                    }

                }
            }


//            let mut s:Vec<GuardPost> = guards.values().collect();
            let combine = guards.values().skip(1);
            let collected_guards:Vec<GuardPost> = guards.values()
                .zip(combine)
                .filter_map(|tuple| if tuple.0.id == tuple.1.id{
                    Some(tuple.0.add(tuple.1))
                }else{
                    None
                })
                .collect();

            let g = collected_guards.iter().max_by_key(|guard| guard.get_total()).unwrap();
            let mut max = 0;
            let mut minute = 0;
            for (i, sleep) in g.shifts.iter().enumerate(){
                if *sleep >= max{
                    minute = i;
                    max = *sleep;
                }
            }
            println!("id {}, minute {}", g.id, minute);
            println!("sum {}", g.shifts.iter().sum::<u32>());
            for i in g.shifts.iter(){
                print!("{}", i);
            }
            print!("\n");
            g.id * minute as u32
            //let mut eval:Vec<(u32, [u8;60])> = guards.values().map(|v| (v.id, v.shifts)).collect();

//            for (a,b) in eval.iter().zip(eval.iter().skip(1)){
//                if a.0 == b.0{
//
//                }
//            }

//            for (k, v) in guards.iter(){
//
//            }

//            for (id, g) in guards{
//                println!("id {:?}", id);
//                for element in g.shifts.iter(){
//                    print!("{:?}", element);
//                }
//                println!("");
//            }

        },
        Part::Two =>{
            0
        }
    }
}
#[derive(Debug)]
struct Event{
    time:DateTime<Utc>,
    event:String
}
struct GuardPost{
    id:u32,
    shifts:[u32;60]
}
impl GuardPost{
    fn fall_asleep(&self, minute:usize) ->GuardPost{
        let mut new_shifts = self.shifts;
        new_shifts[minute] +=1;
        GuardPost{id:self.id, shifts:new_shifts}
    }
    fn wake_up(&self, minute:usize) -> GuardPost{
        let mut new_shifts = self.shifts;
        for index in (0 .. minute).rev(){
            if self.shifts[index] == 1{
                break;
            }
            new_shifts[index] +=1;
        }
        GuardPost{id:self.id, shifts:new_shifts}
    }
    fn add(&self, b:&GuardPost) -> GuardPost{
        let mut new_shifts = self.shifts;
        let out:Vec<u32> = new_shifts.into_iter().zip(b.shifts.into_iter()).map(|x| x.0+x.1).collect();
        for (i,element) in out.iter().enumerate(){
            new_shifts[i] = *element
        }
        GuardPost{id:self.id, shifts:new_shifts}
    }
    fn get_total(&self) -> u32{
        let total = self.shifts.iter().sum();
        println!("id {}, total{}", self.id, total);
        total

    }
}
impl Default for GuardPost{
    fn default() -> GuardPost{
        GuardPost{id:0, shifts:[0; 60]}
    }
}
fn add_array(a:&[u8;60],b:&[u8;60])-> [u8;60]{
    let mut out = *a;
    for (i, element) in b.iter().enumerate(){
       out[i] += element;
    }
    out
}
fn parse_ln(line:&str) -> Event{
    let tuple:Vec<String> = line.replace("[", "")
        .replace("]", " ")
        .split("  ")
        .map(|x| x.to_string())
        .collect();
    Event{ time:Utc.datetime_from_str(&tuple[0], "%Y-%m-%d %H:%M").unwrap(), event: tuple[1].to_string()}

}
