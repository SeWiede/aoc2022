use std::env;
use std::fs::read_to_string;

use std::collections::HashMap;
use std::hash::Hash;

use sscanf::sscanf;

pub fn solve() -> (String, String) {
    let mut sol1 = "<unsolved>".to_string();
    let mut sol2 = "<unsolved>".to_string();

    // Windows style
    let path = "inputs/4/input".replace("/", "\\");

    let contents = read_to_string(path)
        .expect("Error while reading file");

    // Windows style
    let lines = contents.split("\r\n");

    let mut score1 = 0;
    let mut score2 = 0;
    

    for line in lines {
        if line.is_empty() {
            continue;
        }

        let parsed = sscanf::sscanf!(line, "{i32}-{i32},{i32}-{i32}");
        let (mut min1, mut max1, mut min2, mut max2) = parsed.unwrap();

        if (min1 <= min2 && max1 >= max2) || (min2 <= min1 && max2 >= max1) {
            score1 += 1;
        }

        if (min1 <= min2 && max1 >= min2) || (min2 <= min1 && max2 >= min1) {
            score2 += 1;
        }
    }

    sol1 = score1.to_string();
    sol2 = score2.to_string();

    return (sol1.to_string(), sol2.to_string());
}
