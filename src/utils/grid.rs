use std::collections::HashMap;

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
