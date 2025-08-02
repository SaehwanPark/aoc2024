use anyhow::Result;
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

#[allow(dead_code)]
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

fn minimize_robot_time_to_display_easter_egg(robots: &[Robot], width: i32, height: i32) -> usize {
  // The pattern repeats every width * height seconds due to the modular arithmetic
  let max_seconds = width * height;

  let mut min_variance = f64::INFINITY;
  let mut best_seconds = 0;

  for seconds in 0..max_seconds {
    let variance = calculate_position_variance(robots, width, height, seconds);

    if variance < min_variance {
      min_variance = variance;
      best_seconds = seconds;
    }
  }

  best_seconds as usize
}

fn solve(input: &str, width: i32, height: i32, part: u8) -> usize {
  let robots = parse_robots(input);

  match part {
    1 => calculate_safety_factor(&robots, width, height, 100),
    2 => minimize_robot_time_to_display_easter_egg(&robots, width, height),
    _ => panic!("Only part 1 or 2 is possible."),
  }
}

fn print_result(filepath: &str, puzzle_kind: &str) -> Result<()> {
  let input = fs::read_to_string(filepath)?;
  let (width, height) = match puzzle_kind {
    "Simple puzzle" => (11, 7),
    "Full puzzle" => (101, 103),
    _ => panic!("Neither simple nor full puzzle."),
  };
  println!("Input: {puzzle_kind}");
  println!("Part 1 result = {}", solve(&input, width, height, 1));
  println!("Part 2 result = {}\n", solve(&input, width, height, 2));
  Ok(())
}

fn main() -> Result<()> {
  print_result("input/day14_simple.txt", "Simple puzzle")?;
  print_result("input/day14_full.txt", "Full puzzle")?;
  Ok(())
}
