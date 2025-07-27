use std::collections::{HashSet, VecDeque};
use std::fs;

type Position = (usize, usize);

#[derive(Debug)]
struct TopographicMap {
  grid: Vec<Vec<u8>>,
  rows: usize,
  cols: usize,
}

impl TopographicMap {
  fn new(input: &str) -> Self {
    let grid: Vec<Vec<u8>> = input
      .lines()
      .map(|line| {
        line
          .chars()
          .map(|c| c.to_digit(10).unwrap() as u8)
          .collect()
      })
      .collect();

    let rows = grid.len();
    let cols = grid.first().map_or(0, |row| row.len());

    Self { grid, rows, cols }
  }

  fn height_at(&self, pos: Position) -> u8 {
    self.grid[pos.0][pos.1]
  }

  fn find_trailheads(&self) -> Vec<Position> {
    let mut trailheads = Vec::new();

    for (row_idx, row) in self.grid.iter().enumerate() {
      for (col_idx, &height) in row.iter().enumerate() {
        if height == 0 {
          trailheads.push((row_idx, col_idx));
        }
      }
    }

    trailheads
  }

  fn get_valid_neighbors(&self, pos: Position) -> Vec<Position> {
    let (row, col) = pos;
    let current_height = self.height_at(pos);
    let mut neighbors = Vec::new();

    // Check all four cardinal directions
    let directions = [
      (-1i32, 0i32), // up
      (1, 0),        // down
      (0, -1),       // left
      (0, 1),        // right
    ];

    for (dr, dc) in directions {
      let new_row = row as i32 + dr;
      let new_col = col as i32 + dc;

      // Check bounds
      if new_row >= 0 && new_row < self.rows as i32 && new_col >= 0 && new_col < self.cols as i32 {
        let new_pos = (new_row as usize, new_col as usize);
        let new_height = self.height_at(new_pos);

        // Valid trail step: height increases by exactly 1
        if new_height == current_height + 1 {
          neighbors.push(new_pos);
        }
      }
    }

    neighbors
  }

  fn calculate_trailhead_score(&self, trailhead: Position) -> usize {
    let mut reachable_nines = HashSet::new();
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back(trailhead);
    visited.insert(trailhead);

    while let Some(current_pos) = queue.pop_front() {
      let current_height = self.height_at(current_pos);

      // If we reached a height of 9, record it
      if current_height == 9 {
        reachable_nines.insert(current_pos);
        continue;
      }

      // Explore valid neighbors
      for neighbor_pos in self.get_valid_neighbors(current_pos) {
        if visited.insert(neighbor_pos) {
          queue.push_back(neighbor_pos);
        }
      }
    }

    reachable_nines.len()
  }

  fn calculate_trailhead_rating(&self, trailhead: Position) -> usize {
    self.count_distinct_trails(trailhead)
  }

  fn count_distinct_trails(&self, pos: Position) -> usize {
    let current_height = self.height_at(pos);

    // Base case: if we reached height 9, this is one complete trail
    if current_height == 9 {
      return 1;
    }

    // Count all possible trails from valid neighbors
    self
      .get_valid_neighbors(pos)
      .iter()
      .map(|&neighbor_pos| self.count_distinct_trails(neighbor_pos))
      .sum()
  }

  fn solve_part1(&self) -> usize {
    self
      .find_trailheads()
      .iter()
      .map(|&trailhead| self.calculate_trailhead_score(trailhead))
      .sum()
  }

  fn solve_part2(&self) -> usize {
    self
      .find_trailheads()
      .iter()
      .map(|&trailhead| self.calculate_trailhead_rating(trailhead))
      .sum()
  }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Test with simple example
  let simple_input = fs::read_to_string("input/day10_simple.txt")?;
  let simple_map = TopographicMap::new(&simple_input);
  println!("Simple example Part 1: {}", simple_map.solve_part1());
  println!("Simple example Part 2: {}", simple_map.solve_part2());

  // Solve with full input
  let full_input = fs::read_to_string("input/day10_full.txt")?;
  let full_map = TopographicMap::new(&full_input);
  println!("Part 1 result: {}", full_map.solve_part1());
  println!("Part 2 result: {}", full_map.solve_part2());

  Ok(())
}
