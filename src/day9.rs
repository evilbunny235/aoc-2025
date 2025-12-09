pub fn solve_part_1(input: &str) -> u64 {
    let lines = input.trim().lines();

    let coords = lines
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();

            let x: u32 = x.parse().unwrap();
            let y: u32 = y.parse().unwrap();

            (x, y)
        })
        .collect::<Vec<_>>();

    let mut max_rectangle_area = 0u64;

    for i in 0..coords.len() - 1 {
        for j in i + 1..coords.len() {
            let width = (coords[i].0 as i64 - coords[j].0 as i64).abs() as u64 + 1;
            let height = (coords[i].1 as i64 - coords[j].1 as i64).abs() as u64 + 1;

            max_rectangle_area = max_rectangle_area.max(width * height);
        }
    }

    max_rectangle_area
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn p1() {
        assert_eq!(solve_part_1(TEST_DATA), 50);
    }
}
