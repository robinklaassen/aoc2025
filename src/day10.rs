use std::collections::{HashSet, VecDeque};
use z3::ast::Int;
use z3::Solver;

type Lights = Vec<u8>;
type Buttons = Vec<Vec<usize>>;
type Joltages = Vec<u16>;

struct Machine {
    lights_target: Lights,
    buttons: Buttons,
    joltages: Joltages,
}

impl Machine {
    fn from_line(line: &String) -> Self {
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

    fn press_button2(&self, joltages: &Joltages, button_index: usize) -> Joltages {
        let mut new = joltages.clone();
        for &i in &self.buttons[button_index] {
            new[i] += 1;
        }
        new
    }

    fn joltage_state_valid(&self, state: &Joltages) -> bool {
        for (i, &j) in state.iter().enumerate() {
            if j > self.joltages[i] {
                return false;
            }
        }
        true
    }

    // Using Z3 solver to find minimal button presses as this is a linear programming problem
    // Not very fast but works for now
    fn solve_joltages(&self) -> usize {
        let button_presses: Vec<Int> = (0..self.buttons.len())
            .map(|i| Int::fresh_const(&i.to_string()))
            .collect();

        let solver = Solver::new();

        // constraints: all button presses >= 0
        for btn in &button_presses {
            solver.assert(&btn.ge(0));
        }

        // constraints: for each joltage limit, sum of presses of buttons affecting it == limit
        for (i, joltage_limit) in self.joltages.iter().enumerate() {
            let mut expr = Int::from_i64(0);
            for (btn_idx, btn) in self.buttons.iter().enumerate() {
                if !btn.contains(&i) {
                    continue;
                }
                expr = expr + &button_presses[btn_idx];
            }
            solver.assert(&expr.eq(&Int::from_u64(*joltage_limit as u64)));
        }

        if solver.check() != z3::SatResult::Sat {
            panic!("No solution found");
        }

        solver.solutions(button_presses, true).map(|s| {
            // println!("Solution: {:?}", s);
            s.iter().map(|v| v.as_u64().unwrap() as usize).sum()
        }).min().unwrap()
    }
}

fn part1(lines: &Vec<String>) -> usize {
    let machines: Vec<Machine> = lines.iter().map(Machine::from_line).collect();
    machines.iter().map(|m| m.solve_lights()).sum()
}

fn part2(lines: &Vec<String>) -> usize {
    let machines: Vec<Machine> = lines.iter().map(Machine::from_line).collect();
    machines.iter().map(|m| m.solve_joltages()).sum()
}

pub fn main() {
    let test_lines = crate::utils::read_lines("input_test/day10.txt");
    let input_lines = crate::utils::read_lines("input/day10.txt");

    assert_eq!(part1(&test_lines), 7);
    println!("Day 10 part 1 answer: {}", part1(&input_lines));

    assert_eq!(part2(&test_lines), 33);
    println!("Day 10 part 2 test passed");
    println!("Day 10 part 2 answer: {}", part2(&input_lines));
}
