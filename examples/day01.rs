use std::collections::HashMap;
use std::fs;

fn parse_input(content: &str) -> Result<(Vec<i32>, Vec<i32>), Box<dyn std::error::Error>> {
  let mut left_list = Vec::new();
  let mut right_list = Vec::new();

  for line in content.lines() {
    let line = line.trim();
    if line.is_empty() {
      continue;
    }

    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() != 2 {
      return Err(format!("Invalid line format: {line}").into());
    }

    let left: i32 = parts[0].parse()?;
    let right: i32 = parts[1].parse()?;

    left_list.push(left);
    right_list.push(right);
  }

  Ok((left_list, right_list))
}

fn solve_part1(left_list: &[i32], right_list: &[i32]) -> i32 {
  let mut sorted_left = left_list.to_vec();
  let mut sorted_right = right_list.to_vec();

  // Sort both lists
  sorted_left.sort();
  sorted_right.sort();

  // Calculate total distance by pairing smallest with smallest, etc.
  sorted_left
    .iter()
    .zip(sorted_right.iter())
    .map(|(left, right)| (left - right).abs())
    .sum()
}

fn solve_part2(left_list: &[i32], right_list: &[i32]) -> i32 {
  // Count occurrences of each number in the right list
  let mut right_counts: HashMap<i32, i32> = HashMap::new();
  for &num in right_list {
    *right_counts.entry(num).or_insert(0) += 1;
  }

  // Calculate similarity score
  left_list
    .iter()
    .map(|&num| {
      let count = right_counts.get(&num).unwrap_or(&0);
      num * count
    })
    .sum()
}

fn solve_day1(file_path: &str) -> Result<(i32, i32), Box<dyn std::error::Error>> {
  let content = fs::read_to_string(file_path)?;
  let (left_list, right_list) = parse_input(&content)?;

  let part1 = solve_part1(&left_list, &right_list);
  let part2 = solve_part2(&left_list, &right_list);

  Ok((part1, part2))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Test with simple input first
  println!("Testing with simple input:");
  let simple_result = solve_day1("input/day01_simple.txt")?;
  println!(
    "Simple - Part 1: {}, Part 2: {}",
    simple_result.0, simple_result.1
  );

  // Solve with full input
  println!("\nSolving with full input:");
  let full_result = solve_day1("input/day01_full.txt")?;
  println!(
    "Full - Part 1: {}, Part 2: {}",
    full_result.0, full_result.1
  );

  Ok(())
}
