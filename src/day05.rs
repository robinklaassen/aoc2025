fn prep_data(lines: &[String]) -> (Vec<(i64, i64)>, Vec<i64>) {
    let mut is_range_part = true;
    let mut ranges: Vec<(i64, i64)> = Vec::new();
    let mut ids: Vec<i64> = Vec::new();
    for line in lines {
        if line.is_empty() {
            is_range_part = false;
            continue;
        }

        if is_range_part {
            let parts: Vec<&str> = line.split('-').collect();
            let start: i64 = parts[0].parse().unwrap();
            let end: i64 = parts[1].parse().unwrap();
            ranges.push((start, end));
        } else {
            ids.push(line.parse().unwrap());
        }
    }
    (ranges, ids)
}

fn part1(lines: &[String]) -> i32 {
    let (ranges, ids) = prep_data(lines);

    // check each id against the ranges
    ids.iter()
        .filter(|&&id| ranges.iter().any(|&(start, end)| id >= start && id <= end))
        .count() as i32
}

fn part2(lines: &[String]) -> i64 {
    let mut answer = 0;
    let (mut ranges, _) = prep_data(lines);

    // sort them on start ascending
    ranges.sort_by(|a, b| a.0.cmp(&b.0));

    // merge overlapping ranges by comparing current end to next start
    let (mut start, mut end) = ranges[0];
    for (next_start, next_end) in ranges.iter().skip(1) {
        if *next_start <= end {
            // ranges overlap, extend the end if needed
            end = end.max(*next_end);
        } else {
            // no overlap, add the current range to the answer and start a new one
            answer += end - start + 1;
            start = *next_start;
            end = *next_end;
        }
    }

    // add the last range
    answer += end - start + 1;

    answer
}

pub fn main() {
    let test_lines = crate::utils::read_lines("input_test/day05.txt");
    let input_lines = crate::utils::read_lines("input/day05.txt");

    assert_eq!(part1(&test_lines), 3);
    println!("Day 5 part 1 answer: {}", part1(&input_lines));

    assert_eq!(part2(&test_lines), 14);
    println!("Day 5 part 2 answer: {}", part2(&input_lines));
}
