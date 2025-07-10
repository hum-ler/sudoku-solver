# sudoku-solver

Solves a Sudoku puzzle and prints the solution to stdout.

## Usage

```
Usage: sudoku-solver [OPTIONS] [INPUT_FILE]

Arguments:
  [INPUT_FILE]  The input (puzzle) file to read from. Omit to read from stdin

Options:
  -n, --no-border  Do not draw border to format the solution
  -h, --help       Print help
```

Example:
```bash
cat input.txt |sudoku-solver
```

## Input file format

Refer to [`read_to_puzzle()`](target/doc/sudoku_solver/prelude/fn.read_to_puzzle.html).
