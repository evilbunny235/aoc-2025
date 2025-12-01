use std::io::{BufRead, BufReader};

pub fn solve_part_1<T: std::io::Read>(reader: BufReader<T>) -> i64 {
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

    result
}

pub fn solve_part_2<T: std::io::Read>(reader: BufReader<T>) -> i64 {
    let mut dial: i64 = 50;
    let mut result = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        let (op, n) = line.split_at(1);
        let mut n: i64 = n.parse().unwrap();

        result += n / 100;
        n %= 100;

        let before = dial;
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

        if dial < 0 {
            if before != 0 {
                result += 1;
            }

            dial += 100;
        } else if dial >= 100 {
            result += 1;
            dial -= 100;
        } else if dial == 0 {
            result += 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;
    const TEST_DATA: &str = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#;

    #[test]
    fn p1() {
        let reader = BufReader::new(Cursor::new(TEST_DATA));
        let result = solve_part_1(reader);

        assert_eq!(result, 3);
    }

    #[test]
    fn p2() {
        let reader = BufReader::new(Cursor::new(TEST_DATA));
        let result = solve_part_2(reader);

        assert_eq!(result, 6);
    }
}
