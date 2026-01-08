fn calculate(numbers: &[i32], operator: &str) -> i64 {
    match operator {
        "+" => numbers.iter().map(|&n| n as i64).sum(),
        "*" => numbers.iter().map(|&n| n as i64).product(),
        _ => 0,
    }
}

fn part1(lines: &[String]) -> i64 {
    let mut answer = 0;

    // data prep
    let operators: Vec<&str> = lines.last().unwrap().split_whitespace().collect();
    let mut digits: Vec<Vec<i32>> = Vec::new();
    for line in lines.iter().take(lines.len() - 1) {
        let row: Vec<i32> = line
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect();
        digits.push(row);
    }

    // loop through the columns
    for col in 0..operators.len() {
        let col_values: Vec<i32> = digits.iter().map(|r| r[col]).collect();
        answer += calculate(&col_values, operators[col]);
    }

    answer
}

fn part2(lines: &[String]) -> i64 {
    let mut answer = 0;

    // data prep
    let mut operators = lines.last().unwrap().split_whitespace(); // iterator
    let number_lines = lines[..lines.len() - 1].to_vec();

    let mut numbers: Vec<i32> = Vec::new();
    for c in 0..lines[0].chars().count() {
        let digits = number_lines
            .iter()
            .map(|l| l.chars().nth(c).unwrap())
            .filter(|d| *d != ' ')
            .collect::<Vec<char>>();

        if digits.is_empty() {
            // calculate the answer for this column, add it to the total answer, and reset numbers
            let op = operators.next().unwrap();
            answer += calculate(&numbers, op);
            numbers = Vec::new();
            continue;
        }

        numbers.push(digits.iter().collect::<String>().parse::<i32>().unwrap());
    }

    // final calculation for the last column
    let op = operators.next().unwrap();
    answer += calculate(&numbers, op);

    answer
}

pub fn main() {
    let test_lines = crate::utils::read_lines("input_test/day06.txt");
    let input_lines = crate::utils::read_lines("input/day06.txt");

    assert_eq!(part1(&test_lines), 4277556);
    println!("Day 6 part 1 answer: {}", part1(&input_lines));

    assert_eq!(part2(&test_lines), 3263827);
    println!("Day 6 part 2 answer: {}", part2(&input_lines));
}
