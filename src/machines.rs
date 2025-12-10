use good_lp::{
    Expression, IntoAffineExpression, Solution, SolverModel, highs, variable, variables,
};
use std::collections::{HashMap, HashSet, VecDeque};

pub struct Machine {
    lights: usize,
    buttons: Vec<Button>,
    joltages: Vec<u64>,
}

impl Machine {
    pub fn new(lights: usize, buttons: Vec<Button>, joltages: Vec<u64>) -> Self {
        Self {
            lights,
            buttons,
            joltages,
        }
    }

    pub fn fewest_buttons_lights(&self) -> Option<u64> {
        let mut queue = VecDeque::new();
        for button in 0..self.buttons.len() {
            queue.push_back((1, self.buttons[button].state()));
        }

        println!("desired state: {:b}", self.lights);
        while let Some((presses, state)) = queue.pop_front() {
            println!("presses: {presses}, state: {state:b}");
            if state == self.lights {
                return Some(presses);
            }

            if presses as usize > self.buttons.len() {
                break;
            }

            for button in 0..self.buttons.len() {
                queue.push_back((presses + 1, state ^ self.buttons[button].state()));
            }
        }

        None
    }

    pub fn fewest_buttons_joltage(&self) -> Option<u64> {
        let mut vars = variables!();
        let button_presses: Vec<_> = (0..self.buttons.len())
            .map(|_| vars.add(variable().min(0).integer()))
            .collect();

        let mut problem = highs(vars.minimise(button_presses.iter().sum::<Expression>()));
        let mut jolts = vec![0.into_expression(); self.joltages.len()];
        for b in 0..self.buttons.len() {
            for &light in &self.buttons[b].lights {
                // sum each button that affected this counter
                jolts[light] += button_presses[b];
            }
        }

        for (presses, target_joltage) in jolts.into_iter().zip(&self.joltages) {
            problem.add_constraint(presses.eq(*target_joltage as f64));
        }

        let solution = problem.solve().ok()?;

        let mut sum = 0.0;
        for presses in button_presses {
            let v = solution.value(presses);

            sum += v;
        }

        Some(sum as u64)
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Button {
    lights: Vec<usize>,
    state: usize,
}

impl Button {
    pub fn new(lights: Vec<usize>, state: usize) -> Self {
        Self { lights, state }
    }

    pub fn state(&self) -> usize {
        self.state
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn machine_3_test() {
        let buttons = vec![
            Button::new(vec![0, 1, 2, 3, 4], 0b11111),
            Button::new(vec![0, 3, 4], 0b11001),
            Button::new(vec![0, 1, 2, 4, 5], 0b110111),
            Button::new(vec![1, 2], 0b110),
        ];

        let machine = Machine::new(0b101110, buttons, Vec::new());
        assert_eq!(machine.fewest_buttons_lights().unwrap(), 2);
    }
}
