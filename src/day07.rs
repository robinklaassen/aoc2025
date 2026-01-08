use std::collections::{HashMap, HashSet};

fn find_all_char_indices(s: &str, target: char) -> Vec<usize> {
    s.char_indices()
        .filter_map(|(i, c)| if c == target { Some(i) } else { None })
        .collect()
}

fn part1(lines: &[String]) -> i32 {
    let mut split_count = 0;
    let mut tachyon_columns: HashSet<usize> = HashSet::new();

    for (row, line) in lines.iter().enumerate() {
        if row == 0 {
            tachyon_columns.insert(line.find('S').unwrap());
            continue;
        }

        let splitter_indices = find_all_char_indices(line, '^');
        for i in splitter_indices {
            if !tachyon_columns.contains(&i) {
                continue;
            }

            split_count += 1;
            tachyon_columns.remove(&i);
            tachyon_columns.insert(i - 1);
            tachyon_columns.insert(i + 1);
        }
    }
    split_count
}

fn part2(lines: &[String]) -> i64 {
    // This is going to be basically the same as part 1 but with a map instead of a set so we can count beams
    // I could refactor to combine both approaches, but I'll leave it for history's sake
    let mut beams: HashMap<usize, i64> = HashMap::new();

    for (row, line) in lines.iter().enumerate() {
        if row == 0 {
            beams.insert(line.find('S').unwrap(), 1);
            continue;
        }

        let splitter_indices = find_all_char_indices(line, '^');
        for i in splitter_indices {
            if !beams.contains_key(&i) {
                continue;
            }

            let count = *beams.get(&i).unwrap();
            beams.remove(&i);
            *beams.entry(i - 1).or_insert(0) += count;
            *beams.entry(i + 1).or_insert(0) += count;
        }
    }

    beams.values().sum()
}

pub fn main() {
    let test_lines = crate::utils::read_lines("input_test/day07.txt");
    let input_lines = crate::utils::read_lines("input/day07.txt");

    assert_eq!(part1(&test_lines), 21);
    println!("Day 7 part 1 answer: {}", part1(&input_lines));

    assert_eq!(part2(&test_lines), 40);
    println!("Day 7 part 2 answer: {}", part2(&input_lines));
}
