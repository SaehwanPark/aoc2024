use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
  Wall,
  Box,
  BoxLeft,  // Left part of wide box [
  BoxRight, // Right part of wide box ]
  Robot,
  Empty,
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

  fn move_direction(&self, direction: Direction) -> Self {
    match direction {
      Direction::Up => Position::new(self.row - 1, self.col),
      Direction::Down => Position::new(self.row + 1, self.col),
      Direction::Left => Position::new(self.row, self.col - 1),
      Direction::Right => Position::new(self.row, self.col + 1),
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
  Up,
  Down,
  Left,
  Right,
}

impl Direction {
  fn from_char(c: char) -> Option<Self> {
    match c {
      '^' => Some(Direction::Up),
      'v' => Some(Direction::Down),
      '<' => Some(Direction::Left),
      '>' => Some(Direction::Right),
      _ => None,
    }
  }
}

struct Warehouse {
  grid: HashMap<Position, Cell>,
  robot_pos: Position,
  width: i32,
  height: i32,
}

impl Warehouse {
  fn from_input(input: &str) -> Self {
    let (map_str, _) = input.split_once("\n\n").expect("Invalid input format");

    let mut grid = HashMap::new();
    let mut robot_pos = Position::new(0, 0);
    let lines: Vec<&str> = map_str.lines().collect();
    let height = lines.len() as i32;
    let width = lines.first().map_or(0, |line| line.len()) as i32;

    for (row, line) in lines.iter().enumerate() {
      for (col, ch) in line.chars().enumerate() {
        let pos = Position::new(row as i32, col as i32);
        let cell = match ch {
          '#' => Cell::Wall,
          'O' => Cell::Box,
          '@' => {
            robot_pos = pos;
            Cell::Robot
          }
          '.' => Cell::Empty,
          _ => panic!("Invalid character in map: {ch}"),
        };
        grid.insert(pos, cell);
      }
    }

    Self {
      grid,
      robot_pos,
      width,
      height,
    }
  }

  fn from_input_scaled(input: &str) -> Self {
    let (map_str, _) = input.split_once("\n\n").expect("Invalid input format");

    let mut grid = HashMap::new();
    let mut robot_pos = Position::new(0, 0);
    let lines: Vec<&str> = map_str.lines().collect();
    let height = lines.len() as i32;
    let width = lines.first().map_or(0, |line| line.len() * 2) as i32;

    for (row, line) in lines.iter().enumerate() {
      for (col, ch) in line.chars().enumerate() {
        let left_pos = Position::new(row as i32, (col * 2) as i32);
        let right_pos = Position::new(row as i32, (col * 2 + 1) as i32);

        match ch {
          '#' => {
            grid.insert(left_pos, Cell::Wall);
            grid.insert(right_pos, Cell::Wall);
          }
          'O' => {
            grid.insert(left_pos, Cell::BoxLeft);
            grid.insert(right_pos, Cell::BoxRight);
          }
          '@' => {
            robot_pos = left_pos;
            grid.insert(left_pos, Cell::Robot);
            grid.insert(right_pos, Cell::Empty);
          }
          '.' => {
            grid.insert(left_pos, Cell::Empty);
            grid.insert(right_pos, Cell::Empty);
          }
          _ => panic!("Invalid character in map: {ch}"),
        };
      }
    }

    Self {
      grid,
      robot_pos,
      width,
      height,
    }
  }

  fn get_cell(&self, pos: Position) -> Cell {
    *self.grid.get(&pos).unwrap_or(&Cell::Wall)
  }

  fn set_cell(&mut self, pos: Position, cell: Cell) {
    self.grid.insert(pos, cell);
  }

  fn can_push_boxes(&self, start_pos: Position, direction: Direction) -> Option<Vec<Position>> {
    let mut positions_to_move = Vec::new();
    let mut current_pos = start_pos;

    loop {
      current_pos = current_pos.move_direction(direction);

      match self.get_cell(current_pos) {
        Cell::Wall => return None, // Hit a wall, can't push
        Cell::Empty => break,      // Found empty space, can push
        Cell::Box => positions_to_move.push(current_pos),
        Cell::Robot => panic!("Unexpected robot position"),
        Cell::BoxLeft | Cell::BoxRight => return None, // Wide boxes need different logic
      }
    }

    Some(positions_to_move)
  }

  fn can_push_wide_boxes(
    &self,
    start_pos: Position,
    direction: Direction,
  ) -> Option<Vec<Position>> {
    use std::collections::{HashSet, VecDeque};

    let mut to_check = VecDeque::new();
    let mut boxes_to_move = HashSet::new();

    // Start checking from the position the robot wants to move to
    to_check.push_back(start_pos.move_direction(direction));

    while let Some(pos) = to_check.pop_front() {
      match self.get_cell(pos) {
        Cell::Wall => return None, // Can't push, hit a wall
        Cell::Empty => continue,   // Empty space, keep checking
        Cell::BoxLeft => {
          // Found left part of a box, add both parts to move
          let right_pos = Position::new(pos.row, pos.col + 1);
          if boxes_to_move.insert(pos) {
            // Only add to check queue if we haven't seen this box before
            match direction {
              Direction::Up | Direction::Down => {
                // For vertical movement, both parts of the box move
                to_check.push_back(pos.move_direction(direction));
                to_check.push_back(right_pos.move_direction(direction));
              }
              Direction::Left => {
                // For left movement, only check left of the left part
                to_check.push_back(pos.move_direction(direction));
              }
              Direction::Right => {
                // For right movement, only check right of the right part
                to_check.push_back(right_pos.move_direction(direction));
              }
            }
          }
          boxes_to_move.insert(right_pos);
        }
        Cell::BoxRight => {
          // Found right part of a box, add both parts to move
          let left_pos = Position::new(pos.row, pos.col - 1);
          if boxes_to_move.insert(pos) {
            // Only add to check queue if we haven't seen this box before
            match direction {
              Direction::Up | Direction::Down => {
                // For vertical movement, both parts of the box move
                to_check.push_back(pos.move_direction(direction));
                to_check.push_back(left_pos.move_direction(direction));
              }
              Direction::Left => {
                // For left movement, only check left of the left part
                to_check.push_back(left_pos.move_direction(direction));
              }
              Direction::Right => {
                // For right movement, only check right of the right part
                to_check.push_back(pos.move_direction(direction));
              }
            }
          }
          boxes_to_move.insert(left_pos);
        }
        Cell::Box => {
          // Regular single-cell box (part 1 compatibility)
          if boxes_to_move.insert(pos) {
            to_check.push_back(pos.move_direction(direction));
          }
        }
        Cell::Robot => panic!("Unexpected robot position"),
      }
    }

    Some(boxes_to_move.into_iter().collect())
  }

  fn move_robot(&mut self, direction: Direction) {
    let new_robot_pos = self.robot_pos.move_direction(direction);

    match self.get_cell(new_robot_pos) {
      Cell::Wall => (), // Can't move into wall
      Cell::Empty => {
        // Simple move
        self.set_cell(self.robot_pos, Cell::Empty);
        self.set_cell(new_robot_pos, Cell::Robot);
        self.robot_pos = new_robot_pos;
      }
      Cell::Box => {
        // Try to push regular boxes (Part 1)
        if let Some(box_positions) = self.can_push_boxes(self.robot_pos, direction) {
          // Move all boxes one position in the direction
          for &box_pos in box_positions.iter().rev() {
            let new_box_pos = box_pos.move_direction(direction);
            self.set_cell(box_pos, Cell::Empty);
            self.set_cell(new_box_pos, Cell::Box);
          }

          // Move robot
          self.set_cell(self.robot_pos, Cell::Empty);
          self.set_cell(new_robot_pos, Cell::Robot);
          self.robot_pos = new_robot_pos;
        }
        // If can't push, robot doesn't move
      }
      Cell::BoxLeft | Cell::BoxRight => {
        // Try to push wide boxes (Part 2)
        if let Some(box_positions) = self.can_push_wide_boxes(self.robot_pos, direction) {
          // Save the current state of boxes to move
          let mut boxes_state = Vec::new();
          for &pos in &box_positions {
            boxes_state.push((pos, self.get_cell(pos)));
          }

          // Clear all box positions first
          for &pos in &box_positions {
            self.set_cell(pos, Cell::Empty);
          }

          // Place boxes in their new positions
          for (pos, cell) in boxes_state {
            let new_pos = pos.move_direction(direction);
            self.set_cell(new_pos, cell);
          }

          // Move robot
          self.set_cell(self.robot_pos, Cell::Empty);
          self.set_cell(new_robot_pos, Cell::Robot);
          self.robot_pos = new_robot_pos;
        }
        // If can't push, robot doesn't move
      }
      Cell::Robot => panic!("Two robots found"),
    }
  }

  fn execute_moves(&mut self, moves: &str) {
    for ch in moves.chars() {
      if let Some(direction) = Direction::from_char(ch) {
        self.move_robot(direction);
      }
    }
  }

  fn calculate_gps_sum(&self) -> i32 {
    self
      .grid
      .iter()
      .filter_map(|(pos, &cell)| {
        match cell {
          Cell::Box => Some(100 * pos.row + pos.col),
          Cell::BoxLeft => Some(100 * pos.row + pos.col), // GPS is measured from left edge
          _ => None,
        }
      })
      .sum()
  }

  #[allow(dead_code)]
  fn print_warehouse(&self) {
    for row in 0..self.height {
      for col in 0..self.width {
        let pos = Position::new(row, col);
        let ch = match self.get_cell(pos) {
          Cell::Wall => '#',
          Cell::Box => 'O',
          Cell::BoxLeft => '[',
          Cell::BoxRight => ']',
          Cell::Robot => '@',
          Cell::Empty => '.',
        };
        print!("{ch}");
      }
      println!();
    }
    println!();
  }
}

fn parse_input(input: &str) -> (Warehouse, String) {
  let (_, moves_str) = input.split_once("\n\n").expect("Invalid input format");
  let warehouse = Warehouse::from_input(input);
  let moves = moves_str.replace('\n', "");
  (warehouse, moves)
}

fn parse_input_scaled(input: &str) -> (Warehouse, String) {
  let (_, moves_str) = input.split_once("\n\n").expect("Invalid input format");
  let warehouse = Warehouse::from_input_scaled(input);
  let moves = moves_str.replace('\n', "");
  (warehouse, moves)
}

fn solve_part1(input: &str) -> i32 {
  let (mut warehouse, moves) = parse_input(input);
  warehouse.execute_moves(&moves);
  warehouse.calculate_gps_sum()
}

fn solve_part2(input: &str) -> i32 {
  let (mut warehouse, moves) = parse_input_scaled(input);
  warehouse.execute_moves(&moves);
  warehouse.calculate_gps_sum()
}

fn main() {
  // Test with simple example
  let simple_input =
    fs::read_to_string("input/day15_simple.txt").expect("Failed to read simple input file");
  let simple_result = solve_part1(&simple_input);
  println!("Simple example result: {simple_result}");

  // Solve with full input
  let full_input =
    fs::read_to_string("input/day15_full.txt").expect("Failed to read full input file");
  let full_result = solve_part1(&full_input);
  println!("Part 1 result: {full_result}");

  // Part 2
  let simple_result_p2 = solve_part2(&simple_input);
  println!("Simple example Part 2 result: {simple_result_p2}");

  let full_result_p2 = solve_part2(&full_input);
  println!("Part 2 result: {full_result_p2}");
}
