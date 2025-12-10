// use std::collections::{HashMap, HashSet, VecDeque};
//
// use good_lp::{Expression, IntoAffineExpression, SolverModel, highs, variable, variables};
// use z3::{Config, Context, Optimize, Solver, ast::Int};
//
// pub struct Machine {
//     lights: Vec<bool>,
//     buttons: Vec<Button>,
//     joltages: Vec<u64>,
// }
//
// impl Machine {
//     pub fn new(desired_state: Vec<bool>, buttons: Vec<Button>, joltages: Vec<u64>) -> Self {
//         Self {
//             lights: desired_state,
//             buttons,
//             joltages,
//         }
//     }
//
//     fn press_buttons(&self, buttons: &HashSet<usize>) -> Vec<bool> {
//         let mut state = vec![false; self.lights.len()];
//         for &i in buttons {
//             for &light in &self.buttons[i].lights {
//                 state[light] = !state[light];
//             }
//         }
//
//         state
//     }
//
//     pub fn fewest_buttons_lights(&self) -> Option<u64> {
//         // let solver = Optimize::new();
//         //
//         // let button_presses: Vec<Int> = (0..self.buttons.len())
//         //     .map(|b| Int::new_const(format!("{}", b).as_str()))
//         //     .collect();
//         //
//         // // For each counter, sum the presses of buttons affecting it
//         // for (counter, &target) in self.lights.iter().enumerate() {
//         //     let mut sum = Int::from_i64(0);
//         //     for b in (0..self.buttons.len()) {
//         //         if self.buttons[b].lights.contains(&counter) {
//         //             sum = sum + &button_presses[b];
//         //         }
//         //     }
//         //
//         //     if target {
//         //         // light should be on
//         //         let modulo = sum % Int::from_i64(2);
//         //         solver.assert(&modulo.eq(Int::from_i64(1)));
//         //     } else {
//         //         // light should be off
//         //         let modulo = sum % Int::from_i64(2);
//         //         solver.assert(&modulo.eq(Int::from_i64(0)));
//         //     }
//         // }
//         //
//         // // Minimize the total number of button presses
//         // let total_presses = button_presses
//         //     .iter()
//         //     .fold(Int::from_i64(0), |acc, x| acc + x);
//         // solver.minimize(&total_presses);
//         //
//         // // Check if a solution exists
//         // if solver.check(&[]) == z3::SatResult::Sat {
//         //     let model = solver.get_model().unwrap();
//         //     let total = model.eval(&total_presses, true).unwrap().as_i64().unwrap() as i32;
//         //     Some(total as u64)
//         // } else {
//         //     None
//         // }
//         todo!()
//     }
//
//     pub fn fewest_buttons_joltage(&self) -> Option<u64> {
//         let mut vars = variables!();
//         let button_presses: Vec<_> = (0..self.buttons.len())
//             .map(|_| vars.add(variable().min(0).integer()))
//             .collect();
//
//         let mut problem = highs(vars.minimise(button_presses.iter().sum::<Expression>()));
//         let mut jolts = vec![0.into_expression(); self.joltages.len()];
//         for b in 0..self.buttons.len() {
//             for light in self.buttons[b].lights {
//                 // sum each button that affected this counter
//                 jolts[light] += button_presses[b];
//             }
//         }
//
//         for (presses, target_joltage) in jolts.iter().zip(self.joltages) {
//             problem.add_constraint(presses == target_joltage);
//         }
//
//         let solution = problem.solve().ok()?;
//
//         let mut sum = 0;
//         for presses in solution {
//             println!("{}", presses);
//             sum += presses;
//         }
//     }
// }
//
// #[derive(Clone, PartialEq, Eq)]
// pub struct Button {
//     lights: Vec<usize>,
// }
//
// impl Button {
//     pub fn new(lights: Vec<usize>) -> Self {
//         Self { lights }
//     }
// }
