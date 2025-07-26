#![allow(clippy::too_many_arguments)]

use std::fs;

fn check_word_at_position(
  grid: &[Vec<char>],
  start_row: usize,
  start_col: usize,
  dx: i32,
  dy: i32,
  target: &[char],
  rows: usize,
  cols: usize,
) -> bool {
  for (i, &target_char) in target.iter().enumerate() {
    let new_row = start_row as i32 + (i as i32 * dx);
    let new_col = start_col as i32 + (i as i32 * dy);

    // check bounds
    if (new_row < 0) || (new_row >= rows as i32) || (new_col < 0) || (new_col >= cols as i32) {
      return false;
    }

    let new_row = new_row as usize;
    let new_col = new_col as usize;

    // check if character matches
    if grid[new_row][new_col] != target_char {
      return false;
    }
  }
  true
}

fn is_mas_diagonal(char1: char, char2: char) -> bool {
  // check if the 2 characters from MAS or SAM when combined with A
  (char1 == 'M' && char2 == 'S') || (char1 == 'S' && char2 == 'M')
}

fn is_xmas_center(grid: &[Vec<char>], center_row: usize, center_col: usize) -> bool {
  let top_left = grid[center_row - 1][center_col - 1];
  let top_right = grid[center_row - 1][center_col + 1];
  let bottom_left = grid[center_row + 1][center_col - 1];
  let bottom_right = grid[center_row + 1][center_col + 1];

  let diag1_valid = is_mas_diagonal(top_left, bottom_right);
  let diag2_valid = is_mas_diagonal(top_right, bottom_left);

  diag1_valid && diag2_valid
}

fn solve_part1(input: &str) -> usize {
  let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
  let rows = grid.len();
  let cols = grid[0].len();
  let target = "XMAS";
  let target_chars: Vec<char> = target.chars().collect();
  //let target_len = target_chars.len();

  let directions = [
    (0, 1),   // right
    (0, -1),  // left
    (1, 0),   // down
    (-1, 0),  // up
    (1, 1),   // down-right
    (1, -1),  // down-left
    (-1, 1),  // up-right
    (-1, -1), // up-left
  ];

  let mut count = 0;

  for row in 0..rows {
    for col in 0..cols {
      // try each direction from this position
      for &(dx, dy) in &directions {
        if check_word_at_position(&grid, row, col, dx, dy, &target_chars, rows, cols) {
          count += 1;
        }
      }
    }
  }
  count
}

fn solve_part2(input: &str) -> usize {
  let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
  let rows = grid.len();
  let cols = grid[0].len();
  let mut count = 0;

  // look for X-MAS patterns: we need to find A in the center
  for row in 1..rows - 1 {
    for col in 1..cols - 1 {
      if grid[row][col] == 'A' {
        // check if this A is the center of an X-MAS
        if is_xmas_center(&grid, row, col) {
          count += 1;
        }
      }
    }
  }

  count
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let simple_input = fs::read_to_string("input/day04_simple.txt")?;
  let simple_result_part1 = solve_part1(&simple_input);
  let simple_result_part2 = solve_part2(&simple_input);
  println!("Simple input result P1: {simple_result_part1}");
  println!("Simple input result P2: {simple_result_part2}");

  let full_input = fs::read_to_string("input/day04_full.txt")?;
  let full_result_part1 = solve_part1(&full_input);
  let full_result_part2 = solve_part2(&full_input);

  println!("Full input result P1: {full_result_part1}");
  println!("Full input result P2: {full_result_part2}");

  Ok(())
}
