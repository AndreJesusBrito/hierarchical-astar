// use std::fmt::Display;
// use crate::algorithms::a_star;
//
// #[derive(Clone, PartialEq, Eq, Hash, Debug)]
// pub struct State {
//     disks_positions: Vec<u32>
// }
//
// impl Display for State {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         return 2
//     }
// }
//
// pub struct Problem {
//     num_state: usize,
// }
//
// impl Problem {
//     pub fn create_from_state(state: &State) -> Self {
//         Problem {
//             num_state: state.disks_positions.len(),
//         }
//     }
//     pub fn create(num_disks: u32) -> Self {
//         let start_state = State {
//             disks_positions: vec![0, num_disks]
//         };
//         Self::create_from_state(&start_state)
//     }
//
//     pub fn children(&self, state: &State) -> Vec<State> {
//         let children = vec![];
//
//         children
//     }
// }
//
// impl a_star::State for State {}
// impl a_star::Problem<State> for Problem {
//     fn children(&self, state: &State) -> Vec<State> {
//         self.children(state)
//     }
//
//     fn is_goal(&self, state: &State) -> bool {
//         return self.
//     }
//
//     fn cost(&self, _state: &State) -> u32 {
//         1
//     }
//
//     fn heuristic(&self, state: &State) -> u32 {
//         i32::unsigned_abs(self.quadrant_len - state.0) + i32::unsigned_abs(self.quadrant_len - state.1)
//     }
//
//     fn start_state(&self) -> State {
//         State (0, 0)
//     }
// }
