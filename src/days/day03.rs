use std::env;
use std::fs::read_to_string;

use std::collections::HashMap;
use std::hash::Hash;

pub fn solve() -> (String, String) {
    let mut sol1 = "<unsolved>".to_string();
    let mut sol2 = "<unsolved>".to_string();

    // Windows style
    let path = "inputs/3/input".replace("/", "\\");

    let contents = read_to_string(path)
        .expect("Error while reading file");

    // Windows style
    let lines = contents.split("\r\n");

    let mut score1 = 0;
    let mut score2 = 0;
    
    let mut set_of_three = Vec::new();

    for line in lines {
        if line.is_empty() {
            continue;
        }

        let (first_half, second_half) = line.split_at(line.len()/2);

        for c in first_half.chars() {
            if second_half.chars().position(|x| x == c) == None {
                continue;
            } else {
                score1 += get_priority(c);
                break;
            }
        }

        set_of_three.push(line);

        if set_of_three.len() == 3 {

            for c in set_of_three.get(0).unwrap().chars() {
                if set_of_three.get(1).unwrap().chars().position(|x| x==c) != None {
                    if set_of_three.get(2).unwrap().chars().position(|x| x==c) != None{
                        score2 += get_priority(c);
                        break;
                    }
                }
            }

            set_of_three.clear();
        }
    }

    sol1 = score1.to_string();
    sol2 = score2.to_string();

    return (sol1.to_string(), sol2.to_string());
}

fn get_priority(c: char) -> u32 {
    if c as u32 >= 'a' as u32 {
        return c as u32 - 'a' as u32 + 1;
    }else {
        return c as u32 - 'A' as u32 + 26 + 1;
    }
}