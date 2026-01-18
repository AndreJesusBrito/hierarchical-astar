use std::fmt::Display;
use crate::algorithms::{a_star, hierarchical_a_star};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct State(pub i32, pub i32);

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct AbstractState {
    level: u32,
    coords: (i32, i32)
}
impl a_star::State for AbstractState {}


#[derive(Debug)]
pub enum InvalidProblem {
    InvalidLength(i32),
}

pub struct Problem {
    quadrant_len: i32,
    start_state: State,
}

impl Problem {
    pub fn create(quadrant_len: i32) -> Result<Self, InvalidProblem> {

        if quadrant_len <= 0 {
            return Err(InvalidProblem::InvalidLength(quadrant_len))
        }

        Ok(Problem {
            quadrant_len,
            start_state: State (0, 0),
        })
    }

    fn is_valid_state(&self, state: &State) -> bool {
        return state.0 <= self.quadrant_len
            && state.0 >= -self.quadrant_len
            && state.1 <= self.quadrant_len
            && state.1 >= -self.quadrant_len
    }

    pub fn children(&self, state: &State) -> Vec<State> {
        let steps = [
            ( 0,  1),
            ( 1,  1),
            ( 1,  0),
            ( 1, -1),
            ( 0, -1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
        ];

        let mut result = Vec::new();

        for step in steps {
            let new_state = State (
                state.0 + step.0,
                state.1 + step.1,
            );
            if self.is_valid_state(&new_state) {
                result.push(new_state);
            }
        }

        result
    }
}

impl a_star::State for State {}
impl a_star::Problem<State> for Problem {
    fn children(&self, state: &State) -> Vec<State> {
        self.children(state)
    }

    fn is_goal(&self, state: &State) -> bool {
        return state.0 == self.quadrant_len
            && state.1 == self.quadrant_len
    }

    fn cost(&self, _state: &State) -> u32 {
        1
    }

    fn heuristic(&self, state: &State) -> u32 {
        i32::unsigned_abs(self.quadrant_len - state.0) + i32::unsigned_abs(self.quadrant_len - state.1)
    }

    fn start_state(&self) -> State {
        State (0, 0)
    }
}

struct HierarchicalAbstraction;
impl hierarchical_a_star::HierarchicalAbstraction for HierarchicalAbstraction {
    fn abstract_original(&self, state: &State) -> AbstractState {
        state.0
    }

    fn further_abstract(&self, abstract_state: &AbstractState) -> Option<AbstractState> {
        todo!()
    }

    fn children(&self, abstract_state: &AbstractState, problem: &a_star::Problem) -> Vec<AbstractState> {
        let steps = [
            ( 0,  1),
            ( 1,  1),
            ( 1,  0),
            ( 1, -1),
            ( 0, -1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
        ];

        let mut result = Vec::new();

        for step in steps {
            let new_state = AbstractState {
                level: abstract_state.level,
                coords: (
                    abstract_state.coords.0 + step.0,
                    abstract_state.coords.1 + step.1,
                ),
            };

            let x = new_state.coords.0 * new_state.

            let valid_x = ;
            let valid_y = false;
            if valid_x && valid_y {
                result.push(new_state);
            }
        }

        return result
    }

    fn cost(&self, abstract_state: &AbstractState) -> u32 {
        1
    }

    fn is_goal(&self, abstract_state: &AbstractState) -> bool {
        todo!()
    }
}
