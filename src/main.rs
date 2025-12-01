use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("inputs/day1.txt").unwrap();
    let reader = BufReader::new(file);

    let mut dial: i64 = 50;
    let mut result = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        let (op, n) = line.split_at(1);
        let n: i64 = n.parse().unwrap();

        match op {
            "L" => {
                dial -= n;
            }
            "R" => {
                dial += n;
            }
            _ => {
                panic!("unknown op");
            }
        }

        dial = dial % 100;
        if dial < 0 {
            dial += 100;
        }

        if dial == 0 {
            result += 1;
        }
    }

    println!("result: {result}");
}
