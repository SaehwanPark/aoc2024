use regex::Regex;
use std::fs;

#[derive(Debug, Clone, Copy)]
struct Robot {
  position: (i32, i32),
  velocity: (i32, i32),
}

impl Robot {
  fn new(px: i32, py: i32, vx: i32, vy: i32) -> Self {
    Self {
      position: (px, py),
      velocity: (vx, vy),
    }
  }

  fn move_after_seconds(&self, seconds: i32, width: i32, height: i32) -> (i32, i32) {
    let new_x = (self.position.0 + self.velocity.0 * seconds).rem_euclid(width);
    let new_y = (self.position.1 + self.velocity.1 * seconds).rem_euclid(height);
    (new_x, new_y)
  }
}

fn parse_robots(input: &str) -> Vec<Robot> {
  let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();

  input
    .lines()
    .filter_map(|line| {
      re.captures(line.trim()).map(|caps| {
        let px = caps[1].parse::<i32>().unwrap();
        let py = caps[2].parse::<i32>().unwrap();
        let vx = caps[3].parse::<i32>().unwrap();
        let vy = caps[4].parse::<i32>().unwrap();
        Robot::new(px, py, vx, vy)
      })
    })
    .collect()
}

fn calculate_safety_factor(robots: &[Robot], width: i32, height: i32, seconds: i32) -> usize {
  let mid_x = width / 2;
  let mid_y = height / 2;

  let mut quadrants = [0; 4]; // [top_left, top_right, bottom_left, bottom_right]

  for robot in robots {
    let (x, y) = robot.move_after_seconds(seconds, width, height);

    // Skip robots exactly in the middle
    if x == mid_x || y == mid_y {
      continue;
    }

    match (x < mid_x, y < mid_y) {
      (true, true) => quadrants[0] += 1,   // top_left
      (false, true) => quadrants[1] += 1,  // top_right
      (true, false) => quadrants[2] += 1,  // bottom_left
      (false, false) => quadrants[3] += 1, // bottom_right
    }
  }

  quadrants.iter().product()
}

fn solve_part1(input: &str, width: i32, height: i32) -> usize {
  let robots = parse_robots(input);
  calculate_safety_factor(&robots, width, height, 100)
}

fn calculate_position_variance(robots: &[Robot], width: i32, height: i32, seconds: i32) -> f64 {
  let positions: Vec<(i32, i32)> = robots
    .iter()
    .map(|robot| robot.move_after_seconds(seconds, width, height))
    .collect();

  if positions.is_empty() {
    return f64::INFINITY;
  }

  let n = positions.len() as f64;
  let mean_x = positions.iter().map(|(x, _)| *x as f64).sum::<f64>() / n;
  let mean_y = positions.iter().map(|(_, y)| *y as f64).sum::<f64>() / n;

  let variance_x = positions
    .iter()
    .map(|(x, _)| (*x as f64 - mean_x).powi(2))
    .sum::<f64>()
    / n;

  let variance_y = positions
    .iter()
    .map(|(_, y)| (*y as f64 - mean_y).powi(2))
    .sum::<f64>()
    / n;

  variance_x + variance_y
}

fn visualize_robots(robots: &[Robot], width: i32, height: i32, seconds: i32) -> String {
  let positions: std::collections::HashSet<(i32, i32)> = robots
    .iter()
    .map(|robot| robot.move_after_seconds(seconds, width, height))
    .collect();

  let mut grid = String::new();
  for y in 0..height {
    for x in 0..width {
      if positions.contains(&(x, y)) {
        grid.push('#');
      } else {
        grid.push('.');
      }
    }
    grid.push('\n');
  }
  grid
}

fn solve_part2(input: &str, width: i32, height: i32) -> i32 {
  let robots = parse_robots(input);

  // The pattern repeats every width * height seconds due to the modular arithmetic
  let max_seconds = width * height;

  let mut min_variance = f64::INFINITY;
  let mut best_seconds = 0;

  for seconds in 0..max_seconds {
    let variance = calculate_position_variance(&robots, width, height, seconds);

    if variance < min_variance {
      min_variance = variance;
      best_seconds = seconds;
    }
  }

  best_seconds
}

fn main() {
  // Test with simple example
  let simple_input =
    fs::read_to_string("input/day14_simple.txt").expect("Failed to read simple input file");
  let simple_result = solve_part1(&simple_input, 11, 7);
  println!("Simple example result: {simple_result}");
  assert_eq!(simple_result, 12);

  // Solve with full input
  let full_input =
    fs::read_to_string("input/day14_full.txt").expect("Failed to read full input file");
  let full_result = solve_part1(&full_input, 101, 103);
  println!("Part 1 result: {full_result}");

  // Part 2: Find the Christmas tree
  let part2_result = solve_part2(&full_input, 101, 103);
  println!("Part 2 result: {part2_result}");

  // Visualize the Christmas tree
  let robots = parse_robots(&full_input);
  let tree_visualization = visualize_robots(&robots, 101, 103, part2_result);
  println!("Christmas tree at {part2_result} seconds:");

  println!("{tree_visualization}");
}
