mod problems;
mod algorithms;

use problems::missionaries_and_cannibals;
use problems::grid_square;
use algorithms::a_star;

use crate::algorithms::a_star::Problem;

fn run_a_star_mc60_40_7() {
    let mc60_40_7 = missionaries_and_cannibals::Problem::create(60, 40, 7).unwrap();

    let mut a_star = a_star::AStar::new(&mc60_40_7);

    let path_option = a_star.next_path();

    if let Some(path) = path_option {
        let mut node_option = &Some(path);
        while let Some(node) = node_option {
            println!("{}", node.state);
            node_option = &node.parent;
        }

        println!("Total nodes opened = {}, closed = {}", a_star.total_nodes_opened(), a_star.total_nodes_closed());
    } else {
        println!("no solution found!");
    }

}

fn run_a_star_grid_square() {
    let square_grid = grid_square::Problem::create(124).unwrap();

    let mut a_star = a_star::AStar::new(&square_grid);

    let path_option = a_star.next_path();

    if let Some(path) = path_option {
        let mut node_option = &Some(path);
        while let Some(node) = node_option {
            println!("{}", node.state);
            node_option = &node.parent;
        }

        println!("Total nodes opened = {}, closed = {}", a_star.total_nodes_opened(), a_star.total_nodes_closed());
    } else {
        println!("no solution found!");
    }

}

fn main() {
    run_a_star_grid_square();

//    let mut a_star = a_star::AStar::new(&mc60_40_7);

//     let grid = grid_square::Problem::create(20).unwrap();
//
//     let state = grid_square::State (3,3);
//     for child in grid.children(&state) {
//         println!("{}", child);
//     }

    // run_a_star_grid_square();
    // run_a_star_mc60_40_7();

//     let mc60_40_7 = missionaries_and_cannibals::Problem::create(60, 40, 7).unwrap();
//     // let mc60_40_7 = missionaries_and_cannibals::Problem::create(3,3,2).unwrap();
//     let mut open = vec![mc60_40_7.start_state()];
//     let mut closed: Vec<missionaries_and_cannibals::State> = Vec::new();
//     while open.len() > 0 {
//         let next = open.pop().unwrap();
//         if ! closed.contains(&next) {
//             for child in mc60_40_7.children(&next) {
//                 if ! closed.contains(&child) {
//                     open.push(child);
//                 }
//             }
//             closed.push(next);
//         }
//     }
//     println!("total of closed = {}", closed.len());
}
