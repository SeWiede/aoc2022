use std::env;
use std::fs::read_to_string;

use std::collections::{HashMap, LinkedList};
use std::hash::Hash;

use sscanf::sscanf;

pub fn solve() -> (String, String) {
    let mut sol1 = "<unsolved>".to_string();
    let mut sol2 = "<unsolved>".to_string();

    // Windows style
    let path = "inputs/6/input".replace("/", "\\");

    let contents = read_to_string(path)
        .expect("Error while reading file");

    // Windows style
    let lines = contents.split("\r\n");

    let mut score1 = 0;
    let mut score2 = 0;
    
    let mut charTracker: HashMap<char, i32> = Default::default();

    let mut uniques = 0;
    let mut startPos = -1;
    let mut startPos2 = -1;

    for line in lines {
        if line.is_empty() {
            continue;
        }

      
        for s in 0..line.len() {
            if let Some((i,j,c)) = line.chars().skip(s).take(4).enumerate().find_map(|(i, c)| {
                line.chars().skip(s).take(4)
                .enumerate()
                .skip(i + 1)
                .find(|(_, other)| c == *other)
                .map(|(j, _)| (i, j, c))
            }) {
                
            } else {
                if startPos < 0 {
                    startPos = s as i32+4;
                }
            }

            if let Some((i,j,c)) = line.chars().skip(s).take(14).enumerate().find_map(|(i, c)| {
                line.chars().skip(s).take(14)
                .enumerate()
                .skip(i + 1)
                .find(|(_, other)| c == *other)
                .map(|(j, _)| (i, j, c))
            }) {
                
            } else {
                if startPos2 < 0 {
                    startPos2 = s as i32+14;
                }
                break;
            }
        }
    }

    
    sol1 = startPos.to_string();
    sol2 = startPos2.to_string();

    return (sol1.to_string(), sol2.to_string());
}
