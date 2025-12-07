use std::collections::HashSet;

pub fn solve_part_1(input: &str) -> u64 {
    let mut lines = input.trim().lines();

    let line = lines.next().unwrap().as_bytes();
    let start_pos = line.iter().position(|c| *c == b'S').unwrap();

    let mut beams = HashSet::new();
    beams.insert(start_pos);

    let mut split_count = 0;
    for line in lines {
        let line = line.as_bytes();

        for beam_index in beams.clone() {
            if line[beam_index] == b'^' {
                beams.remove(&beam_index);
                beams.insert(beam_index - 1);
                beams.insert(beam_index + 1);

                split_count += 1;
            }
        }
    }

    split_count
}

pub fn solve_part_2(input: &str) -> u64 {
    let mut lines = input.trim().lines();

    let line = lines.next().unwrap().as_bytes();
    let start_pos = line.iter().position(|c| *c == b'S').unwrap();

    let mut timelines = vec![0u64; line.len()];
    timelines[start_pos] = 1;

    for line_str in lines {
        let line = line_str.as_bytes();

        for i in 0..line.len() {
            if line[i] == b'^' {
                if i > 0 {
                    timelines[i - 1] += timelines[i];
                }

                if i < timelines.len() - 1 {
                    timelines[i + 1] += timelines[i];
                }

                timelines[i] = 0;
            }
        }
    }

    timelines.iter().map(|c| *c as u64).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn p1() {
        assert_eq!(solve_part_1(TEST_DATA), 21);
    }

    #[test]
    fn p2() {
        assert_eq!(solve_part_2(TEST_DATA), 40);
    }

    #[test]
    fn t1() {
        let data = ".......S.......
.......|.......
......|^|......
......|.|......
.....|^|^|.....
.....|.|.|.....
....|^|^|^|....
....|.|.|.|....";

        assert_eq!(solve_part_2(data), 8);
    }
}
