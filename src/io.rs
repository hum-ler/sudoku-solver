use std::{
    fs::File,
    io::{self, BufReader, Read},
    path::Path,
};

use anyhow::{Result, anyhow};

use crate::sudoku::{Puzzle, Solution};

/// Reads an input into a puzzle. If input file not provided, reads from stdin instead.
///
/// The content of the file can either be exactly one of:
/// (i)  a 9x9 char grid, with digits 1-9 in the appropriate positions.
/// (ii) a 13x13 char grid, which is the same the 9x9 grid, but with an additional 1-char border
///      around each 3x3 digit square.
/// Non-digit chars, as well as the digit 0, will be regarded as blanks or part of the grid border.
///
/// Examples of accepted input:
///
/// (a)
/// ```text
/// ...26.7.1
/// 68..7..9.
/// 19...45..
/// 82.1...4.
/// ..46.29..
/// .5...3.28
/// ..93...74
/// .4..5..36
/// 7.3.18...
/// ```
///
/// (b)
/// ```text
/// +---+---+---+
/// |   |26 |7 1|
/// |68 | 7 | 9 |
/// |19 |  4|5  |
/// +---+---+---+
/// |82 |1  | 4 |
/// |  4|6 2|9  |
/// | 5 |  3| 28|
/// +---+---+---+
/// |  9|3  | 74|
/// | 4 | 5 | 36|
/// |7 3| 18|   |
/// +---+---+---+
/// ```
pub fn read_to_puzzle<P: AsRef<Path>>(input_file: Option<P>) -> Result<Puzzle> {
    let mut buffer = String::new();
    let mut reader: Box<dyn Read> = if let Some(input_file) = input_file {
        Box::new(File::open(input_file)?)
    } else {
        Box::new(BufReader::new(io::stdin().lock()))
    };
    reader.read_to_string(&mut buffer)?;

    let mut lines = buffer
        .lines()
        .filter(|line| !line.is_empty())
        .map(String::from)
        .collect::<Vec<_>>();
    if lines.len() != 9 && lines.len() != 13 {
        return Err(anyhow!("Invalid input: incorrect number of rows."));
    }

    if lines.len() == 13 {
        if !lines.iter().all(|line| line.chars().count() == 13) {
            return Err(anyhow!("Invalid input: incorrect row len."));
        }

        // Extract the embedded digits from the grid.

        let extract_digits = |line: String| -> String {
            line.chars()
                .enumerate()
                .filter_map(|(index, c)| {
                    if index == 0 || index == 4 || index == 8 || index == 12 {
                        None
                    } else {
                        Some(c)
                    }
                })
                .collect()
        };

        lines = lines
            .into_iter()
            .enumerate()
            .filter_map(|(index, line)| {
                if index == 0 || index == 4 || index == 8 || index == 12 {
                    None
                } else {
                    Some(extract_digits(line))
                }
            })
            .collect::<Vec<_>>();
    }

    let mut puzzle = [[0; 9]; 9];
    for (row, line) in lines.into_iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c.is_ascii_digit()
                && let Some(digit) = c.to_digit(10)
                && digit != 0
            {
                puzzle[row][col] = digit as u8;
            }
        }
    }

    Ok(puzzle)
}

/// Prints the solution to stdout as a 9x9 grid (digits only).
pub fn print_solution(solution: Solution) {
    println!("{}", solution_to_string(solution))
}

/// Prints the solution to stdout as a 13x13 grid (digits + border).
pub fn print_solution_with_border(solution: Solution) {
    println!("{}", solution_to_border_string(solution))
}

/// Converts a solution to a String for printing.
fn solution_to_string(solution: Solution) -> String {
    solution
        .map(|row| String::from_utf8_lossy(&row.map(|byte| byte + b'0')).to_string())
        .join("\n")
}

/// Converts a solution to a String for printing.
fn solution_to_border_string(solution: Solution) -> String {
    format!(
        "╔═══╤═══╤═══╗\n{}╟───┼───┼───╢\n{}╟───┼───┼───╢\n{}╚═══╧═══╧═══╝",
        &solution[..3]
            .iter()
            .map(|row| row_to_border_string(*row))
            .collect::<String>(),
        &solution[3..6]
            .iter()
            .map(|row| row_to_border_string(*row))
            .collect::<String>(),
        &solution[6..]
            .iter()
            .map(|row| row_to_border_string(*row))
            .collect::<String>(),
    )
}

fn row_to_border_string(row: [u8; 9]) -> String {
    format!(
        "║{}{}{}│{}{}{}│{}{}{}║\n",
        row[0], row[1], row[2], row[3], row[4], row[5], row[6], row[7], row[8],
    )
}
