use std::hash::Hash;
use std::rc::Rc;
use std::fmt;

pub trait State: Clone + Eq + Hash + fmt::Debug {}

pub trait Problem<S: State> {
    fn children(&self, state: &S) -> Vec<Rc<S>>;
    fn is_goal(&self, state: &S) -> bool;

    fn cost(&self, state: &S) -> u32;
    fn heuristic(&self, state: &S) -> u32;
    fn start_state(&self) -> S;
}

pub struct Node<S: State> {
    pub state: Rc<S>,
    pub parent: Option<Rc<Node<S>>>,
    pub cost_from_root: u32,
    pub estimated_total_cost: u32,
}

impl<S: State> fmt::Debug for Node<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node")
         .field("state", &self.state)
         .field("cost_from_root", &self.cost_from_root)
         .field("estimated_total_cost", &self.estimated_total_cost)
         .finish()
    }
}

impl<S: State> PartialEq for Node<S> {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state
    }
}

pub struct AStarSets<S: State> {
    pub open: Vec<Rc<Node<S>>>,
    pub closed: Vec<Rc<Node<S>>>,
}

impl<S: State> AStarSets<S> {
    pub fn new() -> Self {
        AStarSets {
            open: Vec::new(),
            closed: Vec::new(),
        }
    }

    pub fn create_and_add_first_node(&mut self, start_state: S, estimated_total_cost: u32) {
        let start_node = create_first_node(start_state, estimated_total_cost);
        self.add_open_node(start_node);
    }

    pub fn add_open_node(&mut self, node: Node<S>) {
        self.open.push(Rc::new(node))
    }
}

pub struct AStar<'a, S: State, P: Problem<S>> {
    problem: &'a P,
    sets: AStarSets<S>,
}

impl<'a, S: State, P: Problem<S>> AStar<'a, S, P> {

    pub fn new(problem: &'a P) -> Self {
        let start_state = problem.start_state();
        let estimated_total_cost = problem.heuristic(&start_state);
        let mut sets = AStarSets::new();
        sets.create_and_add_first_node(start_state, estimated_total_cost);

        Self {
            problem,
            sets,
        }
    }

    pub fn next_path(&mut self) -> Option<Rc<Node<S>>> {
        next_path(
            &mut self.sets,
            |s| self.problem.is_goal(s),
            |s| self.problem.cost(s),
            |s| self.problem.children(s),
            |s| self.problem.heuristic(s),
        )
    }

    pub fn total_nodes_opened(&self) -> usize {
        self.sets.open.len()
    }

    pub fn total_nodes_closed(&self) -> usize {
        self.sets.closed.len()
    }

//     fn buildPath(currentNode: Node<S>, path: Node<S>) -> Vec<State> {
//         let parent = currentNode.parent
//         if (parent.some) {
//             this.buildPath(parent.value, path)
//         }
//         path.push(currentNode)
//     }


//     fn create_node_wrapper(child: N, parent: &NodeWrapper<N>) -> NodeWrapper<N> {
//     }

}


// export class AStar<N extends IAStarNode> {
//     context: IAStarContext<N>
//     openSet: IAStarNodeWrapper<N>[] = []
//
//     constructor(context: IAStarContext<N>) {
//         this.context = context
//     }
//
//
//     private createNodeWrapper(node: N, parent: IAStarNodeWrapper<N>): IAStarNodeWrapper<N> {
//         const g = this.context.cost(node, parent.accumulatedCost, parent.content)
//         const h = this.context.heuristic(node, parent.content)
//
//         return {
//             content: node,
//             parent: newOption(parent),
//             totalCost: g + h,
//             accumulatedCost: g,
//         }
//     }
//
//     private ajustPriority() {
//         this.openSet.sort((a, b) => b.totalCost - a.totalCost)
//     }
//
//
// }
//


pub fn create_first_node<S: State>(state: S, cost_estimation: u32) -> Node<S> {
    Node {
        state: Rc::new(state),
        parent: None,
        cost_from_root: 0,
        estimated_total_cost: cost_estimation,
    }
}

pub fn next_path<S, G, C, CD, H>(
    sets: &mut AStarSets<S>,
    is_goal: G,
    cost: C,
    children: CD,
    mut heuristic: H,
) -> Option<Rc<Node<S>>>
where
    S: State,
    G: Fn(&S) -> bool,
    C: Fn(&S) -> u32,
    CD: Fn(&S) -> Vec<Rc<S>>,
    H: FnMut(&S) -> u32
{
    while sets.open.len() > 0 {
        let current_node = sets.open.pop().unwrap();

        if sets.closed.contains(&current_node) {
            continue
        }

        sets.closed.push(Rc::clone(&current_node));
        if is_goal(&current_node.state) {
            return Some(current_node);
        }

        for child in children(&current_node.state) {
            let g = cost(&child);
            let h = heuristic(&child);

            let cost_from_root = g + current_node.cost_from_root;
            let estimated_total_cost = cost_from_root + h;

            let child_node = Rc::new(Node {
                state: Rc::clone(&child),
                parent: Some(Rc::clone(&current_node)),
                cost_from_root,
                estimated_total_cost,
            });

            if ! sets.closed.contains(&child_node) {
                sets.open.push(child_node);
            }
        }

        // ajust open set priority
        sets.open.sort_by(|a, b| b.estimated_total_cost.cmp(&a.estimated_total_cost));
    }

    // no path exists to goal
    None
}
