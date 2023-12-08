use std::collections::HashMap;

use crate::{Solution, SolutionPair, etc::math::lcm};

pub fn solve(str: String) -> SolutionPair {
    let mut lines = str.lines();
    let dirs: Vec<char> = lines.next().unwrap().chars().collect();

    let nodes: HashMap<String, (String, String)> = lines.skip(1).map(|line| {
        let node = String::from(&line[0..3]);
        let left = String::from(&line[7..10]);
        let right = String::from(&line[12..15]);
        (node, (left, right))
    }).collect();

    // Part 1
    let mut node = String::from("AAA");
    let mut steps = 0;
    loop {
        let dir = dirs[steps % dirs.len()];
        let choices = &nodes[&node];

        if node == "ZZZ" {
            break;
        }

        node = if dir == 'L' {
            choices.0.clone()
        } else {
            choices.1.clone()
        };

        steps += 1;
    }

    // Part 2
    let cur_nodes: Vec<String> = nodes.iter().filter_map(|(key, _)| {
        if key.ends_with('A') {
            Some(key.clone())
        } else {
            None
        }
    }).collect();

    let cycle_lengths: Vec<u64> = cur_nodes.iter().map(|start_node| {
        let mut node = start_node.clone();
        let mut steps = 0;
        loop {
            let dir = dirs[steps % dirs.len()];
            let choices = &nodes[&node];
    
            if node.ends_with('Z') {
                break;
            }
    
            node = if dir == 'L' {
                choices.0.clone()
            } else {
                choices.1.clone()
            };
    
            steps += 1;
        }
        
        steps as u64
    }).collect();

    (Solution::from(steps), Solution::from(lcm(cycle_lengths)))
}