use std::rc::Rc;

use crate::algorithms::a_star;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct State {
    combination: Rc<[u32]>
}

pub struct Problem {
    start_state: State,
}

//     let boxed_array: Box<[usize; 999]> = Box::new([0; 999]);
//     let boxed_slice: Box<[usize]> = vec![0; 999].into_boxed_slice();

impl Problem {

    pub fn create(start_combination: Vec<u32>) -> Self {
        let start_state = State {
            combination: Rc::from(start_combination.into_boxed_slice())
        };
        Self {
            start_state
        }
    }

    pub fn children(&self, _state: &State) -> Vec<State> {
        vec![]
    }
}

impl a_star::State for State {}
impl a_star::Problem<State> for Problem {
    fn children(&self, state: &State) -> Vec<State> {
        self.children(state)
    }

    fn is_goal(&self, state: &State) -> bool {
        state.combination.is_sorted()
    }

    fn cost(&self, _state: &State) -> u32 {
        1
    }

    fn heuristic(&self, _state: &State) -> u32 {
        0
    }

    fn start_state(&self) -> State {
        self.start_state.clone()
    }
}
