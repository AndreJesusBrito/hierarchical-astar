mod problems;
mod algorithms;

use algorithms::a_star::Problem;
use algorithms::hierarchical_a_star;
use algorithms::hierarchical_a_star::HAProblem;
use problems::missionaries_and_cannibals;
use problems::grid_square;
use algorithms::a_star;

use std::io;


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
    let square_grid = grid_square::Problem::create_basic(8).unwrap();

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

fn run_hierarchical_a_star_grid_square() {
    let square_grid = grid_square::Problem::create_basic(8).unwrap();
    let square_grid_abstract = grid_square::HAProblem::create_from_problem(square_grid).unwrap();

    let original_start = square_grid_abstract.original_problem().start_state();
    dbg!(original_start);
    let mut current_abstration = square_grid_abstract.abstract_original(&original_start);
    dbg!(current_abstration);
    while let Some(abstration) = square_grid_abstract.further_abstract(&current_abstration) {
        current_abstration = abstration;
        dbg!(abstration);
    }


//     let mut hierarchical_a_star = hierarchical_a_star::HierarchicalAStar::new(&square_grid_abstract);
//
//     let path_option = hierarchical_a_star.next_path();
//
//     if let Some(path) = path_option {
//         let mut node_option = &Some(path);
//         while let Some(node) = node_option {
//             println!("{}", node.state);
//             node_option = &node.parent;
//         }
//
//         println!("Total nodes opened = {}, closed = {}", hierarchical_a_star.total_nodes_opened(), hierarchical_a_star.total_nodes_closed());
//     } else {
//         println!("no solution found!");
//     }

}

fn main() {
    run_a_star_grid_square();
    run_hierarchical_a_star_grid_square();

//     loop {
//         let mut guess = String::new();
//
//         io::stdin()
//             .read_line(&mut guess)
//             .expect("Failed to read line");
//
//         let guess: u32 = match guess.trim().parse() {
//             Ok(num) => num,
//             Err(_) => continue,
//         };
//
//         println!("You guessed: {guess}");
//
//     }













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
