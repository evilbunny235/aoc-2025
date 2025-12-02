pub fn solve_part_1(s: &str) -> u64 {
    let mut result = 0;

    for pair in s.split(',') {
        let (start, end) = pair.split_once('-').unwrap();

        let start: u64 = start.trim().parse().unwrap();
        let end: u64 = end.trim().parse().unwrap();

        result += (start..=end).filter(is_invalid_id_p1).sum::<u64>();
    }

    result
}

fn count_digits(mut n: u64) -> u32 {
    let mut digit_count = 0;
    while n != 0 {
        n /= 10;
        digit_count += 1;
    }

    digit_count
}

fn is_invalid_id_p1(id: &u64) -> bool {
    let digit_count = count_digits(*id);
    if digit_count % 2 != 0 {
        return false;
    }

    let p = 10u64.pow(digit_count / 2);

    let first_half = *id / p;
    let second_half = *id % p;

    first_half == second_half
}

pub fn solve_part_2(s: &str) -> u64 {
    let mut result = 0;

    for pair in s.split(',') {
        let (start, end) = pair.split_once('-').unwrap();

        let start: u64 = start.trim().parse().unwrap();
        let end: u64 = end.trim().parse().unwrap();

        result += (start..=end).filter(is_invalid_id_p2).sum::<u64>();
    }

    result
}

fn is_invalid_id_p2(id: &u64) -> bool {
    let s = id.to_string().into_bytes();

    let digit_count = s.len();

    for i in 1..=digit_count / 2 {
        if digit_count % i != 0 {
            continue;
        }

        let mut chunks = s.chunks_exact(i);
        let first = chunks.next().unwrap();

        if chunks.all(|c| c == first) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
        1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
        824824821-824824827,2121212118-2121212124";

    #[test]
    fn p1() {
        let result = solve_part_1(&TEST_DATA);
        assert_eq!(result, 1227775554);
    }

    #[test]
    fn p2() {
        let result = solve_part_2(&TEST_DATA);
        assert_eq!(result, 4174379265);
    }
}
