use std::collections::HashMap;
use std::io::BufRead;
use std::{fs::File, io::BufReader};

/// Read a file path into a vector of strings, without newline characters
pub fn read_lines(file_path: &str) -> Vec<String> {
    let file = File::open(file_path).expect("should be able to open file");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|l| l.expect("should be able to parse line"))
        .collect()
}

pub trait Grid2D {
    fn from_lines(lines: &Vec<String>) -> Self;
    fn get_or_default(&self, position: &(i32, i32), default: char) -> char;
}

impl Grid2D for HashMap<(i32, i32), char> {
    fn from_lines(lines: &Vec<String>) -> Self {
        let mut grid = HashMap::new();
        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                grid.insert((x as i32, y as i32), ch);
            }
        }
        grid
    }

    fn get_or_default(&self, position: &(i32, i32), default: char) -> char {
        *self.get(position).unwrap_or(&default)
    }
}

// TODO move directions to a separate module
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Direction {
    pub fn value(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::UpLeft => (-1, -1),
            Direction::UpRight => (1, -1),
            Direction::DownLeft => (-1, 1),
            Direction::DownRight => (1, 1),
        }
    }

    pub fn straight_directions() -> Vec<Direction> {
        vec![
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
    }

    pub fn all_directions() -> Vec<Direction> {
        vec![
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
            Direction::UpLeft,
            Direction::UpRight,
            Direction::DownLeft,
            Direction::DownRight,
        ]
    }
}

pub fn move_in_direction(start: (i32, i32), direction: &Direction, steps: i32) -> (i32, i32) {
    let (dx, dy) = direction.value();
    (start.0 + dx * steps, start.1 + dy * steps)
}
