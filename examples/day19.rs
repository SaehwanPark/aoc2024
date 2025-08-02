use anyhow::Result;
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

fn count_possible_designs(designs: &[&str], patterns: &HashSet<String>) -> usize {
  let mut count = 0;
  for design in designs {
    let mut memo = HashMap::new();
    if can_form_design(design, patterns, &mut memo) {
      count += 1;
    }
  }

  count
}

fn count_possible_constructions_for_designs(designs: &[&str], patterns: &HashSet<String>) -> usize {
  let mut total_ways = 0;
  for design in designs {
    let mut memo = HashMap::new();
    total_ways += count_ways(design, patterns, &mut memo);
  }

  total_ways
}

fn solve(input: &str, part: u8) -> usize {
  let lines: Vec<&str> = input.trim().split('\n').collect();
  let patterns: HashSet<String> = lines[0].split(", ").map(|s| s.to_string()).collect();
  let designs: Vec<&str> = lines[2..].to_vec();

  match part {
    1 => count_possible_designs(&designs, &patterns),
    2 => count_possible_constructions_for_designs(&designs, &patterns),
    _ => panic!("Only part 1 or 2 is possible."),
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
  print_result("input/day19_simple.txt", "Simple puzzle")?;
  print_result("input/day19_full.txt", "Full puzzle")?;
  Ok(())
}
