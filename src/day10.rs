use std::collections::{HashSet, VecDeque};
use z3::Optimize;
use z3::ast::Int;

type Lights = Vec<u8>;
type Buttons = Vec<Vec<usize>>;
type Joltages = Vec<u16>;

struct Machine {
    lights_target: Lights,
    buttons: Buttons,
    joltages: Joltages,
}

impl Machine {
    fn from_line(line: &str) -> Self {
        let mut lights_target: Lights = Vec::new();
        let mut buttons: Buttons = Vec::new();
        let mut joltages: Joltages = Vec::new();

        for part in line.split(" ") {
            if part.starts_with("[") {
                for ch in part.chars().skip(1).take(part.len() - 2) {
                    match ch {
                        '.' => lights_target.push(0u8),
                        '#' => lights_target.push(1u8),
                        _ => {}
                    }
                }
            }
            if part.starts_with("(") {
                let btn: Vec<usize> = part
                    .trim_start_matches('(')
                    .trim_end_matches(')')
                    .split(',')
                    .map(|s| s.parse().unwrap())
                    .collect();
                buttons.push(btn);
            }
            if part.starts_with("{") {
                joltages = part
                    .trim_start_matches('{')
                    .trim_end_matches('}')
                    .split(',')
                    .map(|s| s.parse().unwrap())
                    .collect();
            }
        }

        assert!(!lights_target.is_empty());
        assert!(!buttons.is_empty());
        assert!(!joltages.is_empty());

        Machine {
            lights_target,
            buttons,
            joltages,
        }
    }

    fn press_button(&self, lights: &Lights, button_index: usize) -> Lights {
        let mut new = lights.clone();
        for &i in &self.buttons[button_index] {
            new[i] ^= 1;
        }
        new
    }

    fn solve_lights(&self) -> usize {
        let initial_state: Lights = vec![0u8; self.lights_target.len()];
        let mut visited: HashSet<Lights> = HashSet::new();
        let mut queue: VecDeque<(Lights, usize)> = VecDeque::new();
        queue.push_back((initial_state.clone(), 0));
        visited.insert(initial_state);

        while let Some((current_state, presses)) = queue.pop_front() {
            if current_state == self.lights_target {
                return presses;
            }

            for btn_idx in 0..self.buttons.len() {
                let new_state = self.press_button(&current_state, btn_idx);
                if visited.insert(new_state.clone()) {
                    queue.push_back((new_state, presses + 1));
                }
            }
        }

        unreachable!("No solution found");
    }

    // Using Z3 optimizer to find minimal button presses as this is an optimization problem
    fn solve_joltages(&self) -> usize {
        let button_presses: Vec<Int> = (0..self.buttons.len())
            .map(|i| Int::fresh_const(&i.to_string()))
            .collect();

        let opt = Optimize::new();

        // constraints: all button presses >= 0
        for btn in &button_presses {
            opt.assert(&btn.ge(0));
        }

        // constraints: for each joltage limit, sum of presses of buttons affecting it == limit
        for (i, joltage_limit) in self.joltages.iter().enumerate() {
            let mut expr = Int::from_i64(0);
            for (btn_idx, btn) in self.buttons.iter().enumerate() {
                if !btn.contains(&i) {
                    continue;
                }
                expr += &button_presses[btn_idx];
            }
            opt.assert(&expr.eq(Int::from_u64(*joltage_limit as u64)));
        }

        // Objective: minimize total number of presses
        let mut total = Int::from_i64(0);
        for btn in &button_presses {
            total += btn;
        }
        opt.minimize(&total);

        if opt.check(&[]) != z3::SatResult::Sat {
            panic!("No solution found");
        }

        let model = opt.get_model().unwrap();
        button_presses
            .iter()
            .map(|b| model.eval(b, true).unwrap().as_u64().unwrap() as usize)
            .sum()
    }
}

fn part1(lines: &[String]) -> usize {
    let machines: Vec<Machine> = lines.iter().map(|l| Machine::from_line(l)).collect();
    machines.iter().map(|m| m.solve_lights()).sum()
}

fn part2(lines: &[String]) -> usize {
    let machines: Vec<Machine> = lines.iter().map(|l| Machine::from_line(l)).collect();
    machines.iter().map(|m| m.solve_joltages()).sum()
}

pub fn main() {
    let test_lines = crate::utils::read_lines("input_test/day10.txt");
    let input_lines = crate::utils::read_lines("input/day10.txt");

    assert_eq!(part1(&test_lines), 7);
    println!("Day 10 part 1 answer: {}", part1(&input_lines));

    assert_eq!(part2(&test_lines), 33);
    println!("Day 10 part 2 answer: {}", part2(&input_lines));
}
