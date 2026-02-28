use std::{cmp, fmt::Display};
use crate::algorithms::a_star;
use std::rc::Rc;

#[derive(Copy, Clone,PartialEq, Eq, Hash, Debug)]
pub struct State {
    pub missionaries_at_left: i32,
    pub cannibals_at_left: i32,
    pub boat_at_left: bool,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.missionaries_at_left, self.cannibals_at_left, self.boat_at_left)
    }
}

pub struct Problem {
    pub total_missionaries: i32,
    pub total_cannibals: i32,
    pub boat_capacity: i32,
    pub start_state: State,
}

impl Problem {
    pub fn create(total_missionaries: i32, total_cannibals: i32, boat_capacity: i32) -> Result<Problem, InvalidProblem> {
        if total_cannibals > total_missionaries {
            return Err(
                InvalidProblem::MoreCannibalsThanMissionaries
            )
        }

        let start_state = State {
            missionaries_at_left: total_missionaries,
            cannibals_at_left: total_cannibals,
            boat_at_left: true,
        };

        Ok(Problem {
            start_state,
            total_missionaries,
            total_cannibals,
            boat_capacity,
        })
    }

    fn children(&self, state: &State) -> Vec<Rc<State>> {
        let mut result = Vec::new();

        let mut apply_action = |action: (i32, i32)| {
            let new_state_option = self.apply_action(state, action);
            match new_state_option {
                Some(new_state) => {
                    // TODO: make states unique
                    result.push(Rc::new(new_state))
                }
                None => ()
            };
        };

        let missionaries = self.missionaries_at_current_side(state);
        let cannibals = self.cannibals_at_current_side(state);

        let available_missionaries = cmp::min(missionaries, self.boat_capacity);
        let available_cannibals = cmp::min(cannibals, self.boat_capacity);

        // actions when only missionaries or cannibals go in a the boat
        for i in 1..=available_missionaries {
            let action = (i, 0);
            apply_action(action);
        }
        for i in 1..=available_cannibals {
            let action = (0, i);
            apply_action(action);
        }

        // the other combinations where both missionarie(s) and cannibal(s) go together in the boat.
        for i in 1..available_cannibals {
            // note: start in i because the number of cannibals cannot exceed
            // the missionaries in the boat
            for j in i..available_missionaries {
                if i + j > self.boat_capacity {
                    break;
                }
                let action = (j, i);
                apply_action(action);
            }
        }

        result
    }

    pub fn missionaries_at_right(&self, state: &State) -> i32 {
        return self.total_missionaries - state.missionaries_at_left;
    }

    pub fn cannibals_at_right(&self, state: &State) -> i32 {
        return self.total_cannibals - state.cannibals_at_left;
    }

    pub fn missionaries_at_current_side(&self, state: &State) -> i32 {
        if state.boat_at_left {
            state.missionaries_at_left
        }
        else {
            self.total_missionaries - state.missionaries_at_left
        }
    }

    pub fn cannibals_at_current_side(&self, state: &State) -> i32 {
        if state.boat_at_left {
            state.cannibals_at_left
        }
        else {
            self.total_cannibals - state.cannibals_at_left
        }
    }

    fn apply_action(&self, parent: &State, action: (i32, i32)) -> Option<State> {
        let direction = if parent.boat_at_left { -1 } else { 1 };

        let new_missionaries = parent.missionaries_at_left + action.0 * direction;
        if new_missionaries < 0 || new_missionaries > self.total_missionaries {
            return None
        }

        let new_cannibals = parent.cannibals_at_left + action.1 * direction;
        if new_cannibals < 0 || new_cannibals > self.total_cannibals {
            return None
        }

        let child = State {
            missionaries_at_left: new_missionaries,
            cannibals_at_left: new_cannibals,
            boat_at_left: ! parent.boat_at_left,
        };


        if self.is_state_valid(&child) {
            return Some(child)
        }

        None
    }

    pub fn is_state_valid(&self, state: &State) -> bool {
        let missionaries_at_right = self.missionaries_at_right(state);
        let cannibals_at_right = self.cannibals_at_right(state);

        // the left side has more cannibals
        if state.missionaries_at_left > 0 && state.cannibals_at_left > state.missionaries_at_left {
            return false
        }

        // the right side has more cannibals
        if missionaries_at_right > 0 && cannibals_at_right > missionaries_at_right {
            return false
        }

        return true
    }
}


#[derive(Debug)]
pub enum InvalidProblem {
    // the number of total cannibals cannot be greater than the total number
    // of missionaries since that would yield an invalid first state.
    MoreCannibalsThanMissionaries,
}

impl a_star::State for State {}
impl a_star::Problem<State> for Problem {
    fn children(&self, state: &State) -> Vec<Rc<State>> {
        self.children(state)
    }

    fn is_goal(&self, state: &State) -> bool {
        return state.missionaries_at_left == 0
            && state.cannibals_at_left == 0
            && state.boat_at_left == false
    }

    fn cost(&self, _state: &State) -> u32 {
        1
    }

    fn heuristic(&self, state: &State) -> u32 {
        i32::unsigned_abs((state.missionaries_at_left + state.cannibals_at_left) / self.boat_capacity)
    }

    fn start_state(&self) -> State {
        return self.start_state
    }
}
