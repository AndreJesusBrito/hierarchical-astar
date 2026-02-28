use std::fmt::Display;
use crate::algorithms::{a_star, hierarchical_a_star};
use std::rc::Rc;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct State(pub u32, pub u32);

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
pub enum InvalidProblem {
    InvalidLength(u32),
}

pub struct Problem {
    grid_len: u32,
    start_state: State,
    goal_state: State,
}

impl Problem {
    pub fn create(len_power: u32, start_state: State, goal_state: State) -> Result<Self, InvalidProblem> {
        if len_power <= 2 {
            return Err(InvalidProblem::InvalidLength(len_power))
        }
        // TODO: check if it doesn't overflow

        Ok(Self {
            grid_len: u32::pow(2, len_power),
            start_state,
            goal_state,
        })
    }

    pub fn create_basic(len_power: u32) -> Result<Self, InvalidProblem> {
        let start_state = State(1, 1);
        let goal_state = State(len_power, len_power);
        Self::create(len_power, start_state, goal_state)
    }

    pub fn children(&self, state: &State) -> Vec<Rc<State>> {
        let mut result = Vec::new();
        let mut add_child = |x, y| {
            result.push(Rc::new(
                State (x, y)
            ));
        };

        if state.0 > 1 {
            add_child(state.0 - 1, state.1);

            if state.1 > 1 {
                add_child(state.0 - 1, state.1 - 1);
            }
            if state.1 < self.grid_len {
                add_child(state.0 - 1, state.1 + 1);
            }
        }
        if state.0 < self.grid_len {
            add_child(state.0 + 1, state.1);

            if state.1 > 1 {
                add_child(state.0 + 1, state.1 - 1);
            }
            if state.1 < self.grid_len {
                add_child(state.0 + 1, state.1 + 1);
            }
        }
        if state.1 > 1 {
            add_child(state.0, state.1 - 1);
        }
        if state.1 < self.grid_len {
            add_child(state.0, state.1 + 1);
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
        self.goal_state == *state
    }

    fn cost(&self, _state: &State) -> u32 {
        1
    }

    fn heuristic(&self, state: &State) -> u32 {
        (self.goal_state.0).abs_diff(state.0) + (self.goal_state.1).abs_diff(state.1)
    }

    fn start_state(&self) -> State {
        self.start_state
    }
}



pub struct HAProblem {
    max_abstraction_level: u32,
    problem: Problem,
    abstract_goals: Vec<AbstractState>
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct AbstractState {
    level: u32,
    coords: (u32, u32)
}
impl a_star::State for AbstractState {}

fn further_abstract(abstract_state: &AbstractState, max_abstraction_level: u32) -> Option<AbstractState> {
    let next_level = abstract_state.level + 1;
    dbg!(next_level);
    if next_level <= max_abstraction_level {
        Some(
            AbstractState {
                level: next_level,
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

fn abstract_original(state: &State) -> AbstractState {
    AbstractState {
        level: 1,
        coords: (
            state.0 / 2,
            state.1 / 2,
        )
    }
}

fn compute_abstract_goals(problem: &Problem, max_abstraction_level: u32) -> Vec<AbstractState> {
    let mut result = vec![];

    let mut current_abstract_goal = abstract_original(&problem.goal_state);
    result.push(current_abstract_goal);

    while let Some(next_abstract_goal) = further_abstract(&current_abstract_goal, max_abstraction_level) {
        result.push(next_abstract_goal);
        current_abstract_goal = next_abstract_goal;
    }
    result
}

impl HAProblem {
    pub fn create(len_power: u32, start_state: State, goal_state: State) -> Result<Self, InvalidProblem> {
        let problem = Problem::create(len_power, start_state, goal_state)?;
        Self::create_from_problem(problem)
    }

    pub fn create_basic(len_power: u32) -> Result<Self, InvalidProblem> {
        let problem = Problem::create_basic(len_power)?;
        Self::create_from_problem(problem)
    }

    pub fn create_from_problem(problem: Problem) -> Result<Self, InvalidProblem> {
        let max_abstraction_level = (problem.grid_len).ilog2();
        let abstract_goals = compute_abstract_goals(&problem, max_abstraction_level);

        Ok(HAProblem {
            problem,
            max_abstraction_level,
            abstract_goals,
        })
    }
}

impl hierarchical_a_star::HAProblem<State, AbstractState> for HAProblem {
    fn abstract_original(&self, state: &State) -> AbstractState {
        abstract_original(state)
    }

    fn further_abstract(&self, state: &AbstractState) -> Option<AbstractState> {
        further_abstract(state, self.max_abstraction_level)
    }

    fn abstract_children(&self, abstract_state: &AbstractState) -> Vec<Rc<AbstractState>> {
        let max = (self.problem.grid_len).ilog2();
        let mut result = Vec::new();
        let mut add_child = |x, y| {
            let new_state = AbstractState {
                level: abstract_state.level,
                coords: ( x, y ),
            };

            // TODO: make state independent
            result.push(Rc::new(new_state));
        };

        if abstract_state.coords.0 > 1 {
            add_child(abstract_state.coords.0 - 1, abstract_state.coords.1);

            if abstract_state.coords.1 > 1 {
                add_child(abstract_state.coords.0 - 1, abstract_state.coords.1 - 1);
            }
            if abstract_state.coords.1 < max {
                add_child(abstract_state.coords.0 - 1, abstract_state.coords.1 + 1);
            }
        }
        if abstract_state.coords.0 < max {
            add_child(abstract_state.coords.0 + 1, abstract_state.coords.1);

            if abstract_state.coords.1 > 1 {
                add_child(abstract_state.coords.0 + 1, abstract_state.coords.1 - 1);
            }
            if abstract_state.coords.1 < max {
                add_child(abstract_state.coords.0 + 1, abstract_state.coords.1 + 1);
            }
        }
        if abstract_state.coords.1 > 1 {
            add_child(abstract_state.coords.0, abstract_state.coords.1 - 1);
        }
        if abstract_state.coords.1 < max {
            add_child(abstract_state.coords.0, abstract_state.coords.1 + 1);
        }

        result
    }

    fn abstract_cost(&self, abstract_state: &AbstractState) -> u32 {
        1
    }

    fn is_abstract_goal(&self, abstract_state: &AbstractState) -> bool {
        // TODO: check if can be casted with 'as'
        if let Some(abstract_goal) = self.abstract_goals.get(abstract_state.level as usize - 1) {
            abstract_goal == abstract_state
        } else {
            false
        }
    }

    fn original_problem(&self) -> &impl a_star::Problem<State> {
        &self.problem
    }
}
