pub fn solve_part_1(input: &str) -> u64 {
    let lines = input.lines();

    let mut nums = vec![];
    let mut ops = vec![];

    for line in lines {
        let line = line.trim();
        if line.starts_with('*') || line.starts_with('+') {
            ops = line.split_whitespace().collect();
            break;
        }

        let v: Vec<u64> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        nums.push(v);
    }

    assert!(nums.iter().all(|v| v.len() == ops.len()));

    let mut sum = 0;

    for i in 0..ops.len() {
        let result = match ops[i] {
            "+" => {
                let mut sum = 0;

                for v in &nums {
                    sum += v[i];
                }

                sum
            }
            "*" => {
                let mut prod = 1;

                for v in &nums {
                    prod *= v[i];
                }

                prod
            }
            op => {
                panic!("invalid op: {op}")
            }
        };

        sum += result;
    }

    sum
}

pub fn solve_part_2(input: &str) -> u64 {
    let (data, ops) = input.trim().rsplit_once('\n').unwrap();
    let mut ops = ops.split_whitespace().into_iter();
    let mut op = ops.next().unwrap();

    let data = data.as_bytes();
    let line_len = data.iter().position(|e| *e == b'\n').unwrap() + 1;
    let line_count = data.iter().filter(|&e| *e == b'\n').count() + 1;

    let mut result = 0;
    let mut current_op_result = get_op_identity(op);

    for col in 0..line_len - 1 {
        let mut col_has_data = false;
        let mut num = 0u64;

        for row in 0..line_count {
            let digit = data[col + line_len * row];
            if digit == b' ' {
                continue;
            }

            col_has_data = true;

            let digit = char::from_u32(digit as u32).unwrap().to_digit(10).unwrap();

            num *= 10;
            num += digit as u64;
        }

        if !col_has_data {
            result += current_op_result;

            if let Some(o) = ops.next() {
                op = o;
            } else {
                break;
            }

            current_op_result = get_op_identity(op);

            continue;
        }

        match op {
            "+" => {
                current_op_result += num;
            }
            "*" => {
                current_op_result *= num;
            }
            other => {
                panic!("unexpected op: {other}");
            }
        }
    }

    result += current_op_result;

    result
}

fn get_op_identity(op: &str) -> u64 {
    if op == "+" { 0 } else { 1 }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   + ";

    #[test]
    fn p1() {
        assert_eq!(solve_part_1(TEST_DATA), 4277556);
    }

    #[test]
    fn p2() {
        assert_eq!(solve_part_2(TEST_DATA), 3263827);
    }
}
