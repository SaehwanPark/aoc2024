use anyhow::Result;
use std::collections::HashSet;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
  Up,
  Right,
  Down,
  Left,
}

impl Direction {
  fn turn_right(self) -> Self {
    match self {
      Direction::Up => Direction::Right,
      Direction::Right => Direction::Down,
      Direction::Down => Direction::Left,
      Direction::Left => Direction::Up,
    }
  }

  fn delta(self) -> (i32, i32) {
    match self {
      Direction::Up => (-1, 0),
      Direction::Right => (0, 1),
      Direction::Down => (1, 0),
      Direction::Left => (0, -1),
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
  row: i32,
  col: i32,
}

impl Position {
  fn new(row: i32, col: i32) -> Self {
    Self { row, col }
  }

  fn move_in_direction(self, direction: Direction) -> Self {
    let (delta_row, delta_col) = direction.delta();
    Self {
      row: self.row + delta_row,
      col: self.col + delta_col,
    }
  }
}

#[derive(Debug)]
struct Grid {
  cells: Vec<Vec<char>>,
  rows: usize,
  cols: usize,
}

impl Grid {
  fn from_input(input: &str) -> Self {
    let cells: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let rows = cells.len();
    let cols = if rows > 0 { cells[0].len() } else { 0 };

    Self { cells, rows, cols }
  }

  fn is_valid_position(&self, pos: Position) -> bool {
    pos.row >= 0 && pos.col >= 0 && (pos.row as usize) < self.rows && (pos.col as usize) < self.cols
  }

  fn get_cell(&self, pos: Position) -> Option<char> {
    if self.is_valid_position(pos) {
      Some(self.cells[pos.row as usize][pos.col as usize])
    } else {
      None
    }
  }

  fn find_guard_start(&self) -> Option<(Position, Direction)> {
    for (row_idx, row) in self.cells.iter().enumerate() {
      for (col_idx, &cell) in row.iter().enumerate() {
        let direction = match cell {
          '^' => Some(Direction::Up),
          '>' => Some(Direction::Right),
          'v' => Some(Direction::Down),
          '<' => Some(Direction::Left),
          _ => None,
        };

        if let Some(dir) = direction {
          return Some((Position::new(row_idx as i32, col_idx as i32), dir));
        }
      }
    }
    None
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct GuardState {
  pos: Position,
  dir: Direction,
}

#[derive(Debug)]
struct GuardSimulator {
  grid: Grid,
  guard_start_pos: Position,
  guard_start_dir: Direction,
}

impl GuardSimulator {
  fn new(input: &str) -> Result<Self, String> {
    let grid = Grid::from_input(input);
    let (guard_start_pos, guard_start_dir) = grid
      .find_guard_start()
      .ok_or("No guard found in the grid")?;

    Ok(Self {
      grid,
      guard_start_pos,
      guard_start_dir,
    })
  }

  fn simulate_patrol(&self) -> HashSet<Position> {
    let mut guard_pos = self.guard_start_pos;
    let mut guard_dir = self.guard_start_dir;
    let mut visited_positions = HashSet::new();
    visited_positions.insert(guard_pos);

    loop {
      let next_pos = guard_pos.move_in_direction(guard_dir);

      // Check if guard would leave the grid
      if !self.grid.is_valid_position(next_pos) {
        break;
      }

      // Check if there's an obstacle in front
      if let Some(cell) = self.grid.get_cell(next_pos) {
        if cell == '#' {
          // Turn right if obstacle found
          guard_dir = guard_dir.turn_right();
        } else {
          // Move forward if no obstacle
          guard_pos = next_pos;
          visited_positions.insert(guard_pos);
        }
      }
    }

    visited_positions
  }

  fn simulate_with_obstruction(&self, obstruction_pos: Position) -> bool {
    let mut guard_pos = self.guard_start_pos;
    let mut guard_dir = self.guard_start_dir;
    let mut visited_states = HashSet::new();

    loop {
      let current_state = GuardState {
        pos: guard_pos,
        dir: guard_dir,
      };

      // If we've seen this state before, we're in a loop
      if visited_states.contains(&current_state) {
        return true;
      }

      visited_states.insert(current_state);

      let next_pos = guard_pos.move_in_direction(guard_dir);

      // Check if guard would leave the grid
      if !self.grid.is_valid_position(next_pos) {
        return false;
      }

      // Check if there's an obstacle in front (including our new obstruction)
      let is_obstacle = if next_pos == obstruction_pos {
        true
      } else {
        self.grid.get_cell(next_pos) == Some('#')
      };

      if is_obstacle {
        // Turn right if obstacle found
        guard_dir = guard_dir.turn_right();
      } else {
        // Move forward if no obstacle
        guard_pos = next_pos;
      }
    }
  }

  fn count_loop_positions(&self) -> usize {
    // First, get all positions the guard visits in normal patrol
    let visited_positions = self.simulate_patrol();

    let mut loop_count = 0;

    // Test placing an obstruction at each visited position (except start)
    for &pos in &visited_positions {
      if pos == self.guard_start_pos {
        continue; // Can't place obstruction at guard's starting position
      }

      if self.simulate_with_obstruction(pos) {
        loop_count += 1;
      }
    }

    loop_count
  }
}

fn solve(input: &str, part: u8) -> Result<usize> {
  let simulator = GuardSimulator::new(input).expect("Invalid input");
  match part {
    1 => Ok(simulator.simulate_patrol().len()),
    2 => Ok(simulator.count_loop_positions()),
    _ => panic!("Only parts 1 and 2."),
  }
}

fn print_result(filepath: &str, puzzle_kind: &str) -> Result<()> {
  let input = fs::read_to_string(filepath)?;
  let result1 = solve(&input, 1)?;
  let result2 = solve(&input, 2)?;
  println!("Input: {puzzle_kind}");
  println!("Part 1 result = {result1}");
  println!("Part 2 result = {result2}\n");
  Ok(())
}

fn main() -> Result<()> {
  print_result("input/day06_simple.txt", "Simple puzzle")?;
  print_result("input/day06_full.txt", "Full puzzle")?;
  Ok(())
}
