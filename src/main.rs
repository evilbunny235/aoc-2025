pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;

use std::{fs::File, io::Read};

fn main() {
    let mut file = File::open("inputs/day4.txt").unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    let result = day4::solve_part_2(&buf);

    println!("{result}");
}
