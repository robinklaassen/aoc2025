use std::collections::HashMap;

use itertools::Itertools;

#[derive(Eq, Hash, PartialEq)]
struct Point3D {
    x: i64,
    y: i64,
    z: i64,
}

impl Point3D {
    fn from_line(line: &str) -> Point3D {
        let parts: Vec<&str> = line.split(',').collect();
        assert_eq!(parts.len(), 3);
        Point3D {
            x: parts[0].parse().unwrap(),
            y: parts[1].parse().unwrap(),
            z: parts[2].parse().unwrap(),
        }
    }

    /// Calculates the Euclidean distance between two 3D points.
    fn distance(&self, other: &Point3D) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        ((dx * dx + dy * dy + dz * dz) as f64).sqrt()
    }
}

fn sorted_pairwise_distances(points: &Vec<Point3D>) -> Vec<((&Point3D, &Point3D), f64)> {
    let mut distances: Vec<_> = points
        .iter()
        .combinations(2)
        .map(|pair| ((pair[0], pair[1]), pair[0].distance(pair[1])))
        .collect();
    distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    distances
}

pub fn part1(lines: &Vec<String>, num_connections: usize) -> i64 {
    let points: Vec<Point3D> = lines.iter().map(|line| Point3D::from_line(line)).collect();
    let mut groups: HashMap<&Point3D, usize> =
        points.iter().enumerate().map(|(i, p)| (p, i)).collect();

    // Connect given amount of pairs starting from the shortest distance
    let distances = sorted_pairwise_distances(&points);
    for ((p1, p2), _dist) in distances.iter().take(num_connections) {
        let group1 = groups.get(p1).unwrap();
        let group2 = groups.get(p2).unwrap();
        if group1 != group2 {
            // Merge groups
            let old_group = *group2;
            let new_group = *group1;
            for (_p, g) in groups.iter_mut() {
                if *g == old_group {
                    *g = new_group;
                }
            }
        }
    }

    // Count sizes per group id
    let mut counts: HashMap<usize, usize> = HashMap::new();
    for &g in groups.values() {
        *counts.entry(g).or_insert(0) += 1;
    }

    counts.values().sorted().rev().take(3).product::<usize>() as i64
}

fn part2(lines: &Vec<String>) -> i64 {
    let mut answer = 0;
    let points: Vec<Point3D> = lines.iter().map(|line| Point3D::from_line(line)).collect();
    let mut groups: HashMap<&Point3D, usize> =
        points.iter().enumerate().map(|(i, p)| (p, i)).collect();

    let distances = sorted_pairwise_distances(&points);
    for ((p1, p2), _dist) in distances {
        let group1 = groups.get(p1).unwrap();
        let group2 = groups.get(p2).unwrap();
        if group1 != group2 {
            // Merge groups
            let old_group = *group2;
            let new_group = *group1;
            for (_p, g) in groups.iter_mut() {
                if *g == old_group {
                    *g = new_group;
                }
            }
        }

        // Check if all points are in the same group
        let first_group = groups.values().next().unwrap();
        if groups.values().all(|&g| g == *first_group) {
            answer = p1.x * p2.x;
            break;
        }
    }

    answer
}

pub fn main() {
    let test_lines = crate::utils::read_lines("input_test/day08.txt");
    let input_lines = crate::utils::read_lines("input/day08.txt");

    assert_eq!(part1(&test_lines, 10), 40);
    println!("Day 8 part 1 answer: {}", part1(&input_lines, 1000));

    assert_eq!(part2(&test_lines), 25272);
    println!("Day 8 part 2 answer: {}", part2(&input_lines));
}
