use std::collections::HashMap;

type Present = Vec<Vec<u8>>;
type PresentColl = Vec<Present>;
type Regions = Vec<String>;

fn parse_input(lines: &Vec<String>) -> (PresentColl, Regions) {
    let mut presents: PresentColl = Vec::new();

    let mut regions_index_start = 0;
    let mut present = Present::new();
    for (i, line) in lines.iter().enumerate() {
        if line.len() == 2 {
            continue; // do not process region header lines here
        }

        if line.contains("x") {
            regions_index_start = i;
            break;
        }

        if line.is_empty() {
            presents.push(present);
            present = Present::new();
            continue;
        }

        let row: Vec<u8> = line
            .chars()
            .map(|c| if c == '#' { 1 } else { 0 } as u8)
            .collect();
        present.push(row);
    }

    let regions = lines[regions_index_start..].to_vec();

    (presents, regions)
}

fn part1(input_lines: &Vec<String>) -> usize {
    let (presents, regions) = parse_input(input_lines);
    let present_sizes: Vec<usize> = presents
        .iter()
        .map(|p| p.iter().flatten().filter(|x| **x == 1).count())
        .collect();

    let mut valid_region_count = 0;

    for region in regions {
        let region_parts: Vec<_> = region.split(": ").collect();
        let size_parts: Vec<usize> = region_parts[0]
            .split("x")
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        let present_counts: Vec<usize> = region_parts[1]
            .split(" ")
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        let total_present_count: usize = present_counts.iter().sum();

        // try fitting without packing, every present is assumed 3x3 size
        if total_present_count <= (size_parts[0] / 3) * (size_parts[1] / 3) {
            valid_region_count += 1;
            continue;
        }

        // check if perfect packing would still have too little space
        let area_size: usize = size_parts.iter().product();
        let present_size: usize = present_counts
            .iter()
            .enumerate()
            .map(|(i, pc)| present_sizes[i] * pc)
            .sum();
        if present_size > area_size {
            continue;
        }

        // apparently the real input is structured in such a way that this panic is never triggered
        // this is not so for the examples, that's why we skip testing those
        panic!("Packing is difficult");
    }

    valid_region_count
}

pub fn main() {
    let input_lines = crate::utils::read_lines("input/day12.txt");
    println!("Day 12 part 1 answer: {}", part1(&input_lines));
}
