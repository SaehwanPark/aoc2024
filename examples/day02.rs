use anyhow::Result;
use std::fs;

#[derive(Debug, Clone)]
struct Report {
  levels: Vec<i32>,
}

impl Report {
  fn new(levels: Vec<i32>) -> Self {
    Self { levels }
  }

  fn is_safe(&self) -> bool {
    Self::check_safety(&self.levels)
  }

  fn is_safe_with_dampener(&self) -> bool {
    // first check if it's already safe
    if self.is_safe() {
      return true;
    }

    for skip_index in 0..self.levels.len() {
      let modified_levels: Vec<i32> = self
        .levels
        .iter()
        .enumerate()
        .filter_map(|(i, &level)| if i != skip_index { Some(level) } else { None })
        .collect();

      if Self::check_safety(&modified_levels) {
        return true;
      }
    }

    false
  }

  fn check_safety(levels: &[i32]) -> bool {
    if levels.len() < 2 {
      return true;
    }

    let mut is_increasing: Option<bool> = None;

    for window in levels.windows(2) {
      let diff = window[1] - window[0];
      let abs_diff = diff.abs();

      // check if difference is within valid range
      if !(1..=3).contains(&abs_diff) {
        return false;
      }

      // determine direction of first comparison
      if is_increasing.is_none() {
        is_increasing = Some(diff > 0);
      } else {
        let current_increasing = diff > 0;
        if is_increasing.unwrap() != current_increasing {
          return false;
        }
      }
    }

    true
  }
}

fn parse_input(content: &str) -> Vec<Report> {
  content
    .lines()
    .filter(|line| !line.trim().is_empty())
    .map(|line| {
      let levels: Vec<i32> = line
        .split_whitespace()
        .map(|num| num.parse().expect("Invalid number in input"))
        .collect();
      Report::new(levels)
    })
    .collect()
}

fn count_safe_reports(reports: &[Report]) -> usize {
  reports.iter().filter(|report| report.is_safe()).count()
}

fn count_safe_reports_with_dampener(reports: &[Report]) -> usize {
  reports
    .iter()
    .filter(|report| report.is_safe_with_dampener())
    .count()
}

fn solve(input: &str, part: u8) -> usize {
  let reports = parse_input(input);
  match part {
    1 => count_safe_reports(&reports),
    2 => count_safe_reports_with_dampener(&reports),
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
  print_result("input/day02_simple.txt", "Simple puzzle")?;
  print_result("input/day02_full.txt", "Full puzzle")?;
  Ok(())
}
