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

    #[allow(dead_code)]
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
