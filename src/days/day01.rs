/* 

DAY TEMPLATE

use std::env;
use std::fs;

pub fn solve() -> (String, String) {
    let mut sol1 = "<unsolved>".to_string();
    let mut sol2 = "<unsolved>".to_string();

    // Windows style
    let path = "../../inputs/1/input".replace("/", "\\");

    let contents = read_to_string(path)
        .expect("Error while reading file");

    // Windows style
    let lines = contents.split("\r\n");

    for line in lines {
        println!("{}", line)
    }

    return (sol1.to_string(), sol2.to_string());
} */

use std::env;
use std::fs::read_to_string;

pub fn solve() -> (String, String) {
    let mut sol1 = "<unsolved>".to_string();
    let mut sol2 = "<unsolved>".to_string();

    // Windows style
    let path = "inputs/1/input".replace("/", "\\");

    let contents = read_to_string(path)
        .expect("Error while reading file");

    // Windows style
    let lines = contents.split("\r\n");

    let mut current_sum = 0;

    let mut vec: Vec<i32> = Vec::new();

    for line in lines {
        if line.is_empty() {
            if vec.len() > 0 {
                let mut pos = -1;
                for (i, x) in vec.iter().enumerate() {
                    if current_sum > *x {
                        pos = i as i32;
                        break;
                    }
                }

                if pos >= 0 && pos <= 2{
                    vec.insert(pos as usize, current_sum);
                }

                if vec.len() > 3 {
                    vec.pop();
                }
            }else {
                vec.push(current_sum);
            }

            current_sum = 0;
            continue;
        }

        current_sum += line.to_string().parse::<i32>().unwrap();
    }

    sol1 = vec[0].to_string();
    sol2 = vec.iter().sum::<i32>().to_string();

    return (sol1.to_string(), sol2.to_string());
}