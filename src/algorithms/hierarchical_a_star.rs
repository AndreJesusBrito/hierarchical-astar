use std::collections::HashMap;
use std::rc::Rc;
use crate::algorithms::a_star::{self, Problem};

use super::a_star::{
    State,
    Node,
};


pub struct AStarSetsStackNode<S: a_star::State> {
    sets: a_star::AStarSets<S>,
    next: Box<Option<AStarSetsStackNode<S>>>,
}

impl<S: State> AStarSetsStackNode<S> {

    pub fn new_level(start_state: S, estimated_total_cost: u32) -> Self {
        let first_level_sets = a_star::AStarSets::init(start_state, estimated_total_cost);
        AStarSetsStackNode {
            sets: first_level_sets ,
            next: Box::new(Option::None),
        }
    }

    pub fn current_level_sets(&mut self) -> &mut a_star::AStarSets<S> {
        &mut self.sets
    }

    pub fn has_next_level(&self) -> bool {
        self.next.is_some()
    }

    pub fn take_next_level(&mut self) -> Option<AStarSetsStackNode<S>> {
        self.next.take()
    }

    pub fn return_next_level(&mut self, next_level: Self) {
        self.next.replace(next_level);
    }
}



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
    abstract_sets: AStarSetsStackNode<SS>,
    cache: HashMap<SS, u32>,
}

impl<'a, S: State, SS: State, P: HAProblem<S, SS>> HierarchicalAStar<'a, S, SS, P> {

    pub fn new(problem: &'a P) -> Self {
        let mut cache: HashMap<SS, u32> = HashMap::new();

        let start_state = problem.original_problem().start_state();
        // HACK: 0 because is the first node
        let estimated_total_cost = 0;
        let start_abstraction = problem.abstract_original(&start_state);
        let mut abstract_sets = AStarSetsStackNode::new_level(start_abstraction, estimated_total_cost);

        let start_abstraction = problem.abstract_original(&start_state);
        let estimated_total_cost = retrieve_hierarchical_heuristic(
            start_abstraction,
            &mut abstract_sets,
            |ss| problem.further_abstract(ss),
            |ss| problem.is_abstract_goal(ss),
            |ss| problem.abstract_cost(ss),
            |ss| problem.abstract_children(ss),
            &mut cache,
        );

        let sets = a_star::AStarSets::init(start_state, estimated_total_cost);
        Self {
            problem,
            sets,
            abstract_sets: abstract_sets,
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
                let start_abstraction = self.problem.abstract_original(s);
                retrieve_hierarchical_heuristic(
                    start_abstraction,
                    &mut self.abstract_sets,
                    |ss| self.problem.further_abstract(ss),
                    |ss| self.problem.is_abstract_goal(ss),
                    |ss| self.problem.abstract_cost(ss),
                    |ss| self.problem.abstract_children(ss),
                    &mut self.cache,
                )
            }
        )
    }

    pub fn total_nodes_opened(&self) -> usize {
        self.sets.open.len()
    }

    pub fn total_nodes_closed(&self) -> usize {
        self.sets.closed.len()
    }

}

fn retrieve_hierarchical_heuristic<SS, SA, AG, AC, ACD>(
    abstract_state: SS,
    abstract_sets: &mut AStarSetsStackNode<SS>,
    state_abstraction: SA,
    is_abstract_goal: AG,
    abstract_cost: AC,
    abstract_children: ACD,
    cache: &mut HashMap<SS, u32>
) -> u32
where
    SS: State,
    SA: Fn(&SS) -> Option<SS>,
    AG: Fn(&SS) -> bool,
    AC: Fn(&SS) -> u32,
    ACD: Fn(&SS) -> Vec<Rc<SS>>,
{
    let Some(cached_cost) = cache.get(&abstract_state).copied() else {
        let abstract_result = compute_abstract_heuristic(
            &abstract_state,
            abstract_sets,
            state_abstraction,
            is_abstract_goal,
            abstract_cost,
            abstract_children,
            cache
        );

        // TODO: check if it should replace previous cache values
        let previous_cache = cache.get(&abstract_state);
        if previous_cache.is_none() {
            cache.insert(abstract_state, abstract_result);
        }

        return abstract_result
    };

    return cached_cost
}

fn cache_path_states<SS: a_star::State>(cache: &mut HashMap<Rc<SS>, u32>, goal_node: Rc<Node<SS>>) {
    let path_cost = goal_node.cost_from_root;
    let mut current_node = &goal_node;
    while let Some(parent_node) = &current_node.parent {
        let parent_cost_to_goal = path_cost - parent_node.cost_from_root;
        cache.insert(Rc::clone(&parent_node.state), parent_cost_to_goal);
        current_node = parent_node;
    }
}

fn compute_abstract_heuristic<SS, SA, AG, AC, ACD>(
    abstract_state: &SS,
    abstract_sets: &mut AStarSetsStackNode<SS>,
    further_abstract: SA,
    is_abstract_goal: AG,
    abstract_cost: AC,
    abstract_children: ACD,
    mut cache: &mut HashMap<SS, u32>,
) -> u32
where
    SS: State,
    SA: Fn(&SS) -> Option<SS>,
    AG: Fn(&SS) -> bool,
    AC: Fn(&SS) -> u32,
    ACD: Fn(&SS) -> Vec<Rc<SS>>,
{
    let next_level_start = further_abstract(&abstract_state);

    // take or create the next abstraction sets, next_level, to be used in the heuristic
    let mut next_level = match next_level_start {
        Some(start) => {
            let estimated_total_cost = 0;
            Some(
                abstract_sets.take_next_level().unwrap_or_else(|| {
                    AStarSetsStackNode::new_level(start, estimated_total_cost)
                })
            )
        },
        None => None, // if there are no more levels then there are no sets to create or take
    };
    let current_sets = abstract_sets.current_level_sets();

    let abstract_path = a_star::next_path(
        current_sets,
        |ss| is_abstract_goal(ss),
        |ss| abstract_cost(ss),
        |ss| abstract_children(ss),
        |ss| {
            match further_abstract(ss) {
                Some(sss) => {
                    if next_level.is_none() {
                        // no heuristic information
                        return 0;
                    }
                    return retrieve_hierarchical_heuristic(
                        sss,
                        next_level.as_mut().unwrap(),
                        |ss| further_abstract(ss),
                        |ss| is_abstract_goal(ss),
                        |ss| abstract_cost(ss),
                        |ss| abstract_children(ss),
                        &mut cache,
                    )
                }
                None => 0
            }
        },

    );

    // add/return the next_levels ownership to the current_level
    if let Some(next_level_sets) = next_level {
        abstract_sets.return_next_level(next_level_sets);
    }

    match abstract_path {
        Some(node) => node.cost_from_root,
        None => u32::max_value(),
    }
}
