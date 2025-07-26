use std::fs;
use std::path::Path;

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

fn read_input_file<P: AsRef<Path>>(file_path: P) -> Result<String, Box<dyn std::error::Error>> {
  let content = fs::read_to_string(file_path)?;
  Ok(content)
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
  // test with simple example first
  let simple_content = read_input_file("input/day02_simple.txt")?;
  let simple_reports = parse_input(&simple_content);
  let simple_safe_count = count_safe_reports(&simple_reports);
  let simple_safe_count_with_dampener = count_safe_reports_with_dampener(&simple_reports);

  println!(
    "Parsed {} reports from simple input, {} safe reports, {} w/ dampener",
    simple_reports.len(),
    simple_safe_count,
    simple_safe_count_with_dampener
  );

  let full_content = read_input_file("input/day02_full.txt")?;
  let full_reports = parse_input(&full_content);
  let full_safe_count = count_safe_reports(&full_reports);
  let full_safe_count_with_dampener = count_safe_reports_with_dampener(&full_reports);

  println!(
    "Parsed {} reports from full input, {} safe reports, {} w/ dampener",
    full_reports.len(),
    full_safe_count,
    full_safe_count_with_dampener
  );

  Ok(())
}
