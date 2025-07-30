use std::collections::{HashMap, HashSet};
use std::fs;

fn can_form_design(
  design: &str,
  patterns: &HashSet<String>,
  memo: &mut HashMap<String, bool>,
) -> bool {
  if design.is_empty() {
    return true;
  }

  if let Some(&result) = memo.get(design) {
    return result;
  }

  for pattern in patterns {
    if design.starts_with(pattern) {
      let remaining = &design[pattern.len()..];
      if can_form_design(remaining, patterns, memo) {
        memo.insert(design.to_string(), true);
        return true;
      }
    }
  }

  memo.insert(design.to_string(), false);
  false
}

fn count_ways(
  design: &str,
  patterns: &HashSet<String>,
  memo: &mut HashMap<String, usize>,
) -> usize {
  if design.is_empty() {
    return 1; // One way to form empty string
  }

  if let Some(&result) = memo.get(design) {
    return result;
  }

  let mut total_ways = 0;
  for pattern in patterns {
    if design.starts_with(pattern) {
      let remaining = &design[pattern.len()..];
      total_ways += count_ways(remaining, patterns, memo);
    }
  }

  memo.insert(design.to_string(), total_ways);
  total_ways
}

fn solve_part1(input: &str) -> usize {
  let lines: Vec<&str> = input.trim().split('\n').collect();

  // Parse patterns from first line
  let patterns: HashSet<String> = lines[0].split(", ").map(|s| s.to_string()).collect();

  // Parse designs (skip first line and empty line)
  let designs: Vec<&str> = lines[2..].to_vec();

  let mut count = 0;
  for design in designs {
    let mut memo = HashMap::new();
    if can_form_design(design, &patterns, &mut memo) {
      count += 1;
    }
  }

  count
}

fn solve_part2(input: &str) -> usize {
  let lines: Vec<&str> = input.trim().split('\n').collect();

  // Parse patterns from first line
  let patterns: HashSet<String> = lines[0].split(", ").map(|s| s.to_string()).collect();

  // Parse designs (skip first line and empty line)
  let designs: Vec<&str> = lines[2..].to_vec();

  let mut total_ways = 0;
  for design in designs {
    let mut memo = HashMap::new();
    total_ways += count_ways(design, &patterns, &mut memo);
  }

  total_ways
}

fn main() {
  let input_simple =
    fs::read_to_string("input/day19_simple.txt").expect("Failed to read simple input");
  let input_full = fs::read_to_string("input/day19_full.txt").expect("Failed to read full input");

  println!("Part 1 (simple): {}", solve_part1(&input_simple));
  println!("Part 1 (full): {}", solve_part1(&input_full));

  println!("Part 2 (simple): {}", solve_part2(&input_simple));
  println!("Part 2 (full): {}", solve_part2(&input_full));
}
