pub mod day1;
pub mod day2;
pub mod day3;

use std::{fs::File, io::Read};

fn main() {
    let mut file = File::open("inputs/day3.txt").unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    let result = day3::solve_part_2(&buf);

    println!("{result}");
}
