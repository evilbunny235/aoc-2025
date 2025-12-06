pub fn solve_part_1(input: &str) -> u64 {
    let mut lines = input.lines();

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
}
