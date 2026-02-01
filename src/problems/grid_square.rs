use std::fmt::Display;
use crate::algorithms::{a_star, hierarchical_a_star};
use std::rc::Rc;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct State(pub i32, pub i32);

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
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

        if quadrant_len <= 2 {
            return Err(InvalidProblem::InvalidLength(quadrant_len))
        }

        Ok(Problem {
            quadrant_len,
            start_state: State (1, 1),
        })
    }

    fn is_valid_state(&self, state: &State) -> bool {
        return state.0 <= self.quadrant_len
            && state.0 >= 1
            && state.1 <= self.quadrant_len
            && state.1 >= 1
    }

    pub fn children(&self, state: &State) -> Vec<Rc<State>> {
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
                // TODO: make states unique
                result.push(Rc::new(new_state));
            }
        }

        result
    }
}

impl a_star::State for State {}
impl a_star::Problem<State> for Problem {
    fn children(&self, state: &State) -> Vec<Rc::<State>> {
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
        State (1, 1)
    }
}

pub struct HAProblem {
    max_abstraction_level: u32,
    problem: Problem,
}

impl HAProblem {
    pub fn create(quadrant_len: i32) -> Result<Self, InvalidProblem> {
        let problem = Problem::create(quadrant_len)?;
        let max_abstraction_level = i32::ilog2(quadrant_len + 1);
        Ok(HAProblem {
            problem,
            max_abstraction_level
        })
    }
}

impl hierarchical_a_star::HAProblem<State, AbstractState> for HAProblem {
    fn abstract_original(&self, state: &State) -> AbstractState {
        AbstractState {
            level: 1,
            coords: (
                (state.0 + 1) / 2,
                (state.1 + 1) / 2,
            )
        }
    }

    fn further_abstract(&self, abstract_state: &AbstractState) -> Option<AbstractState> {
        if abstract_state.level <= self.max_abstraction_level {
            Some(
                AbstractState {
                    level: abstract_state.level + 1,
                    coords: (
                        abstract_state.coords.0 / 2,
                        abstract_state.coords.1 / 2,
                    )
                }
            )
        } else {
            None
        }
    }

    fn abstract_children(&self, abstract_state: &AbstractState) -> Vec<Rc<AbstractState>> {
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

            let x = (abstract_state.coords.0 + new_state.coords.0) * (2 as i32).pow(abstract_state.level);
            let y = (abstract_state.coords.1 + new_state.coords.1) * (2 as i32).pow(abstract_state.level);

            let valid_x = x >= 1 && x <= self.problem.quadrant_len;
            let valid_y = y >= 1 && y <= self.problem.quadrant_len;
            if valid_x && valid_y {
                // TODO: make state independent
                result.push(Rc::new(new_state));
            }
        }

        return result
    }

    fn abstract_cost(&self, abstract_state: &AbstractState) -> u32 {
        1
    }

    fn is_abstract_goal(&self, abstract_state: &AbstractState) -> bool {
        todo!()
    }

    fn original_problem(&self) -> &impl a_star::Problem<State> {
        &self.problem
    }
}
