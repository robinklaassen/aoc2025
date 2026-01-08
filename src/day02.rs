use std::fs;

fn parse_line(line: &String) -> Vec<(i64, i64)> {
    let mut ranges: Vec<(i64, i64)> = Vec::new();

    for range_str in line.split(",") {
        let bla: Vec<&str> = range_str.split("-").collect();
        let start = bla[0].parse::<i64>().unwrap();
        let end = bla[1].parse::<i64>().unwrap();
        ranges.push((start, end));
    }

    ranges
}

/// check if the number is made of a sequence repeated twice
/// uses a combination of integer division and modulus to split the number in half
fn is_invalid(num: i64) -> bool {
    let length = num.to_string().len();
    if length % 2 == 1 {
        return false;
    } // odd numbers never are a sequence repeated twice
    let base: i64 = 10;
    let divisor = base.pow((length / 2).try_into().unwrap());
    let left = num / divisor;
    let right = num % divisor;
    left == right
}

fn part1(line: &String) -> i64 {
    let mut answer = 0;
    let ranges = parse_line(line);

    for (start, end) in ranges {
        answer += (start..=end).filter(|x| is_invalid(*x)).sum::<i64>()
    }

    answer
}

/// extract a block of m digits starting at position p (0-indexed from right)
fn extract_block(n: i64, p: u32, m: u32) -> i64 {
    let pow_p = 10i64.pow(p);
    let pow_m = 10i64.pow(m);
    (n / pow_p) % pow_m
}

fn is_invalid2(num: i64) -> bool {
    // use the new extract_block function!
    let length = num.to_string().len();
    for block_size in 1..=(length / 2) {
        if !length.is_multiple_of(block_size) {
            continue;
        } // for example 5 can never be invalid in segments of 2
        let mut all_equal = true;
        let num_blocks = length / block_size;
        let first_block = extract_block(num, 0, block_size as u32);
        for block_index in 1..num_blocks {
            let current_block =
                extract_block(num, (block_index * block_size) as u32, block_size as u32);
            if current_block != first_block {
                all_equal = false;
                break;
            }
        }
        if all_equal {
            return true;
        }
    }

    false // default return false if no block size matched
}

fn part2(line: &String) -> i64 {
    let mut answer = 0;
    let ranges = parse_line(line);

    for (start, end) in ranges {
        answer += (start..=end).filter(|x| is_invalid2(*x)).sum::<i64>()
    }

    answer
}

pub fn main() {
    let test_line =
        fs::read_to_string("input_test/day02.txt").expect("should be able to read file");
    let input_line = fs::read_to_string("input/day02.txt").expect("should be able to read file");

    assert!(is_invalid(11));
    assert!(is_invalid(1010));
    assert!(is_invalid(446446));

    assert!(!is_invalid(12));
    assert!(!is_invalid(123));
    assert!(!is_invalid(700234098));

    assert_eq!(part1(&test_line), 1227775554);
    println!("Day 2 part 1 answer: {}", part1(&input_line));

    assert!(is_invalid2(111));
    assert!(is_invalid2(1010));
    assert!(is_invalid2(824824824));

    assert!(!is_invalid2(10));
    assert!(!is_invalid2(200));
    assert!(!is_invalid2(3003));

    assert_eq!(part2(&test_line), 4174379265);
    println!("Day 2 part 2 answer: {}", part2(&input_line));
}
