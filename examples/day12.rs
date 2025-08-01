use anyhow::Result;
use std::collections::{HashSet, VecDeque};
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
  row: usize,
  col: usize,
}

impl Point {
  const fn new(row: usize, col: usize) -> Self {
    Self { row, col }
  }

  fn neighbors(self, rows: usize, cols: usize) -> impl Iterator<Item = Point> {
    const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    DIRECTIONS.into_iter().filter_map(move |(dr, dc)| {
      let new_row = self.row.wrapping_add_signed(dr);
      let new_col = self.col.wrapping_add_signed(dc);

      if new_row < rows && new_col < cols {
        Some(Point::new(new_row, new_col))
      } else {
        None
      }
    })
  }
}

#[derive(Debug)]
struct Region {
  cells: HashSet<Point>,
  area: usize,
  perimeter: usize,
  sides: usize,
}

impl Region {
  fn new() -> Self {
    Self {
      cells: HashSet::new(),
      area: 0,
      perimeter: 0,
      sides: 0,
    }
  }

  fn calculate_perimeter(&mut self, grid: &[Vec<char>]) {
    let rows = grid.len();
    let cols = grid[0].len();

    self.perimeter = self
      .cells
      .iter()
      .map(|&point| {
        4 - point
          .neighbors(rows, cols)
          .filter(|&neighbor| self.cells.contains(&neighbor))
          .count()
      })
      .sum();
  }

  fn calculate_sides(&mut self, grid: &[Vec<char>]) {
    let rows = grid.len();
    let cols = grid[0].len();

    // For each cell, count corners
    // A corner exists when:
    // 1. Two adjacent neighbors are different from current cell
    // 2. Or when diagonal neighbor is different but both adjacent neighbors are same

    self.sides = self
      .cells
      .iter()
      .map(|&point| self.count_corners(point, rows, cols))
      .sum();
  }

  fn count_corners(&self, point: Point, rows: usize, cols: usize) -> usize {
    let row = point.row as isize;
    let col = point.col as isize;

    // Check all 4 corners of this cell
    let corner_checks = [
      // Top-left corner: check top, left, and top-left diagonal
      ((-1, 0), (0, -1), (-1, -1)),
      // Top-right corner: check top, right, and top-right diagonal
      ((-1, 0), (0, 1), (-1, 1)),
      // Bottom-left corner: check bottom, left, and bottom-left diagonal
      ((1, 0), (0, -1), (1, -1)),
      // Bottom-right corner: check bottom, right, and bottom-right diagonal
      ((1, 0), (0, 1), (1, 1)),
    ];

    corner_checks
      .iter()
      .filter(|&&(side1, side2, diag)| {
        let side1_same = self.is_same_region(row + side1.0, col + side1.1, rows, cols);
        let side2_same = self.is_same_region(row + side2.0, col + side2.1, rows, cols);
        let diag_same = self.is_same_region(row + diag.0, col + diag.1, rows, cols);

        // Corner exists if:
        // 1. Both adjacent sides are different (external corner)
        // 2. Both adjacent sides are same but diagonal is different (internal corner)
        (!side1_same && !side2_same) || (side1_same && side2_same && !diag_same)
      })
      .count()
  }

  fn is_same_region(&self, row: isize, col: isize, rows: usize, cols: usize) -> bool {
    if row < 0 || col < 0 || row >= rows as isize || col >= cols as isize {
      false
    } else {
      self.cells.contains(&Point::new(row as usize, col as usize))
    }
  }

  fn multiply_area_by_perimeter(&self) -> usize {
    self.area * self.perimeter
  }

  fn multiply_area_by_sides(&self) -> usize {
    self.area * self.sides
  }
}

struct GardenMap {
  grid: Vec<Vec<char>>,
  regions: Vec<Region>,
}

impl GardenMap {
  fn new(input: &str) -> Self {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut garden = Self {
      grid,
      regions: Vec::new(),
    };

    garden.find_regions();
    garden
  }

  fn find_regions(&mut self) {
    let rows = self.grid.len();
    let cols = self.grid[0].len();
    let mut visited = vec![vec![false; cols]; rows];

    for row in 0..rows {
      for col in 0..cols {
        if !visited[row][col] {
          let start_point = Point::new(row, col);
          let plant_type = self.grid[row][col];

          let mut region = Region::new();
          self.flood_fill(start_point, plant_type, &mut visited, &mut region);

          region.area = region.cells.len();
          region.calculate_perimeter(&self.grid);
          region.calculate_sides(&self.grid);

          self.regions.push(region);
        }
      }
    }
  }

  fn flood_fill(
    &self,
    start: Point,
    plant_type: char,
    visited: &mut [Vec<bool>],
    region: &mut Region,
  ) {
    let rows = self.grid.len();
    let cols = self.grid[0].len();
    let mut queue = VecDeque::new();

    queue.push_back(start);
    visited[start.row][start.col] = true;
    region.cells.insert(start);

    while let Some(current) = queue.pop_front() {
      for neighbor in current.neighbors(rows, cols) {
        if !visited[neighbor.row][neighbor.col]
          && self.grid[neighbor.row][neighbor.col] == plant_type
        {
          visited[neighbor.row][neighbor.col] = true;
          region.cells.insert(neighbor);
          queue.push_back(neighbor);
        }
      }
    }
  }

  fn calculate_total_price(&self) -> usize {
    self
      .regions
      .iter()
      .map(|region| region.multiply_area_by_perimeter())
      .sum()
  }

  fn calculate_total_price_under_bulk_discount(&self) -> usize {
    self
      .regions
      .iter()
      .map(|region| region.multiply_area_by_sides())
      .sum()
  }
}

fn solve(input: &str, part: u8) -> usize {
  let garden = GardenMap::new(input);
  match part {
    1 => garden.calculate_total_price(),
    2 => garden.calculate_total_price_under_bulk_discount(),
    _ => panic!("Only part 1 or 2 is available."),
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
  print_result("input/day12_simple.txt", "Simple puzzle")?;
  print_result("input/day12_full.txt", "Full puzzle")?;
  Ok(())
}
