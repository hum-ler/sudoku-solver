use std::array;

/// 9x9 Sudoku grid in reading order.
///
/// Use 1-9 to represent a digit, and 0 to represent a blank or unknown.
type Grid = [[u8; 9]; 9];
pub type Puzzle = Grid;
pub type Solution = Grid;

/// Finds a solution to the given puzzle, if any.
pub fn solve(puzzle: Puzzle) -> Option<Solution> {
    if !is_valid_puzzle(puzzle) {
        return None;
    }

    let blanks = blanks(puzzle);
    if blanks.is_empty() {
        return Some(puzzle);
    }

    find_solution(puzzle, 0, &blanks)
}

fn is_valid_puzzle(puzzle: Puzzle) -> bool {
    (0..9).all(|index| {
        slice_has_unique_digits(horizontal_slice(puzzle, index))
            && slice_has_unique_digits(vertical_slice(puzzle, index))
            && slice_has_unique_digits(square_slice(puzzle, index))
    })
}

/// Checks that a slice has all unique digits, except 0, which is ignored.
fn slice_has_unique_digits(slice: [u8; 9]) -> bool {
    let mut unique_digits = [false; 9];

    for digit in slice {
        if digit == 0 {
            continue;
        }

        let index = (digit - 1) as usize;
        if unique_digits[index] {
            return false;
        }

        unique_digits[index] = true;
    }

    true
}

/// (row, col)
type GridPos = (usize, usize);

/// Finds all the blank positions in a [Puzzle] that need to be filled in to form a [Solution].
fn blanks(puzzle: Puzzle) -> Vec<GridPos> {
    puzzle
        .iter()
        .enumerate()
        .flat_map(|(row, digits)| {
            digits.iter().enumerate().filter_map(
                move |(col, digit)| {
                    if *digit == 0 { Some((row, col)) } else { None }
                },
            )
        })
        .collect()
}

/// Gets a view of a row in a [Puzzle].
fn horizontal_slice(puzzle: Puzzle, row: usize) -> [u8; 9] {
    if !(0..9).contains(&row) {
        panic!("Invalid row index: {row}");
    }

    puzzle[row]
}

/// Gets a view of a col in a [Puzzle].
fn vertical_slice(puzzle: Puzzle, col: usize) -> [u8; 9] {
    if !(0..9).contains(&col) {
        panic!("Invalid col index: {col}");
    }

    array::from_fn(|row| puzzle[row][col])
}

/// Gets a view of a square in a [Puzzle].
///
/// Squares are indexed as follows:
///
/// ```text
/// +-+-+-+
/// |0|1|2|
/// +-+-+-+
/// |3|4|5|
/// +-+-+-+
/// |6|7|8|
/// +-+-+-+
/// ```
///
/// Elements in each square are indexed as follows:
/// ```text
/// +---+
/// |012|
/// |345|
/// |678|
/// +---+
/// ```
fn square_slice(puzzle: Puzzle, square: usize) -> [u8; 9] {
    match square {
        0 => [
            puzzle[0][0],
            puzzle[0][1],
            puzzle[0][2],
            puzzle[1][0],
            puzzle[1][1],
            puzzle[1][2],
            puzzle[2][0],
            puzzle[2][1],
            puzzle[2][2],
        ],
        1 => [
            puzzle[0][3],
            puzzle[0][4],
            puzzle[0][5],
            puzzle[1][3],
            puzzle[1][4],
            puzzle[1][5],
            puzzle[2][3],
            puzzle[2][4],
            puzzle[2][5],
        ],
        2 => [
            puzzle[0][6],
            puzzle[0][7],
            puzzle[0][8],
            puzzle[1][6],
            puzzle[1][7],
            puzzle[1][8],
            puzzle[2][6],
            puzzle[2][7],
            puzzle[2][8],
        ],
        3 => [
            puzzle[3][0],
            puzzle[3][1],
            puzzle[3][2],
            puzzle[4][0],
            puzzle[4][1],
            puzzle[4][2],
            puzzle[5][0],
            puzzle[5][1],
            puzzle[5][2],
        ],
        4 => [
            puzzle[3][3],
            puzzle[3][4],
            puzzle[3][5],
            puzzle[4][3],
            puzzle[4][4],
            puzzle[4][5],
            puzzle[5][3],
            puzzle[5][4],
            puzzle[5][5],
        ],
        5 => [
            puzzle[3][6],
            puzzle[3][7],
            puzzle[3][8],
            puzzle[4][6],
            puzzle[4][7],
            puzzle[4][8],
            puzzle[5][6],
            puzzle[5][7],
            puzzle[5][8],
        ],
        6 => [
            puzzle[6][0],
            puzzle[6][1],
            puzzle[6][2],
            puzzle[7][0],
            puzzle[7][1],
            puzzle[7][2],
            puzzle[8][0],
            puzzle[8][1],
            puzzle[8][2],
        ],
        7 => [
            puzzle[6][3],
            puzzle[6][4],
            puzzle[6][5],
            puzzle[7][3],
            puzzle[7][4],
            puzzle[7][5],
            puzzle[8][3],
            puzzle[8][4],
            puzzle[8][5],
        ],
        8 => [
            puzzle[6][6],
            puzzle[6][7],
            puzzle[6][8],
            puzzle[7][6],
            puzzle[7][7],
            puzzle[7][8],
            puzzle[8][6],
            puzzle[8][7],
            puzzle[8][8],
        ],
        _ => panic!("Invalid square index: {square}"),
    }
}

