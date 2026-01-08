fn solve_line(line: &str, num_digits: usize) -> i64 {
    let mut output = String::new();

    let digits = line
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect::<Vec<i64>>();

    let mut slice_start = 0;
    for d in 0..num_digits {
        let slice = &digits[slice_start..digits.len() - (num_digits - d - 1)];
        let max_digit = slice.iter().max().unwrap();
        let max_digit_index = slice.iter().position(|&x| x == *max_digit).unwrap();

        output.push_str(&max_digit.to_string());
        slice_start += max_digit_index + 1;
    }

    output.parse::<i64>().unwrap()
}

fn part1(lines: &[String]) -> i64 {
    lines.iter().map(|line| solve_line(line, 2)).sum()
}

fn part2(lines: &[String]) -> i64 {
    lines.iter().map(|line| solve_line(line, 12)).sum()
}

pub fn main() {
    let test_lines = crate::utils::read_lines("input_test/day03.txt");
    let input_lines = crate::utils::read_lines("input/day03.txt");

    assert_eq!(part1(&test_lines), 357);
    println!("Day 3 part 1 answer: {}", part1(&input_lines));

    assert_eq!(part2(&test_lines), 3121910778619);
    println!("Day 3 part 2 answer: {}", part2(&input_lines));
}
