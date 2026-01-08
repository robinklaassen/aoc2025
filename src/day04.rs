use crate::utils::direction::{Direction, move_in_direction};
use crate::utils::grid::Grid2D;
use std::collections::HashMap;

type PaperGrid = HashMap<(i32, i32), char>;

/// Get all accessible paper positions ('@') that have less than 4 adjacent papers
fn get_accessible(grid: &PaperGrid) -> Vec<(i32, i32)> {
    let mut accessible = Vec::new();

    for (pos, val) in grid {
        if *val != '@' {
            continue;
        }

        let adj_paper_count = Direction::all_directions()
            .iter()
            .map(|d| move_in_direction(*pos, d, 1))
            .map(|p| grid.get_or_default(&p, '.'))
            .filter(|&c| c == '@')
            .count();

        if adj_paper_count < 4 {
            accessible.push(*pos);
        }
    }

    accessible
}

fn part1(lines: &[String]) -> i32 {
    let grid = HashMap::from_lines(lines);
    get_accessible(&grid).len() as i32
}

fn part2(lines: &[String]) -> i32 {
    let mut grid = HashMap::from_lines(lines);
    let mut answer = 0;
    let mut done = false;

    while !done {
        let accessible = get_accessible(&grid);
        done = accessible.is_empty();

        answer += accessible.len() as i32;

        for pos in accessible {
            // mark position as processed
            grid.insert(pos, '.');
        }
    }

    answer
}

pub fn main() {
    let test_lines = crate::utils::read_lines("input_test/day04.txt");
    let input_lines = crate::utils::read_lines("input/day04.txt");

    assert_eq!(part1(&test_lines), 13);
    println!("Day 4 part 1 answer: {}", part1(&input_lines));

    assert_eq!(part2(&test_lines), 43);
    println!("Day 4 part 2 answer: {}", part2(&input_lines));
}
