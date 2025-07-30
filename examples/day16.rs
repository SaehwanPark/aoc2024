use anyhow::Result;
use std::collections::{BinaryHeap, HashMap};
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
  North,
  East,
  South,
  West,
}

impl Direction {
  fn turn_clockwise(self) -> Self {
    match self {
      Direction::North => Direction::East,
      Direction::East => Direction::South,
      Direction::South => Direction::West,
      Direction::West => Direction::North,
    }
  }

  fn turn_counterclockwise(self) -> Self {
    match self {
      Direction::North => Direction::West,
      Direction::West => Direction::South,
      Direction::South => Direction::East,
      Direction::East => Direction::North,
    }
  }

  fn delta(self) -> (i32, i32) {
    match self {
      Direction::North => (-1, 0),
      Direction::East => (0, 1),
      Direction::South => (1, 0),
      Direction::West => (0, -1),
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
  row: usize,
  col: usize,
}

impl Position {
  fn new(row: usize, col: usize) -> Self {
    Self { row, col }
  }

  fn move_in_direction(self, direction: Direction, rows: usize, cols: usize) -> Option<Self> {
    let (dr, dc) = direction.delta();
    let new_row = self.row as i32 + dr;
    let new_col = self.col as i32 + dc;

    if new_row >= 0 && new_row < rows as i32 && new_col >= 0 && new_col < cols as i32 {
      Some(Position::new(new_row as usize, new_col as usize))
    } else {
      None
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
  pos: Position,
  dir: Direction,
}

impl State {
  fn new(pos: Position, dir: Direction) -> Self {
    Self { pos, dir }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Node {
  cost: u32,
  state: State,
}

impl Ord for Node {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    other.cost.cmp(&self.cost) // Reverse for min-heap
  }
}

impl PartialOrd for Node {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

struct Maze {
  grid: Vec<Vec<char>>,
  start_pos: Position,
  end_pos: Position,
  rows: usize,
  cols: usize,
}

impl Maze {
  fn from_input(input: &str) -> Self {
    let lines: Vec<&str> = input.trim().lines().collect();
    let rows = lines.len();
    let cols = lines[0].len();
    let mut grid = vec![vec!['.'; cols]; rows];
    let mut start_pos = Position::new(0, 0);
    let mut end_pos = Position::new(0, 0);

    for (row, line) in lines.iter().enumerate() {
      for (col, ch) in line.chars().enumerate() {
        grid[row][col] = ch;
        match ch {
          'S' => start_pos = Position::new(row, col),
          'E' => end_pos = Position::new(row, col),
          _ => {}
        }
      }
    }

    Self {
      grid,
      start_pos,
      end_pos,
      rows,
      cols,
    }
  }

  fn is_wall(&self, pos: Position) -> bool {
    self.grid[pos.row][pos.col] == '#'
  }

  fn dijkstra_from_start(&self) -> HashMap<State, u32> {
    let mut heap = BinaryHeap::new();
    let mut distances: HashMap<State, u32> = HashMap::new();

    let start_state = State::new(self.start_pos, Direction::East);
    heap.push(Node {
      cost: 0,
      state: start_state,
    });
    distances.insert(start_state, 0);

    while let Some(Node { cost, state }) = heap.pop() {
      if let Some(&best_cost) = distances.get(&state) {
        if cost > best_cost {
          continue;
        }
      }

      // Try moving forward (cost: 1)
      if let Some(next_pos) = state.pos.move_in_direction(state.dir, self.rows, self.cols) {
        if !self.is_wall(next_pos) {
          let next_state = State::new(next_pos, state.dir);
          let next_cost = cost + 1;

          let should_update = distances
            .get(&next_state)
            .is_none_or(|&existing_cost| next_cost < existing_cost);

          if should_update {
            distances.insert(next_state, next_cost);
            heap.push(Node {
              cost: next_cost,
              state: next_state,
            });
          }
        }
      }

      // Try turning clockwise (cost: 1000)
      let clockwise_state = State::new(state.pos, state.dir.turn_clockwise());
      let turn_cost = cost + 1000;

      let should_update = distances
        .get(&clockwise_state)
        .is_none_or(|&existing_cost| turn_cost < existing_cost);

      if should_update {
        distances.insert(clockwise_state, turn_cost);
        heap.push(Node {
          cost: turn_cost,
          state: clockwise_state,
        });
      }

      // Try turning counterclockwise (cost: 1000)
      let counterclockwise_state = State::new(state.pos, state.dir.turn_counterclockwise());

      let should_update = distances
        .get(&counterclockwise_state)
        .is_none_or(|&existing_cost| turn_cost < existing_cost);

      if should_update {
        distances.insert(counterclockwise_state, turn_cost);
        heap.push(Node {
          cost: turn_cost,
          state: counterclockwise_state,
        });
      }
    }

    distances
  }

  fn dijkstra_from_end(&self) -> HashMap<State, u32> {
    let mut heap = BinaryHeap::new();
    let mut distances: HashMap<State, u32> = HashMap::new();

    // Start from end position in all directions
    for &dir in &[
      Direction::North,
      Direction::East,
      Direction::South,
      Direction::West,
    ] {
      let end_state = State::new(self.end_pos, dir);
      heap.push(Node {
        cost: 0,
        state: end_state,
      });
      distances.insert(end_state, 0);
    }

    while let Some(Node { cost, state }) = heap.pop() {
      if let Some(&best_cost) = distances.get(&state) {
        if cost > best_cost {
          continue;
        }
      }

      // Try moving backward (reverse direction)
      let reverse_dir = match state.dir {
        Direction::North => Direction::South,
        Direction::East => Direction::West,
        Direction::South => Direction::North,
        Direction::West => Direction::East,
      };

      if let Some(prev_pos) = state
        .pos
        .move_in_direction(reverse_dir, self.rows, self.cols)
      {
        if !self.is_wall(prev_pos) {
          let prev_state = State::new(prev_pos, state.dir);
          let prev_cost = cost + 1;

          let should_update = distances
            .get(&prev_state)
            .is_none_or(|&existing_cost| prev_cost < existing_cost);

          if should_update {
            distances.insert(prev_state, prev_cost);
            heap.push(Node {
              cost: prev_cost,
              state: prev_state,
            });
          }
        }
      }

      // Try reverse turns (clockwise -> counterclockwise, counterclockwise -> clockwise)
      let from_clockwise = State::new(state.pos, state.dir.turn_counterclockwise());
      let turn_cost = cost + 1000;

      let should_update = distances
        .get(&from_clockwise)
        .is_none_or(|&existing_cost| turn_cost < existing_cost);

      if should_update {
        distances.insert(from_clockwise, turn_cost);
        heap.push(Node {
          cost: turn_cost,
          state: from_clockwise,
        });
      }

      let from_counterclockwise = State::new(state.pos, state.dir.turn_clockwise());

      let should_update = distances
        .get(&from_counterclockwise)
        .is_none_or(|&existing_cost| turn_cost < existing_cost);

      if should_update {
        distances.insert(from_counterclockwise, turn_cost);
        heap.push(Node {
          cost: turn_cost,
          state: from_counterclockwise,
        });
      }
    }

    distances
  }

  fn find_minimum_score(&self) -> u32 {
    let distances = self.dijkstra_from_start();

    // Find minimum cost to reach end position from any direction
    [
      Direction::North,
      Direction::East,
      Direction::South,
      Direction::West,
    ]
    .iter()
    .filter_map(|&dir| distances.get(&State::new(self.end_pos, dir)))
    .min()
    .copied()
    .unwrap_or(u32::MAX)
  }

  fn find_optimal_tiles(&self) -> usize {
    let from_start = self.dijkstra_from_start();
    let from_end = self.dijkstra_from_end();

    let min_score = self.find_minimum_score();
    let mut optimal_tiles = std::collections::HashSet::new();

    // A tile is optimal if there exists a direction such that:
    // distance_from_start(pos, dir) + distance_to_end(pos, dir) == min_score
    for row in 0..self.rows {
      for col in 0..self.cols {
        let pos = Position::new(row, col);
        if self.is_wall(pos) {
          continue;
        }

        for &dir in &[
          Direction::North,
          Direction::East,
          Direction::South,
          Direction::West,
        ] {
          let state = State::new(pos, dir);

          if let (Some(&dist_from_start), Some(&dist_to_end)) =
            (from_start.get(&state), from_end.get(&state))
          {
            if dist_from_start + dist_to_end == min_score {
              optimal_tiles.insert(pos);
              break; // Found one direction that works, no need to check others
            }
          }
        }
      }
    }

    optimal_tiles.len()
  }
}

fn solve(input: &str, part: u8) -> usize {
  let maze = Maze::from_input(input);
  match part {
    1 => maze.find_minimum_score() as usize,
    2 => maze.find_optimal_tiles(),
    _ => panic!("Only parts 1 or 2."),
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
  print_result("input/day16_simple.txt", "Simple puzzle")?;
  print_result("input/day16_full.txt", "Full puzzle")?;
  Ok(())
}
