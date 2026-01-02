use geo::{Contains, Coord, LineString, Polygon, Rect};
use itertools::Itertools;

fn line_to_coord(line: &str) -> Coord {
    let parts = line.split(',').collect::<Vec<_>>();
    Coord {
        x: parts[0].parse().unwrap(),
        y: parts[1].parse().unwrap(),
    }
}

fn rect_area(p1: &Coord, p2: &Coord) -> usize {
    let width = (p1.x - p2.x).abs() as usize + 1;
    let height = (p1.y - p2.y).abs() as usize + 1;
    width * height
}

fn part1(lines: &Vec<String>) -> usize {
    let points: Vec<Coord> = lines.iter().map(|l| line_to_coord(l)).collect();
    points
        .iter()
        .combinations(2)
        .map(|pair| rect_area(pair[0], pair[1]))
        .max()
        .unwrap()
}

// This solution works but is relatively slow
fn part2(lines: &Vec<String>) -> usize {
    let points: Vec<Coord> = lines.iter().map(|l| line_to_coord(l)).collect();
    let polygon = Polygon::new(LineString::from(points.clone()), vec![]);

    points
        .iter()
        .combinations(2)
        .filter(|pair| {
            // check if rectangle formed by pair is within polygon
            let rect = Rect::new(pair[0].clone(), pair[1].clone());
            polygon.contains(&rect)
        })
        .map(|pair| rect_area(pair[0], pair[1]))
        .max()
        .unwrap()
}

pub fn main() {
    let test_lines = crate::utils::read_lines("input_test/day09.txt");
    let input_lines = crate::utils::read_lines("input/day09.txt");

    assert_eq!(part1(&test_lines), 50);
    println!("Day 9 part 1 answer: {}", part1(&input_lines));

    assert_eq!(part2(&test_lines), 24);
    println!("Day 9 part 2 answer: {}", part2(&input_lines));
}
