pub mod day1;

use std::{fs::File, io::BufReader};

fn main() {
    let file = File::open("inputs/day1.txt").unwrap();
    let reader = BufReader::new(file);

    let result = day1::solve_part_2(reader);

    println!("{result}");
}
