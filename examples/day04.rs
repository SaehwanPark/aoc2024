use anyhow::Result;
use std::fs;

type Grid = Vec<Vec<char>>;
type Direction = (i32, i32);

const DIRECTIONS: [Direction; 8] = [
  (0, 1), // right
  (0, -1),
  (1, 0), // down
  (-1, 0),
  (1, 1),
  (1, -1),
  (-1, 1),
  (-1, -1),
];

fn is_within_bounds(row: i32, col: i32, rows: usize, cols: usize) -> bool {
  row >= 0 && row < rows as i32 && col >= 0 && col < cols as i32
}

fn is_mas_diagnonal(char1: char, char2: char) -> bool {
  // check if the 2 characters form MAS or SAM when combined with A
  matches!((char1, char2), ('M', 'S') | ('S', 'M'))
}

fn check_word_at_position(
  grid: &Grid,
  start_row: usize,
  start_col: usize,
  direction: Direction,
  target: &[char],
) -> bool {
  let (dx, dy) = direction;
  let (rows, cols) = (grid.len(), grid[0].len());

  for (i, &target_char) in target.iter().enumerate() {
    let new_row = start_row as i32 + (i as i32 * dx);
    let new_col = start_col as i32 + (i as i32 * dy);

    if !is_within_bounds(new_row, new_col, rows, cols) {
      return false;
    }

    let (row_idx, col_idx) = (new_row as usize, new_col as usize);
    if grid[row_idx][col_idx] != target_char {
      return false;
    }
  }
  true
}

fn is_xmas_center(grid: &Grid, center_row: usize, center_col: usize) -> bool {
  let top_left = grid[center_row - 1][center_col - 1];
  let top_right = grid[center_row - 1][center_col + 1];
  let bottom_left = grid[center_row + 1][center_col - 1];
  let bottom_right = grid[center_row + 1][center_col + 1];

  is_mas_diagnonal(top_left, bottom_right) && is_mas_diagnonal(top_right, bottom_left)
}

fn parse_grid(input: &str) -> Grid {
  input.lines().map(|l| l.chars().collect()).collect()
}

fn solve_part1(input: &str) -> usize {
  let grid = parse_grid(input);
  let (rows, cols) = (grid.len(), grid[0].len());
  let target_chars: Vec<char> = "XMAS".chars().collect();
  let mut count = 0;
  for row in 0..rows {
    for col in 0..cols {
      for &dir in &DIRECTIONS {
        if check_word_at_position(&grid, row, col, dir, &target_chars) {
          count += 1;
        }
      }
    }
  }
  count
}

fn solve_part2(input: &str) -> usize {
  let grid = parse_grid(input);
  let (rows, cols) = (grid.len(), grid[0].len());
  let mut count = 0;

  // look for X-MAS patterns: find A in the center
  for row in 1..rows - 1 {
    for col in 1..cols - 1 {
      if grid[row][col] == 'A' && is_xmas_center(&grid, row, col) {
        count += 1;
      }
    }
  }
  count
}

fn process_input_file(filename: &str, file_type: &str) -> Result<()> {
  let input = fs::read_to_string(filename)?;
  let part1_result = solve_part1(&input);
  let part2_result = solve_part2(&input);

  println!("{file_type} input result P1: {part1_result}");
  println!("{file_type} input result P2: {part2_result}");

  Ok(())
}

fn main() -> Result<()> {
  process_input_file("input/day04_simple.txt", "Simple")?;
  process_input_file("input/day04_full.txt", "Full")?;
  Ok(())
}
