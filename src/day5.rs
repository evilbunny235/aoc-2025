pub fn solve_part_1(input: &str) -> u64 {
    let mut lines = input.lines();

    let ranges = read_ranges(&mut lines);
    count_ids(lines, &ranges)
}

fn read_ranges(lines: &mut std::str::Lines<'_>) -> Vec<(u64, u64)> {
    let mut ranges = vec![];
    while let Some(line) = lines.next()
        && !line.is_empty()
    {
        let (start, end) = line.split_once('-').unwrap();
        let start: u64 = start.parse().unwrap();
        let end: u64 = end.parse().unwrap();

        ranges.push((start, end));
    }

    ranges
}

fn count_ids(mut lines: std::str::Lines<'_>, ranges: &[(u64, u64)]) -> u64 {
    let mut count = 0;
    while let Some(line) = lines.next() {
        let id: u64 = line.parse().unwrap();

        if ranges.iter().any(|r| id >= r.0 && id <= r.1) {
            count += 1;
        }
    }

    count
}

pub fn solve_part_2(input: &str) -> u64 {
    let mut lines = input.lines();

    let ranges = read_ranges(&mut lines);
    let ranges = merge_overlapping_ranges(ranges);

    ranges.iter().map(|(start, end)| end - start + 1).sum()
}

pub fn merge_overlapping_ranges(mut ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut result = Vec::with_capacity(ranges.len());

    ranges.sort();

    for range_to_merge in ranges {
        if let Some(range_to_merge_with) = result
            .iter_mut()
            .find(|(start, end)| range_to_merge.0 >= *start && range_to_merge.0 <= *end)
        {
            range_to_merge_with.1 = range_to_merge_with.1.max(range_to_merge.1);
        } else {
            result.push(range_to_merge);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn p1() {
        assert_eq!(solve_part_1(TEST_DATA), 3);
    }

    #[test]
    fn p2() {
        assert_eq!(solve_part_2(TEST_DATA), 14);

        assert!(false);
    }
}
