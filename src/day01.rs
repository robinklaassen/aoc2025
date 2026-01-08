use crate::utils;

fn parse_line(line: String) -> i32 {
    let mut chars = line.chars();
    let direction = chars.next().expect("should find first character");
    let count = chars
        .as_str()
        .parse::<i32>()
        .expect("should parse string into int");
    if direction == 'R' { count } else { -count }
}

fn count_zero_passes(start: i32, rotation: i32) -> i32 {
    if !(0..100).contains(&start) {
        panic!("Start has incorrect value: {start}");
    }

    let mut zero_passes = 0;

    // for any absolute rotation 100 or greater, there is always a number of zero passes regardless of end position
    // in rust, integer division rounds towards zero (truncating division)
    zero_passes += (rotation / 100).abs();

    // then some analysis remains
    let rot_remainder = rotation % 100; // the remainder can be negative but is between -100 and 100 exclusive
    let new_position = start + rot_remainder;
    if new_position >= 100 {
        zero_passes += 1
    }; // zero pass while turning right
    if start != 0 && new_position <= 0 {
        zero_passes += 1
    } // zero pass while turning left

    zero_passes
}

fn part1(lines: &Vec<String>) -> i32 {
    let mut position = 50;
    let mut zero_counter = 0;
    for line in lines {
        let rotation = parse_line(line.to_string());
        position = (position + rotation).rem_euclid(100);
        if position == 0 {
            zero_counter += 1
        }
    }
    zero_counter
}

fn part2(lines: &Vec<String>) -> i32 {
    let mut position = 50;
    let mut zero_counter = 0;
    for line in lines {
        let rotation = parse_line(line.to_string());
        zero_counter += count_zero_passes(position, rotation);
        position = (position + rotation).rem_euclid(100);
    }
    zero_counter
}

pub fn main() {
    let test_lines = utils::read_lines("input_test/day01.txt");
    let input_lines = utils::read_lines("input/day01.txt");

    // part 1
    assert_eq!(part1(&test_lines), 3);
    println!("Day 1 part 1 answer: {}", part1(&input_lines));

    // part 2
    assert_eq!(count_zero_passes(50, 1), 0);
    assert_eq!(count_zero_passes(50, 80), 1);
    assert_eq!(count_zero_passes(50, 180), 2);
    assert_eq!(count_zero_passes(50, -50), 1);
    assert_eq!(count_zero_passes(50, 50), 1);
    assert_eq!(count_zero_passes(50, -1), 0);
    assert_eq!(count_zero_passes(50, -80), 1);
    assert_eq!(count_zero_passes(50, -180), 2);
    assert_eq!(count_zero_passes(0, 50), 0);
    assert_eq!(count_zero_passes(0, 100), 1);
    assert_eq!(count_zero_passes(0, 150), 1);
    assert_eq!(count_zero_passes(0, -50), 0);
    assert_eq!(count_zero_passes(0, -100), 1);
    assert_eq!(count_zero_passes(0, -150), 1);

    assert_eq!(part2(&test_lines), 6);
    println!("Day 1 part 2 answer: {}", part2(&input_lines));
}