/// Finds a [Solution] to a [Puzzle] by backtracking.
fn find_solution(mut puzzle: Puzzle, blank: usize, blanks: &[GridPos]) -> Option<Solution> {
    if blank == blanks.len() {
        return Some(puzzle);
    }

    let (row, col) = blanks[blank];

    for digit in 1..=9 {
        puzzle[row][col] = digit;

        if !is_valid_puzzle(puzzle) {
            continue;
        }

        if let Some(solution) = find_solution(puzzle, blank + 1, blanks) {
            return Some(solution);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const SLICE_TEST_1: Puzzle = [
        [0, 0, 0, 1, 1, 1, 2, 2, 2],
        [0, 0, 0, 1, 1, 1, 2, 2, 2],
        [0, 0, 0, 1, 1, 1, 2, 2, 2],
        [3, 3, 3, 4, 4, 4, 5, 5, 5],
        [3, 3, 3, 4, 4, 4, 5, 5, 5],
        [3, 3, 3, 4, 4, 4, 5, 5, 5],
        [6, 6, 6, 7, 7, 7, 8, 8, 8],
        [6, 6, 6, 7, 7, 7, 8, 8, 8],
        [6, 6, 6, 7, 7, 7, 8, 8, 8],
    ];
    const SLICE_TEST_2: Puzzle = [
        [1, 2, 3, 1, 2, 3, 1, 2, 3],
        [4, 5, 6, 4, 5, 6, 4, 5, 6],
        [7, 8, 9, 7, 8, 9, 7, 8, 9],
        [1, 2, 3, 1, 2, 3, 1, 2, 3],
        [4, 5, 6, 4, 5, 6, 4, 5, 6],
        [7, 8, 9, 7, 8, 9, 7, 8, 9],
        [1, 2, 3, 1, 2, 3, 1, 2, 3],
        [4, 5, 6, 4, 5, 6, 4, 5, 6],
        [7, 8, 9, 7, 8, 9, 7, 8, 9],
    ];

    #[test]
    fn check_horizontal_slice() {
        assert_eq!(
            horizontal_slice(SLICE_TEST_1, 0),
            [0, 0, 0, 1, 1, 1, 2, 2, 2]
        );
        assert_eq!(
            horizontal_slice(SLICE_TEST_1, 4),
            [3, 3, 3, 4, 4, 4, 5, 5, 5]
        );
        assert_eq!(
            horizontal_slice(SLICE_TEST_1, 8),
            [6, 6, 6, 7, 7, 7, 8, 8, 8]
        );

        assert_eq!(
            horizontal_slice(SLICE_TEST_1, 1),
            horizontal_slice(SLICE_TEST_1, 2)
        );
        assert_eq!(
            horizontal_slice(SLICE_TEST_1, 3),
            horizontal_slice(SLICE_TEST_1, 5)
        );
        assert_eq!(
            horizontal_slice(SLICE_TEST_1, 6),
            horizontal_slice(SLICE_TEST_1, 7)
        );

        for index in [0, 3, 6] {
            assert_eq!(
                horizontal_slice(SLICE_TEST_2, index),
                [1, 2, 3, 1, 2, 3, 1, 2, 3]
            );
        }
        for index in [1, 4, 7] {
            assert_eq!(
                horizontal_slice(SLICE_TEST_2, index),
                [4, 5, 6, 4, 5, 6, 4, 5, 6]
            );
        }
        for index in [2, 5, 8] {
            assert_eq!(
                horizontal_slice(SLICE_TEST_2, index),
                [7, 8, 9, 7, 8, 9, 7, 8, 9]
            );
        }
    }

    #[test]
    #[should_panic]
    fn check_invalid_horizontal_slice() {
        horizontal_slice(SLICE_TEST_1, 9);
    }

    #[test]
    fn check_vertical_slice() {
        assert_eq!(vertical_slice(SLICE_TEST_1, 0), [0, 0, 0, 3, 3, 3, 6, 6, 6]);
        assert_eq!(vertical_slice(SLICE_TEST_1, 4), [1, 1, 1, 4, 4, 4, 7, 7, 7]);
        assert_eq!(vertical_slice(SLICE_TEST_1, 8), [2, 2, 2, 5, 5, 5, 8, 8, 8]);

        assert_eq!(
            vertical_slice(SLICE_TEST_1, 1),
            vertical_slice(SLICE_TEST_1, 2)
        );
        assert_eq!(
            vertical_slice(SLICE_TEST_1, 3),
            vertical_slice(SLICE_TEST_1, 5)
        );
        assert_eq!(
            vertical_slice(SLICE_TEST_1, 6),
            vertical_slice(SLICE_TEST_1, 7)
        );

        for index in [0, 3, 6] {
            assert_eq!(
                vertical_slice(SLICE_TEST_2, index),
                [1, 4, 7, 1, 4, 7, 1, 4, 7]
            );
        }
        for index in [1, 4, 7] {
            assert_eq!(
                vertical_slice(SLICE_TEST_2, index),
                [2, 5, 8, 2, 5, 8, 2, 5, 8]
            );
        }
        for index in [2, 5, 8] {
            assert_eq!(
                vertical_slice(SLICE_TEST_2, index),
                [3, 6, 9, 3, 6, 9, 3, 6, 9]
            );
        }
    }

    #[test]
    #[should_panic]
    fn check_invalid_vertical_slice() {
        vertical_slice(SLICE_TEST_1, 9);
    }

    #[test]
    fn check_square_slice() {
        for index in 0..9 {
            assert!(
                square_slice(SLICE_TEST_1, index)
                    .iter()
                    .all(|digit| *digit == index as u8)
            );
        }

        for index in 0..9 {
            assert_eq!(
                square_slice(SLICE_TEST_2, index),
                [1, 2, 3, 4, 5, 6, 7, 8, 9]
            );
        }
    }

    #[test]
    #[should_panic]
    fn check_invalid_square_slice() {
        square_slice(SLICE_TEST_1, 9);
    }

    #[test]
    fn check_slice_uniqueness() {
        assert!(slice_has_unique_digits([1, 2, 3, 4, 5, 6, 7, 8, 9]));
        assert!(slice_has_unique_digits([9, 8, 7, 6, 5, 4, 3, 2, 1]));

        assert!(slice_has_unique_digits([0, 2, 0, 4, 0, 6, 0, 8, 0]));
        assert!(slice_has_unique_digits([9, 0, 7, 0, 5, 0, 3, 0, 1]));

        assert!(!slice_has_unique_digits([1, 1, 2, 2, 3, 3, 4, 4, 5]));
        assert!(!slice_has_unique_digits([9, 8, 7, 6, 5, 4, 3, 2, 2]));

        assert!(slice_has_unique_digits([0; 9]));
    }
}
