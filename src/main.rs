mod days;

use days::{day01};

use std::env;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("invalid number of args: one day <number> expected!")
    }

    let day = &args[1].to_string().parse::<i32>().unwrap();

    println!("Solving aoc2022 for day {}", day);

    let time = Instant::now();

    let elapsed_time = time.elapsed().as_millis();

    let (sol1, sol2) = get_solver(*day)();

    println!("Day {} first solution: {}", day, sol1);
    println!("Day {} second solution: {}", day, sol2);

    println!("Total runtime: {} ms", elapsed_time);
}

fn get_solver(day: i32) -> fn() -> (String, String) {
    match day {
        1 => day01::solve,
        _ => unimplemented!(),
    }
}
