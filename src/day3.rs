pub fn solve_part_1(buf: &str) -> u64 {
    let mut sum = 0;

    for line in buf.split('\n') {
        if line.is_empty() {
            continue;
        }

        let line = line.as_bytes();

        // the reverse is necessary because max_by_key returns the last max, not the first
        let (pos_first_max, first_max) = line[..line.len() - 1]
            .iter()
            .enumerate()
            .rev()
            .max_by_key(|(_index, val)| *val)
            .unwrap();

        let first_max = char::from_u32(*first_max as u32)
            .unwrap()
            .to_digit(10)
            .unwrap();

        let second_max = line[pos_first_max + 1..].iter().max().unwrap();

        let second_max = char::from_u32(*second_max as u32)
            .unwrap()
            .to_digit(10)
            .unwrap();

        let max_joltage = first_max * 10 + second_max;
        sum += max_joltage as u64;
    }

    sum
}

pub fn solve_part_2(buf: &str) -> u64 {
    let mut sum = 0;

    for line in buf.split('\n') {
        if line.is_empty() {
            continue;
        }

        let line = line.as_bytes();

        let mut max_joltage = 0;
        let mut range_start = 0;

        for i in 1..=12 {
            let range_end = line.len() - 12 + i;
            let (max, pos) = find_first_max_in_slice(&line[range_start..range_end]);
            range_start = range_start + pos + 1;
            max_joltage *= 10;
            max_joltage += max as u64;
        }

        sum += max_joltage as u64;
    }

    sum
}

fn find_first_max_in_slice(s: &[u8]) -> (u32, usize) {
    // the reverse is necessary because max_by_key returns the last max, not the first
    let (pos, first_max) = s
        .iter()
        .enumerate()
        .rev()
        .max_by_key(|(_index, val)| *val)
        .unwrap();

    let first_max = char::from_u32(*first_max as u32)
        .unwrap()
        .to_digit(10)
        .unwrap();

    (first_max, pos)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn p1() {
        let result = solve_part_1(TEST_DATA);

        assert_eq!(result, 357);
    }

    #[test]
    fn p2() {
        assert_eq!(solve_part_2("987654321111111"), 987654321111);
        assert_eq!(solve_part_2("811111111111119"), 811111111119);
        assert_eq!(solve_part_2("234234234234278"), 434234234278);
        assert_eq!(solve_part_2("818181911112111"), 888911112111);
    }
}
