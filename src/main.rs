#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]

pub mod solutions;
pub mod utils;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    day: u8,
    challenge: u8,
}

fn main() {
    let args = Args::parse();

    println!(
        "{}",
        match args.day {
            1 => match args.challenge {
                1 => solutions::day_1::part_1(),
                _ => panic!("Invalid challenge"),
            },
            _ => panic!("Invalid day"),
        }
    );
}
