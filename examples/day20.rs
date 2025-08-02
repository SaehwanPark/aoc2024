use anyhow::Result;
use std::collections::{HashMap, VecDeque};
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
  row: usize,
  col: usize,
}

impl Point {
  fn new(row: usize, col: usize) -> Self {
    Self { row, col }
  }

  fn neighbors(&self) -> Vec<Point> {
    let mut neighbors = Vec::new();
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    for (dr, dc) in directions {
      let new_row = self.row as isize + dr;
      let new_col = self.col as isize + dc;

      if new_row >= 0 && new_col >= 0 {
        neighbors.push(Point::new(new_row as usize, new_col as usize));
      }
    }

    neighbors
  }
}

fn is_valid_pos(grid: &[Vec<char>], pos: Point) -> bool {
  pos.row < grid.len() && pos.col < grid[0].len()
}

fn is_track(grid: &[Vec<char>], pos: Point) -> bool {
  if !is_valid_pos(grid, pos) {
    return false;
  }
  let ch = grid[pos.row][pos.col];
  ch == '.' || ch == 'S' || ch == 'E'
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Point, Point) {
  let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
  let mut start = Point::new(0, 0);
  let mut end = Point::new(0, 0);

  for (row, line) in grid.iter().enumerate() {
    for (col, &ch) in line.iter().enumerate() {
      if ch == 'S' {
        start = Point::new(row, col);
      } else if ch == 'E' {
        end = Point::new(row, col);
      }
    }
  }

  (grid, start, end)
}

fn find_path(grid: &[Vec<char>], start: Point, end: Point) -> Vec<Point> {
  let mut queue = VecDeque::new();
  let mut visited = HashMap::new();
  let mut parent = HashMap::new();

  queue.push_back(start);
  visited.insert(start, 0);

  while let Some(current) = queue.pop_front() {
    if current == end {
      break;
    }

    for neighbor in current.neighbors() {
      if is_track(grid, neighbor) && !visited.contains_key(&neighbor) {
        visited.insert(neighbor, visited[&current] + 1);
        parent.insert(neighbor, current);
        queue.push_back(neighbor);
      }
    }
  }

  // Reconstruct path
  let mut path = Vec::new();
  let mut current = end;
  path.push(current);

  while let Some(&prev) = parent.get(&current) {
    path.push(prev);
    current = prev;
  }

  path.reverse();
  path
}

fn solve_with_cheat_limit(input: &str, min_savings: usize, max_cheat_time: usize) -> usize {
  let (grid, start, end) = parse_input(input);
  let path = find_path(&grid, start, end);

  // Create a map from position to index in path
  let mut pos_to_index = HashMap::new();
  for (i, &pos) in path.iter().enumerate() {
    pos_to_index.insert(pos, i);
  }

  let mut cheat_count = 0;
  let max_dist = max_cheat_time as isize;

  // For each position on the path, try all possible cheats
  for (start_idx, &cheat_start) in path.iter().enumerate() {
    // Try all positions within max_cheat_time Manhattan distance
    for dr in -max_dist..=max_dist {
      for dc in -max_dist..=max_dist {
        let manhattan_dist = dr.abs() + dc.abs();
        if manhattan_dist == 0 || manhattan_dist > max_dist {
          continue;
        }

        let cheat_end_row = cheat_start.row as isize + dr;
        let cheat_end_col = cheat_start.col as isize + dc;

        if cheat_end_row < 0 || cheat_end_col < 0 {
          continue;
        }

        let cheat_end = Point::new(cheat_end_row as usize, cheat_end_col as usize);

        // Check if cheat_end is a valid track position and on the path
        if is_track(&grid, cheat_end) {
          if let Some(&end_idx) = pos_to_index.get(&cheat_end) {
            if end_idx > start_idx {
              let normal_dist = end_idx - start_idx;
              let cheat_dist = manhattan_dist as usize;

              if normal_dist > cheat_dist {
                let time_saved = normal_dist - cheat_dist;

                if time_saved >= min_savings {
                  cheat_count += 1;
                }
              }
            }
          }
        }
      }
    }
  }

  cheat_count
}

fn solve(input: &str, part: u8) -> usize {
  let min_savings = 100;
  let cheat_limit = match part {
    1 => 2,
    2 => 20,
    _ => panic!("Only part 1 or 2 is possible."),
  };
  solve_with_cheat_limit(input, min_savings, cheat_limit)
}

fn print_result(filepath: &str, puzzle_kind: &str) -> Result<()> {
  let input = fs::read_to_string(filepath)?;
  println!("Input: {puzzle_kind}");
  println!("Part 1 result = {}", solve(&input, 1));
  println!("Part 2 result = {}\n", solve(&input, 2));
  Ok(())
}

fn main() -> Result<()> {
  print_result("input/day20_simple.txt", "Simple puzzle")?;
  print_result("input/day20_full.txt", "Full puzzle")?;
  Ok(())
}
