use std::env;
use std::fs::read_to_string;

use std::collections::HashMap;
use std::hash::Hash;

pub fn solve() -> (String, String) {
    let mut sol1 = "<unsolved>".to_string();
    let mut sol2 = "<unsolved>".to_string();

    // Windows style
    let path = "../../inputs/2/input".replace("/", "\\");

    let contents = read_to_string(path)
        .expect("Error while reading file");

    // Windows style
    let lines = contents.split("\r\n");

    // A, X Rock
    // B, Y Paper
    // C, Z Scissors 

    let mut pickScore = HashMap::new();
    pickScore.insert('X', 1);
    pickScore.insert('Y', 2);
    pickScore.insert('Z', 3);

    let mut winScoreA = HashMap::new();
    winScoreA.insert('X', 3);
    winScoreA.insert('Y', 6);
    winScoreA.insert('Z', 0);

    let mut winScoreB = HashMap::new();
    winScoreB.insert('X', 0);
    winScoreB.insert('Y', 3);
    winScoreB.insert('Z', 6);

    let mut winScoreC = HashMap::new();
    winScoreC.insert('X', 6);
    winScoreC.insert('Y', 0);
    winScoreC.insert('Z', 3);

    let mut winScore = HashMap::new();
    winScore.insert('A', winScoreA);
    winScore.insert('B', winScoreB);
    winScore.insert('C', winScoreC);

    // X lose
    // Y draw
    // Z win

    let mut needOffset  = HashMap::new();
    needOffset.insert('X', 2);
    needOffset.insert('Y', 0);
    needOffset.insert('Z', 1);
    
    let mut score1 = 0;
    let mut score2 = 0;

    for line in lines {
        if line.is_empty() {
            continue;
        }

        let opp_pick = line.chars().nth(0).unwrap();
        let my_pick =line.chars().nth(2).unwrap();

        let mut line_score1 = *pickScore.get(&my_pick).unwrap();
        line_score1 += *winScore.get(&opp_pick).unwrap().get(&my_pick).unwrap();

        let offset_char = char::from_u32('X' as u32 + (((opp_pick as u32 - 'A' as u32 ) + *needOffset.get(&my_pick).unwrap()) %3)).unwrap();

        let mut line_score2 = *pickScore.get(&offset_char).unwrap();
        line_score2 += *winScore.get(&opp_pick).unwrap().get(&offset_char).unwrap();

        score1 += line_score1;
        score2 += line_score2;
    }

    sol1 = score1.to_string();
    sol2 = score2.to_string();

    return (sol1.to_string(), sol2.to_string());
}