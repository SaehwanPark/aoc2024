use anyhow::Result;
use std::collections::{HashSet, VecDeque};
use std::fs;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Position {
  x: i32,
  y: i32,
}

impl Position {
  fn new(x: i32, y: i32) -> Self {
    Self { x, y }
  }

  fn neighbors(&self) -> [Position; 4] {
    [
      Position::new(self.x + 1, self.y),
      Position::new(self.x - 1, self.y),
      Position::new(self.x, self.y + 1),
      Position::new(self.x, self.y - 1),
    ]
  }

  fn is_valid(&self, grid_size: i32) -> bool {
    self.x >= 0 && self.x < grid_size && self.y >= 0 && self.y < grid_size
  }
}

fn parse_input(input: &str) -> Vec<Position> {
  input
    .lines()
    .map(|line| {
      let parts: Vec<&str> = line.split(',').collect();
      Position::new(
        parts[0].parse().expect("Invalid x coordinate"),
        parts[1].parse().expect("Invalid y coordinate"),
      )
    })
    .collect()
}

fn bfs_shortest_path(
  start: Position,
  end: Position,
  corrupted: &HashSet<Position>,
  grid_size: i32,
) -> Option<i32> {
  let mut queue = VecDeque::new();
  let mut visited = HashSet::new();

  queue.push_back((start, 0));
  visited.insert(start);

  while let Some((current, steps)) = queue.pop_front() {
    if current == end {
      return Some(steps);
    }

    for neighbor in current.neighbors() {
      if neighbor.is_valid(grid_size)
        && !corrupted.contains(&neighbor)
        && !visited.contains(&neighbor)
      {
        visited.insert(neighbor);
        queue.push_back((neighbor, steps + 1));
      }
    }
  }

  None
}

fn minimize_steps_to_exit(
  byte_positions: &[Position],
  grid_size: i32,
  num_bytes: usize,
) -> Option<i32> {
  let corrupted: HashSet<Position> = byte_positions.iter().take(num_bytes).cloned().collect();

  let start = Position::new(0, 0);
  let end = Position::new(grid_size - 1, grid_size - 1);

  bfs_shortest_path(start, end, &corrupted, grid_size) // error defaults to -1
}

fn get_first_byte_coordinate_to_prevent_exit(
  byte_positions: &[Position],
  grid_size: i32,
) -> Option<Position> {
  let start = Position::new(0, 0);
  let end = Position::new(grid_size - 1, grid_size - 1);

  // Binary search for the first byte that blocks the path
  let mut left = 0;
  let mut right = byte_positions.len();

  while left < right {
    let mid = (left + right) / 2;
    let corrupted: HashSet<Position> = byte_positions.iter().take(mid + 1).cloned().collect();

    if bfs_shortest_path(start, end, &corrupted, grid_size).is_some() {
      // Path still exists, need more bytes
      left = mid + 1;
    } else {
      // Path blocked, this might be our answer
      right = mid;
    }
  }

  // left should now point to the first byte that blocks the path
  if left < byte_positions.len() {
    Some(byte_positions[left])
  } else {
    None
  }
}

fn solve(input: &str, grid_size: i32, num_bytes: usize, part: u8) -> String {
  let byte_positions = parse_input(input);
  match part {
    1 => minimize_steps_to_exit(&byte_positions, grid_size, num_bytes)
      .map_or(String::from("None"), |x| x.to_string()),
    2 => get_first_byte_coordinate_to_prevent_exit(&byte_positions, grid_size)
      .map_or(String::from("None"), |p| format!("{},{}", p.x, p.y)),
    _ => panic!("Only parts 1 or 2."),
  }
}

fn print_result(filepath: &str, puzzle_kind: &str) -> Result<()> {
  let input = fs::read_to_string(filepath)?;
  let (grid_size, num_bytes) = match puzzle_kind {
    "Simple puzzle" => (7, 12),
    "Full puzzle" => (71, 1024),
    _ => panic!("Unsupported puzzle!"),
  };
  println!("Input: {puzzle_kind}");
  println!("Part 1 result = {}", solve(&input, grid_size, num_bytes, 1));
  println!(
    "Part 2 result = {}\n",
    solve(&input, grid_size, num_bytes, 2)
  );
  Ok(())
}

fn main() -> Result<()> {
  print_result("input/day18_simple.txt", "Simple puzzle")?;
  print_result("input/day18_full.txt", "Full puzzle")?;
  Ok(())
}
