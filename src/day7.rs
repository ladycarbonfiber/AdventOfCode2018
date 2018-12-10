use crate::common::Part;
use crate::common::read_input;
use regex::Regex;
use lazy_static::lazy_static;
use petgraph::prelude::*;
use petgraph::algo::*;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;

pub fn solve(part: Part) -> String {
    let input = read_input(7);
    let node_regex = Regex::new(r"Step (\w) .+ step (\w)").unwrap();
    match part {
        Part::One => {
            let mut out = String::new();
            let mut node_table = HashMap::new();
            let mut deps = Graph::<&str, &str>::new();
            let edges: Vec<(&str, &str)> = input.lines().map(|x| parse_line(x)).collect();
            let mut nodes = HashSet::new();
            edges.iter().for_each(|x| {
                nodes.insert(x.0);
                nodes.insert(x.1);
            });
            for node in nodes {
                node_table.insert(node, deps.add_node(node));
            }
            let adjusted_edges: Vec<(NodeIndex<u32>, NodeIndex<u32>)> = edges.iter()
                .map(|x| (*node_table.get(x.0).unwrap(), *node_table.get(x.1).unwrap()))
                .collect();
            deps.extend_with_edges(adjusted_edges);
            let mut indx_deq = VecDeque::new();
            let mut processed = HashSet::new();
            for c in b'A'..= b'Z' {
                if deps.neighbors_directed(*node_table.get(&(c as char).to_string().as_str()).unwrap(), Direction::Incoming).
                    fold(true, |x, y| x && processed.contains(deps[y])) {
                    indx_deq.push_back(*node_table.get(&(c as char).to_string().as_str()).unwrap());
                }
            }
            loop {
                let maybe_current = indx_deq.pop_front();
                match maybe_current {
                    Some(current) => {
                        if deps.neighbors_directed(current, Direction::Incoming)
                            .fold(true, |x, y| x && processed.contains(deps[y]))
                            {
                                out.push_str(deps[current]);
                                processed.insert(deps[current]);
                                let mut next_nodes: Vec<NodeIndex> = deps.neighbors_directed(current, Direction::Outgoing).collect();
                                for node in indx_deq.drain(..) {
                                    next_nodes.push(node);
                                }
                                next_nodes.sort_by(|x, y| deps[*x].cmp(deps[*y]).reverse()
                                    .then(deps[*x].len().cmp(&deps[*y].len())));
                                for node in next_nodes.drain(..) {
                                    if !indx_deq.contains(&node) {
                                        indx_deq.push_front(node);
                                    }
                                }
                            } else {
                            indx_deq.push_back(current);
                        }
                    }
                    None => break
                }
            }
            out
        }
        Part::Two => {
            "".to_string()
        }
    }
}

fn parse_line(line: &str) -> (&str, &str) {
    lazy_static! {
        static ref node_regex:Regex =  Regex::new(r"Step (\w) .+ step (\w)").unwrap();
        }
    match node_regex.captures(line) {
        Some(t) => {
            (t.get(1).map_or("", |m| m.as_str()), t.get(2).map_or("", |m| m.as_str()))
        }
        None => {
            println!("Error parsing {}", line);
            ("", "")
        }
    }
}