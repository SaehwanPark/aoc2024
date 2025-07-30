use anyhow::{Ok, Result};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
  Wall,
  Box,
  BoxLeft,  // left part of wide box
  BoxRight, // right part of wide box
  Robot,
  Empty,
}

impl Cell {
  fn from_char(c: char) -> Self {
    match c {
      '#' => Cell::Wall,
      'O' => Cell::Box,
      '@' => Cell::Robot,
      '.' => Cell::Empty,
      _ => panic!("invalid character in map: {c}"),
    }
  }

  fn to_char(self) -> char {
    match self {
      Cell::Wall => '#',
      Cell::Box => 'O',
      Cell::BoxLeft => '[',
      Cell::BoxRight => ']',
      Cell::Robot => '@',
      Cell::Empty => '.',
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
  row: i32,
  col: i32,
}

impl Position {
  const fn new(row: i32, col: i32) -> Self {
    Self { row, col }
  }

  fn move_in_direction(self, direction: Direction) -> Self {
    match direction {
      Direction::Up => Self::new(self.row - 1, self.col),
      Direction::Down => Self::new(self.row + 1, self.col),
      Direction::Left => Self::new(self.row, self.col - 1),
      Direction::Right => Self::new(self.row, self.col + 1),
    }
  }

  fn gps_coordinate(self) -> i32 {
    100 * self.row + self.col
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
  fn new(grid: HashMap<Position, Cell>, robot_pos: Position, width: i32, height: i32) -> Self {
    Self {
      grid,
      robot_pos,
      width,
      height,
    }
  }

  fn place_normal_cell(
    grid: &mut HashMap<Position, Cell>,
    robot_pos: &mut Position,
    row: i32,
    col: i32,
    ch: char,
  ) {
    let pos = Position::new(row, col);
    let cell = Cell::from_char(ch);

    if cell == Cell::Robot {
      *robot_pos = pos;
    }

    grid.insert(pos, cell);
  }

  fn place_scaled_cell(
    grid: &mut HashMap<Position, Cell>,
    robot_pos: &mut Position,
    row: i32,
    col: i32,
    ch: char,
  ) {
    let left_pos = Position::new(row, col * 2);
    let right_pos = Position::new(row, col * 2 + 1);

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
        *robot_pos = left_pos;
        grid.insert(left_pos, Cell::Robot);
        grid.insert(right_pos, Cell::Empty);
      }
      '.' => {
        grid.insert(left_pos, Cell::Empty);
        grid.insert(right_pos, Cell::Empty);
      }
      _ => panic!("Invalid character in map: {ch}"),
    }
  }

  fn parse_map(map_str: &str, scaled: bool) -> Self {
    let mut grid = HashMap::new();
    let mut robot_pos = Position::new(0, 0);
    let lines: Vec<&str> = map_str.lines().collect();
    let height = lines.len() as i32;
    let width = if scaled {
      lines.first().map_or(0, |l| l.len() * 2) as i32
    } else {
      lines.first().map_or(0, |l| l.len()) as i32
    };

    for (row, line) in lines.iter().enumerate() {
      for (col, ch) in line.chars().enumerate() {
        if scaled {
          Self::place_scaled_cell(&mut grid, &mut robot_pos, row as i32, col as i32, ch);
        } else {
          Self::place_normal_cell(&mut grid, &mut robot_pos, row as i32, col as i32, ch);
        }
      }
    }

    Self::new(grid, robot_pos, width, height)
  }

  fn from_input(input: &str) -> Self {
    let (map_str, _) = input.split_once("\n\n").expect("Invalid input format");
    Self::parse_map(map_str, false)
  }

  fn from_input_scaled(input: &str) -> Self {
    let (map_str, _) = input.split_once("\n\n").expect("Invalid input format");
    Self::parse_map(map_str, true)
  }

  fn get_cell(&self, pos: Position) -> Cell {
    *self.grid.get(&pos).unwrap_or(&Cell::Wall)
  }

  fn set_cell(&mut self, pos: Position, cell: Cell) {
    self.grid.insert(pos, cell);
  }

  fn try_push_simple_boxes(
    &self,
    start_pos: Position,
    direction: Direction,
  ) -> Option<Vec<Position>> {
    let mut positions_to_move = Vec::new();
    let mut current_pos = start_pos;

    loop {
      current_pos = current_pos.move_in_direction(direction);

      match self.get_cell(current_pos) {
        Cell::Wall => return None,
        Cell::Empty => break,
        Cell::Box => positions_to_move.push(current_pos),
        Cell::Robot => panic!("Unexpected robot position"),
        Cell::BoxLeft | Cell::BoxRight => return None, // use wide box logic instead
      }
    }

    Some(positions_to_move)
  }

  fn add_box_check_positions(
    to_check: &mut VecDeque<Position>,
    left_pos: Position,
    right_pos: Position,
    direction: Direction,
  ) {
    match direction {
      Direction::Up | Direction::Down => {
        // for vertical movement, both parts of the box move
        to_check.push_back(left_pos.move_in_direction(direction));
        to_check.push_back(right_pos.move_in_direction(direction));
      }
      Direction::Left => {
        // for left movement, only check left of the left part
        to_check.push_back(left_pos.move_in_direction(direction));
      }
      Direction::Right => {
        // for right movement, only check right of the right part
        to_check.push_back(right_pos.move_in_direction(direction));
      }
    }
  }

  fn try_push_wide_boxes(
    &self,
    start_pos: Position,
    direction: Direction,
  ) -> Option<Vec<Position>> {
    let mut to_check = VecDeque::new();
    let mut boxes_to_move = HashSet::new();

    to_check.push_back(start_pos.move_in_direction(direction));

    while let Some(pos) = to_check.pop_front() {
      match self.get_cell(pos) {
        Cell::Wall => return None,
        Cell::Empty => continue,
        Cell::BoxLeft => {
          let right_pos = Position::new(pos.row, pos.col + 1);
          if boxes_to_move.insert(pos) {
            Self::add_box_check_positions(&mut to_check, pos, right_pos, direction);
          }
          boxes_to_move.insert(right_pos);
        }
        Cell::BoxRight => {
          let left_pos = Position::new(pos.row, pos.col - 1);
          if boxes_to_move.insert(pos) {
            Self::add_box_check_positions(&mut to_check, left_pos, pos, direction);
          }
          boxes_to_move.insert(left_pos);
        }
        Cell::Box => {
          if boxes_to_move.insert(pos) {
            to_check.push_back(pos.move_in_direction(direction));
          }
        }
        Cell::Robot => panic!("Unexpected robot position."),
      }
    }

    Some(boxes_to_move.into_iter().collect())
  }

  fn execute_simple_box_push(&mut self, box_positions: &[Position], direction: Direction) {
    // move all boxes one positionin the direction (in reverse order)
    for &box_pos in box_positions.iter().rev() {
      let new_box_pos = box_pos.move_in_direction(direction);
      self.set_cell(box_pos, Cell::Empty);
      self.set_cell(new_box_pos, Cell::Box);
    }
  }

  fn execute_wide_box_push(&mut self, box_positions: &[Position], direction: Direction) {
    // save the current state of boxes to move
    let boxes_state: Vec<(Position, Cell)> = box_positions
      .iter()
      .map(|&p| (p, self.get_cell(p)))
      .collect();

    // clear all box positions first
    for &pos in box_positions {
      self.set_cell(pos, Cell::Empty);
    }

    // pace boxes in their new positions
    for (pos, cell) in boxes_state {
      let new_pos = pos.move_in_direction(direction);
      self.set_cell(new_pos, cell);
    }
  }

  fn move_robot_to(&mut self, new_pos: Position) {
    self.set_cell(self.robot_pos, Cell::Empty);
    self.set_cell(new_pos, Cell::Robot);
    self.robot_pos = new_pos;
  }

  fn try_move_robot(&mut self, direction: Direction) {
    let new_robot_pos = self.robot_pos.move_in_direction(direction);

    match self.get_cell(new_robot_pos) {
      Cell::Wall => return, // can't move into wall
      Cell::Empty => self.move_robot_to(new_robot_pos),
      Cell::Box => {
        if let Some(box_pos) = self.try_push_simple_boxes(self.robot_pos, direction) {
          self.execute_simple_box_push(&box_pos, direction);
          self.move_robot_to(new_robot_pos);
        }
      }
      Cell::BoxLeft | Cell::BoxRight => {
        if let Some(box_pos) = self.try_push_wide_boxes(self.robot_pos, direction) {
          self.execute_wide_box_push(&box_pos, direction);
          self.move_robot_to(new_robot_pos);
        }
      }
      Cell::Robot => panic!("Two robots found."),
    }
  }

  fn execute_moves(&mut self, moves: &str) {
    for ch in moves.chars() {
      if let Some(dir) = Direction::from_char(ch) {
        self.try_move_robot(dir);
      }
    }
  }

  fn calculate_gps_sum(&self) -> i32 {
    self
      .grid
      .iter()
      .filter_map(|(pos, &cell)| match cell {
        Cell::Box | Cell::BoxLeft => Some(pos.gps_coordinate()),
        _ => None,
      })
      .sum()
  }

  #[allow(dead_code)]
  fn print_warehouse(&self) {
    for row in 0..self.height {
      for col in 0..self.width {
        let pos = Position::new(row, col);
        print!("{}", self.get_cell(pos).to_char());
      }
      println!();
    }
    println!();
  }
}

fn parse_moves(input: &str) -> String {
  let (_, moves_str) = input.split_once("\n\n").expect("Invalid input format");
  moves_str.replace('\n', "")
}

fn solve(input: &str, part: u8) -> i32 {
  let mut warehouse = match part {
    1 => Warehouse::from_input(input),
    2 => Warehouse::from_input_scaled(input),
    _ => panic!("There are only parts 1 and 2."),
  };

  let moves = parse_moves(input);
  warehouse.execute_moves(&moves);
  warehouse.calculate_gps_sum()
}

fn print_result(filepath: &str, puzzle_kind: &str) -> Result<()> {
  let input = fs::read_to_string(filepath)?;
  println!("Input: {puzzle_kind}");
  println!("Part 1 result = {}", solve(&input, 1));
  println!("Part 2 result = {}\n", solve(&input, 2));
  Ok(())
}

fn main() -> Result<()> {
  print_result("input/day15_simple.txt", "Simple puzzle")?;
  print_result("input/day15_full.txt", "Full puzzle")?;
  Ok(())
}
