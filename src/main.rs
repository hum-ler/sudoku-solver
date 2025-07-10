use std::path::PathBuf;

use anyhow::{Result, anyhow};
use clap::Parser;

use sudoku_solver::prelude::*;

#[derive(Parser)]
struct Args {
    /// The input (puzzle) file to read from, omit to read from stdin.
    input_file: Option<PathBuf>,

    /// Do not draw border to format the solution.
    #[arg(short = 'n', long = "no-border")]
    plain_output: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let puzzle = read_to_puzzle(args.input_file)?;
    let solution = solve(puzzle).ok_or(anyhow!("No solution."))?;

    if args.plain_output {
        print_solution(solution);
    } else {
        print_solution_with_border(solution);
    }

    Ok(())
}
