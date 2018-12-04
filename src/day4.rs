use common::*;
use std::collections::HashMap;
use chrono::prelude::*;
use regex::Regex;
use time::Duration;
use itertools::*;

pub fn solve(part: Part) -> u32 {
    let input = read_input(4);
    let mut events: Vec<Event> = input.lines().map(|x| parse_ln(x)).collect();
    events.sort_by_key(|k| k.time);
    let mut guards = HashMap::new();
    //let mut times = HashMap::new();
    let guard_regex = Regex::new(r"#(\d+) ").unwrap();

    for event in events {
        let mut time = event.time;
        if time.hour() != 0 {
            time = time + Duration::days(1);
        }
        let key = (time.month(), time.day());
        match guard_regex.captures(&event.event) {
            Some(t) => {
                let id_str = t.get(1).map_or("", |m| m.as_str());
                let id = id_str.parse::<u32>().unwrap();
                let shifts = [0; 60];
                guards.insert(key, GuardPost { id, shifts });
            }
            None => {
                let new_val = if event.event.as_str().contains("asleep") {
                    //let k = key;
                    let minute = event.time.minute() as usize;
                    let gp = guards.get(&key).unwrap();
                    gp.fall_asleep(minute)
                } else {
                    let minute = event.time.minute() as usize;
                    let gp = guards.get(&key).unwrap();
                    gp.wake_up(minute)
                };
                guards.insert(key, new_val);
            }
        }
    }
    let mut combined_guards: HashMap<u32, GuardPost> = HashMap::new();
    for (key, group) in &guards.values()
        .sorted_by_key(|e| (*e).id)
        .iter()
        .group_by(|e| e.id) {
        let g = group.fold(GuardPost { id: key, shifts: [0; 60] }, |a, b| a.add(b));
        combined_guards.insert(key, g);
    }
    match part {
        Part::One => {
            let g = combined_guards.values()
                .max_by_key(|guard| (*guard).get_total())
                .unwrap();
            let mut max = 0;
            let mut minute = 0;
            for (i, sleep) in g.shifts.iter().enumerate() {
                if *sleep > max {
                    minute = i;
                    max = *sleep;
                }
            }
            g.id * minute as u32
        }
        Part::Two => {
            let g = combined_guards.values()
                .max_by_key(|guard| (*guard).get_max().1)
                .unwrap();
//                .max_by_key(|(i,val)| *val)
//                .unwrap();
            let (minute, _) = g.get_max();
            minute as u32 * g.id
        }
    }
}

#[derive(Debug)]
struct Event {
    time: DateTime<Utc>,
    event: String,
}

struct GuardPost {
    id: u32,
    shifts: [u32; 60],
}

impl GuardPost {
    fn fall_asleep(&self, minute: usize) -> GuardPost {
        let mut new_shifts = self.shifts;
        new_shifts[minute] += 1;
        GuardPost { id: self.id, shifts: new_shifts }
    }
    fn wake_up(&self, minute: usize) -> GuardPost {
        let mut new_shifts = self.shifts;
        for index in (0..minute).rev() {
            if self.shifts[index] == 1 {
                break;
            }
            new_shifts[index] += 1;
        }
        GuardPost { id: self.id, shifts: new_shifts }
    }
    fn add(&self, b: &GuardPost) -> GuardPost {
        let mut new_shifts = self.shifts;
        let out: Vec<u32> = new_shifts.into_iter()
            .zip(b.shifts.into_iter())
            .map(|x| x.0 + x.1)
            .collect();
        for (i, element) in out.iter().enumerate() {
            new_shifts[i] = *element
        }
        GuardPost { id: self.id, shifts: new_shifts }
    }
    fn get_total(&self) -> u32 {
        let total = self.shifts.iter().sum();
        //println!("id {}, total{}", self.id, total);
        total
    }
    fn get_max(&self) -> (usize, u32) {
        let (index, max) = self.shifts.iter().enumerate()
            .max_by_key(|(_, e)| *e).unwrap();
        (index, *max)
    }
}

impl Default for GuardPost {
    fn default() -> GuardPost {
        GuardPost { id: 0, shifts: [0; 60] }
    }
}

fn parse_ln(line: &str) -> Event {
    let tuple: Vec<String> = line.replace("[", "")
        .replace("]", " ")
        .split("  ")
        .map(|x| x.to_string())
        .collect();
    Event { time: Utc.datetime_from_str(&tuple[0], "%Y-%m-%d %H:%M").unwrap(), event: tuple[1].to_string() }
}
