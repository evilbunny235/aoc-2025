pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;

use std::{fs::File, io::Read};

fn main() {
    let mut file = File::open("inputs/day6.txt").unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    let result = day6::solve_part_1(&buf);

    println!("{result}");
}
