use std::collections::HashMap;
use std::rc::Rc;
use crate::algorithms::a_star::{self, Problem};

use super::a_star::{
    State,
    Node,
};


pub trait HAProblem<S: State, SS: State> {
    fn original_problem(&self) -> &impl a_star::Problem<S>;
    fn abstract_original(&self, state: &S) -> SS;
    fn further_abstract(&self, state: &SS) -> Option<SS>;
    fn abstract_children(&self, state: &SS) -> Vec<Rc<SS>>;
    fn abstract_cost(&self, state: &SS) -> u32;
    fn is_abstract_goal(&self, state: &SS) -> bool;
}


pub struct HierarchicalAStar<'a, S: State, SS: State, P: HAProblem<S, SS>> {
    problem: &'a P,
    sets: a_star::AStarSets<S>,
    cache: HashMap<Rc<SS>, u32>,
}

impl<'a, S: State, SS: State, P: HAProblem<S, SS>> HierarchicalAStar<'a, S, SS, P> {
    pub fn new(problem: &'a P) -> Self {
        let mut cache = HashMap::new();

        let start_state = problem.original_problem().start_state();

        let start_abstraction = problem.abstract_original(&start_state);
        let estimated_total_cost = retrieve_hierarchical_heuristic(
            start_abstraction,
            & |ss| problem.further_abstract(ss),
            & |ss| problem.is_abstract_goal(ss),
            & |ss| problem.abstract_cost(ss),
            & |ss| problem.abstract_children(ss),
            &mut cache,
        );

        let mut sets = a_star::AStarSets::new();
        sets.create_and_add_first_node(start_state, estimated_total_cost);
        Self {
            problem,
            sets,
            cache,
        }
    }

    pub fn next_path(&mut self) -> Option<Rc<Node<S>>> {
        a_star::next_path(
            &mut self.sets,
            |s| self.problem.original_problem().is_goal(s),
            |s| self.problem.original_problem().cost(s),
            |s| self.problem.original_problem().children(s),
            |s| {
                let state_abstraction = self.problem.abstract_original(s);
                retrieve_hierarchical_heuristic(
                    state_abstraction,
                    & |ss| self.problem.further_abstract(ss),
                    & |ss| self.problem.is_abstract_goal(ss),
                    & |ss| self.problem.abstract_cost(ss),
                    & |ss| self.problem.abstract_children(ss),
                    &mut self.cache,
                )
            }
        )
    }

    pub fn total_nodes_opened(&self) -> usize {
        self.total_original_nodes_opened() + self.total_abstract_nodes_opened()
    }

    pub fn total_nodes_closed(&self) -> usize {
        self.total_original_nodes_closed() + self.total_abstract_nodes_closed()
    }

    pub fn total_original_nodes_opened(&self) -> usize {
        self.sets.open.len()
    }

    pub fn total_original_nodes_closed(&self) -> usize {
        self.sets.closed.len()
    }

    pub fn total_abstract_nodes_opened(&self) -> usize {
        todo!()
    }

    pub fn total_abstract_nodes_closed(&self) -> usize {
        todo!()
    }
}

fn retrieve_hierarchical_heuristic<SS, SA, AG, AC, ACD>(
    start_state: SS,
    further_abstract: &SA,
    is_abstract_goal: &AG,
    abstract_cost: &AC,
    abstract_children: &ACD,
    mut cache: &mut HashMap<Rc<SS>, u32>
) -> u32
where
    SS: State,
    SA: Fn(&SS) -> Option<SS>,
    AG: Fn(&SS) -> bool,
    AC: Fn(&SS) -> u32,
    ACD: Fn(&SS) -> Vec<Rc<SS>>,
{
    if let Some(cached_cost) = cache.get(&start_state).copied() {
        return cached_cost
    }

    let mut current_sets = a_star::AStarSets::new();
    let start_state_heuristic = if let Some(start_state_abstration) = further_abstract(&start_state) {
        match cache.get(&start_state_abstration).copied() {
            Some(h) => h,
            None => {
                retrieve_hierarchical_heuristic(
                    start_state_abstration,
                    further_abstract,
                    is_abstract_goal,
                    abstract_cost,
                    abstract_children,
                    &mut cache,
                )
            }
        }
    } else { 0 };
    current_sets.create_and_add_first_node(start_state, start_state_heuristic);

    let path = a_star::next_path(
        &mut current_sets,
        |s| is_abstract_goal(s),
        |s| abstract_cost(s),
        |s| abstract_children(s),
        |s| {
            match further_abstract(s) {
                Some(abstract_state) => retrieve_hierarchical_heuristic(
                    abstract_state,
                    further_abstract,
                    is_abstract_goal,
                    abstract_cost,
                    abstract_children,
                    &mut cache,
                ),
                None => 0
            }
        },
    );

    match path {
        Some(goal_node) => cache_path_states(cache, goal_node),
        None => u32::max_value(),
    }
}

fn cache_path_states<SS: a_star::State>(cache: &mut HashMap<Rc<SS>, u32>, goal_node: Rc<Node<SS>>) -> u32 {
    let path_cost = goal_node.cost_from_root;
    let mut current_node = &goal_node;
    while let Some(parent_node) = &current_node.parent {
        let parent_cost_to_goal = path_cost - parent_node.cost_from_root;
        cache.insert(Rc::clone(&parent_node.state), parent_cost_to_goal);
        current_node = parent_node;
    }
    return path_cost;
}
