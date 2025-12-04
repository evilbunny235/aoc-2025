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

fn count_adjacent_rolls(lines: &[&[u8]], row_index: usize, col_index: usize) -> u8 {
    let mut count = 0;

    if row_index > 0 {
        count += count_rolls_on_line(lines, row_index - 1, col_index);
    }

    count += count_rolls_on_line(lines, row_index, col_index);

    // subtract 1 because it's the actual roll we want to check around
    count -= 1;

    if row_index < lines[row_index].len() - 1 {
        count += count_rolls_on_line(lines, row_index + 1, col_index);
    }

    count
}

fn count_rolls_on_line(lines: &[&[u8]], row_index: usize, col_index: usize) -> u8 {
    let mut count = 0;
    if col_index > 0 {
        if lines[row_index][col_index - 1] == b'@' {
            count += 1;
        }
    }

    if lines[row_index][col_index] == b'@' {
        count += 1;
    }

    if col_index < lines[row_index].len() - 1 {
        if lines[row_index][col_index + 1] == b'@' {
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
}
