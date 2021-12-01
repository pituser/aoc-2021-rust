use std::{env, process};

mod day01;

mod util;

fn main() {
    let day: u32;

    // Read day to run from command line
    let args = env::args().collect::<Vec<String>>();

    if args.len() == 2 {
        day = match args[1].parse::<u32>() {
            Ok(n) => n,
            Err(_) => {
                println!("Error parsing day number: \"{}\"", args[1]);
                process::exit(1);
            }
        }
    } else {
        println!("usage: aoc-2021-rust <day>");
        process::exit(1);
    }

    // Run soluntion(s) for that day
    println!("Advent of Code 2021 --- Day {}", day);

    match day {
        1 => day01::solve(),
        _ => {
            println!("Day {} not (yet) solved.", day);
        }
    }
}
