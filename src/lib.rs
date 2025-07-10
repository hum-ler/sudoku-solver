mod io;
mod sudoku;

pub mod prelude {
    pub use super::{
        io::{print_solution, print_solution_with_border, read_to_puzzle},
        sudoku::{Puzzle, Solution, solve},
    };
}
