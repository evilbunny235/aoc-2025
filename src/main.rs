pub mod day1;
pub mod day2;

use std::{fs::File, io::Read};

fn main() {
    let mut file = File::open("inputs/day2.txt").unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    let result = day2::solve_part_2(&buf);

    println!("{result}");
}
