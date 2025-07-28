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

fn parse_input(filename: &str) -> Vec<Position> {
  let content =
    fs::read_to_string(filename).unwrap_or_else(|_| panic!("Failed to read file: {filename}"));

  content
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

fn solve_part1(filename: &str, grid_size: i32, num_bytes: usize) -> Option<i32> {
  let byte_positions = parse_input(filename);
  let corrupted: HashSet<Position> = byte_positions.iter().take(num_bytes).cloned().collect();

  let start = Position::new(0, 0);
  let end = Position::new(grid_size - 1, grid_size - 1);

  bfs_shortest_path(start, end, &corrupted, grid_size)
}

fn solve_part2(filename: &str, grid_size: i32) -> Option<Position> {
  let byte_positions = parse_input(filename);
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

fn main() {
  // Test with example (7x7 grid, first 12 bytes)
  match solve_part1("input/day18_simple.txt", 7, 12) {
    Some(steps) => println!("Example: Minimum steps needed: {steps}"),
    None => println!("Example: No path found!"),
  }

  // Solve full problem (71x71 grid, first 1024 bytes)
  match solve_part1("input/day18_full.txt", 71, 1024) {
    Some(steps) => println!("Part 1: Minimum steps needed: {steps}"),
    None => println!("Part 1: No path found!"),
  }

  // Part 2: Find first blocking byte
  match solve_part2("input/day18_simple.txt", 7) {
    Some(pos) => println!(
      "Example Part 2: First blocking byte at: {},{}",
      pos.x, pos.y
    ),
    None => println!("Example Part 2: No blocking byte found!"),
  }

  match solve_part2("input/day18_full.txt", 71) {
    Some(pos) => println!("Part 2: First blocking byte at: {},{}", pos.x, pos.y),
    None => println!("Part 2: No blocking byte found!"),
  }
}
