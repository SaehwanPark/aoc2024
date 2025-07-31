use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
  row: i32,
  col: i32,
}

impl Position {
  fn new(row: i32, col: i32) -> Self {
    Self { row, col }
  }

  fn is_within_bounds(&self, grid_height: i32, grid_width: i32) -> bool {
    self.row >= 0 && self.row < grid_height && self.col >= 0 && self.col < grid_width
  }
}

struct Grid {
  height: i32,
  width: i32,
  antennas: HashMap<char, Vec<Position>>,
}

impl Grid {
  fn parse(input: &str) -> Self {
    let lines: Vec<&str> = input.trim().lines().collect();
    let height = lines.len() as i32;
    let width = lines.first().map_or(0, |line| line.len()) as i32;

    let mut antennas: HashMap<char, Vec<Position>> = HashMap::new();

    for (row, line) in lines.iter().enumerate() {
      for (col, ch) in line.chars().enumerate() {
        if ch != '.' {
          antennas
            .entry(ch)
            .or_default()
            .push(Position::new(row as i32, col as i32));
        }
      }
    }

    Self {
      height,
      width,
      antennas,
    }
  }

  fn find_antinodes(&self) -> HashSet<Position> {
    let mut antinodes = HashSet::new();

    for positions in self.antennas.values() {
      for (i, &pos1) in positions.iter().enumerate() {
        for &pos2 in positions.iter().skip(i + 1) {
          // Antinode 1: pos1 - (pos2 - pos1) = 2*pos1 - pos2
          let antinode1 = Position::new(2 * pos1.row - pos2.row, 2 * pos1.col - pos2.col);

          // Antinode 2: pos2 + (pos2 - pos1) = 2*pos2 - pos1
          let antinode2 = Position::new(2 * pos2.row - pos1.row, 2 * pos2.col - pos1.col);

          if antinode1.is_within_bounds(self.height, self.width) {
            antinodes.insert(antinode1);
          }

          if antinode2.is_within_bounds(self.height, self.width) {
            antinodes.insert(antinode2);
          }
        }
      }
    }

    antinodes
  }

  fn find_antinodes_alternatively(&self) -> HashSet<Position> {
    let mut antinodes = HashSet::new();

    for positions in self.antennas.values() {
      // Skip frequencies with only one antenna
      if positions.len() < 2 {
        continue;
      }

      // All antenna positions are antinodes when there are at least 2 antennas
      for &pos in positions {
        antinodes.insert(pos);
      }

      for (i, &pos1) in positions.iter().enumerate() {
        for &pos2 in positions.iter().skip(i + 1) {
          let row_diff = pos2.row - pos1.row;
          let col_diff = pos2.col - pos1.col;

          // Reduce the difference vector to its simplest form (GCD)
          let gcd = gcd(row_diff.abs(), col_diff.abs());
          let step_row = row_diff / gcd;
          let step_col = col_diff / gcd;

          // Find all antinodes in the positive direction from pos1
          let mut current_pos = pos1;
          loop {
            current_pos = Position::new(current_pos.row + step_row, current_pos.col + step_col);
            if !current_pos.is_within_bounds(self.height, self.width) {
              break;
            }
            antinodes.insert(current_pos);
          }

          // Find all antinodes in the negative direction from pos1
          current_pos = pos1;
          loop {
            current_pos = Position::new(current_pos.row - step_row, current_pos.col - step_col);
            if !current_pos.is_within_bounds(self.height, self.width) {
              break;
            }
            antinodes.insert(current_pos);
          }
        }
      }
    }

    antinodes
  }
}

fn gcd(a: i32, b: i32) -> i32 {
  if b == 0 { a } else { gcd(b, a % b) }
}

fn solve(input: &str, part: u8) -> usize {
  let grid = Grid::parse(input);
  match part {
    1 => grid.find_antinodes().len(),
    2 => grid.find_antinodes_alternatively().len(),
    _ => panic!("Only parts 1 and 2."),
  }
}

fn print_result(filepath: &str, puzzle_kind: &str) -> Result<()> {
  let input = fs::read_to_string(filepath)?;
  println!("Input: {puzzle_kind}");
  println!("Part 1 result = {}", solve(&input, 1));
  println!("Part 2 result = {}\n", solve(&input, 2));
  Ok(())
}

fn main() -> Result<()> {
  print_result("input/day08_simple.txt", "Simple puzzle")?;
  print_result("input/day08_full.txt", "Full puzzle")?;
  Ok(())
}
