pub fn solve_part_1(input: &str) -> u64 {
    let mut result = 0;

    let lines: Vec<_> = input.lines().map(|l| l.as_bytes()).collect();

    let line_len = lines[0].len();
    lines.iter().all(|l| l.len() == line_len);

    for row_index in 0..lines.len() {
        for col_index in 0..lines[row_index].len() {
            if lines[row_index][col_index] != b'@' {
                continue;
            }

            if count_adjacent_rolls(&lines, row_index, col_index) < 4 {
                result += 1;
            }
        }
    }

    result
}

pub fn solve_part_2(input: &str) -> u64 {
    let mut result = 0;

    let mut lines: Vec<_> = input.lines().map(|l| l.as_bytes().to_vec()).collect();

    let line_len = lines[0].len();
    lines.iter().all(|l| l.len() == line_len);

    loop {
        let mut rolls_removed_this_iteration = 0;
        for row_index in 0..lines.len() {
            for col_index in 0..lines[row_index].len() {
                if lines[row_index][col_index] != b'@' {
                    continue;
                }

                if count_adjacent_rolls(&lines, row_index, col_index) < 4 {
                    rolls_removed_this_iteration += 1;

                    lines[row_index][col_index] = b'.';
                }
            }
        }

        result += rolls_removed_this_iteration;

        if rolls_removed_this_iteration == 0 {
            break;
        }
    }

    result
}

fn count_adjacent_rolls<T: AsRef<[u8]>>(lines: &[T], row_index: usize, col_index: usize) -> u8 {
    let mut count = 0;

    if row_index > 0 {
        count += count_rolls_on_line(lines, row_index - 1, col_index);
    }

    count += count_rolls_on_line(lines, row_index, col_index);

    // subtract 1 because it's the actual roll we want to check around
    count -= 1;

    if row_index < lines[row_index].as_ref().len() - 1 {
        count += count_rolls_on_line(lines, row_index + 1, col_index);
    }

    count
}

fn count_rolls_on_line<T: AsRef<[u8]>>(lines: &[T], row_index: usize, col_index: usize) -> u8 {
    let mut count = 0;
    let line = lines[row_index].as_ref();

    if col_index > 0 {
        if line[col_index - 1] == b'@' {
            count += 1;
        }
    }

    if line[col_index] == b'@' {
        count += 1;
    }

    if col_index < line.len() - 1 {
        if line[col_index + 1] == b'@' {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn p1() {
        assert_eq!(solve_part_1(TEST_DATA), 13);
    }

    #[test]
    fn p2() {
        assert_eq!(solve_part_2(TEST_DATA), 43);
    }
}
