use std::env;
use std::fs::read_to_string;

use std::collections::{HashMap, LinkedList};
use std::hash::Hash;

use sscanf::sscanf;

pub fn solve() -> (String, String) {
    let mut sol1 = "<unsolved>".to_string();
    let mut sol2 = "<unsolved>".to_string();

    // Windows style
    let path = "inputs/5/input".replace("/", "\\");

    let contents = read_to_string(path)
        .expect("Error while reading file");

    // Windows style
    let lines = contents.split("\r\n");

    let mut score1 = "".to_string();
    let mut score2 = "".to_string();
    
    let mut stacks: Vec<LinkedList<char>> = Default::default();
    let mut stacks2: Vec<LinkedList<char>> = Default::default();

    let mut line_len = 0;

    let mut stacks_built = false;
    let mut stacks_init = false;

    for line in lines {
        if line.is_empty() {
            
            if !stacks_built {
                stacks2 = stacks.clone();
            }
            stacks_built = true;

            continue;
        }

        
        if !stacks_built {
            if !stacks_init {
                line_len = line.len();
                
                stacks = vec![Default::default(); line_len/4 + 1];

                stacks_init =  true;
            }


            let mut pos = 1;

            while pos <= line_len {
                let c = line.chars().nth(pos).unwrap();
                
                if c != ' '  {
                    if c as i32 - '9' as i32 <= 0 {
                        break;
                    }
                    
                    stacks[pos/4].push_front(c);
                } 

                pos += 4;
            }
        } else {
            let (mut times, mut from, mut to) = sscanf::sscanf!(line, "move {i32} from {i32} to {i32}").unwrap();

            from -= 1;
            to -= 1;

            let stackcopypart = stacks2[from as usize].clone();
            
            for t in 0..times {
                let elem = stacks[from as usize].pop_back().unwrap();
                stacks[to as usize].push_back(elem);

                stacks2[from as usize].pop_back();
            }

            stacks2[to as usize].append(&mut LinkedList::from_iter(stackcopypart.into_iter().rev().take(times as usize).rev()));
        }   
    }

    for s in stacks {
        score1 = format!("{}{}", score1, s.back().unwrap());
    }
    
    for s in stacks2 {
        score2 = format!("{}{}", score2, s.back().unwrap());
    }
    
    sol1 = score1.to_string();
    sol2 = score2.to_string();

    return (sol1.to_string(), sol2.to_string());
}
