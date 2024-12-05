use std::i8;

struct Grid {
    values: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

pub fn day4_part1(input: String) -> usize {
    let grid = parse_char_grid(input);
    let mut sum = 0;

    for line_index in 0..grid.values.len() {
        let line = &grid.values[line_index];
        for char_index in 0..line.len() {
            let c = line[char_index];
            if c == 'X' {
                sum += count_xmas(&grid, line_index, char_index);
            }
        }
    }

    sum
}

pub fn day4_part2(input: String) -> usize {
    let grid = parse_char_grid(input);
    let mut sum = 0;

    for line_index in 0..grid.values.len() {
        let line = &grid.values[line_index];
        for char_index in 0..line.len() {
            let c = line[char_index];
            if c == 'A' {
                sum += count_x_mas(&grid, line_index, char_index);
            }
        }
    }

    sum
}

fn count_x_mas(grid: &Grid, line_index: usize, char_index: usize) -> usize {
    if line_index == 0
        || char_index == 0
        || line_index == grid.height - 1
        || char_index == grid.width - 1
    {
        return 0;
    }
    let values = &grid.values;

    let mas1 = String::from_iter([
        values[line_index - 1][char_index - 1],
        values[line_index][char_index],
        values[line_index + 1][char_index + 1],
    ]);
    let mas2 = String::from_iter([
        values[line_index + 1][char_index - 1],
        values[line_index][char_index],
        values[line_index - 1][char_index + 1],
    ]);

    if (mas1 == "MAS" || mas1 == "SAM") && (mas2 == "MAS" || mas2 == "SAM") {
        println!("Found {line_index},{char_index}");
        1
    } else {
        0
    }
}

fn count_xmas(grid: &Grid, line: usize, c: usize) -> usize {
    let mut count = 0;
    for velocity_x in -1..=1 {
        for velocity_y in -1..=1 {
            if velocity_x == 0 && velocity_y == 0 {
                continue;
            }
            count += count_xmas_direction(grid, line, c, velocity_x, velocity_y);
        }
    }

    count
}

fn count_xmas_direction(
    grid: &Grid,
    line_index: usize,
    char_index: usize,
    velocity_x: i8,
    velocity_y: i8,
) -> usize {
    if !check_bound(grid.width, char_index, velocity_x)
        || !check_bound(grid.height, line_index, velocity_y)
    {
        println!("Skip {line_index},{char_index} ({velocity_x}, {velocity_y})");
        return 0;
    }

    let expected_chars = ['X', 'M', 'A', 'S'];

    for i in 1..4 {
        let check_line = (line_index as isize) + (i * velocity_y as isize);
        let check_char = (char_index as isize) + (i * velocity_x as isize);
        if grid.values[check_line as usize][check_char as usize] != expected_chars[i as usize] {
            println!("Failed {line_index},{char_index} ({velocity_x}, {velocity_y}) on {check_line},{check_char}");
            return 0;
        }
    }

    println!("Found {line_index},{char_index} ({velocity_x}, {velocity_y})");
    1
}

fn check_bound(max: usize, start: usize, direction: i8) -> bool {
    match direction {
        0 => true,
        i8::MIN..0 => ((4 * direction).abs() as usize) <= start + 1,
        1..=i8::MAX => (4 * direction) as usize + start <= max,
    }
}

fn parse_char_grid(input: String) -> Grid {
    let values: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let height = values.len();
    let width = values[0].len();

    Grid {
        values,
        height,
        width,
    }
}

#[cfg(test)]
mod test {
    use crate::day4::{day4_part1, day4_part2};

    #[test]
    fn test_part1() {
        assert_eq!(
            18,
            day4_part1(
                r#"
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#
                    .to_string()
            )
        )
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            9,
            day4_part2(
                r#"
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#
                    .to_string()
            )
        )
    }
}
