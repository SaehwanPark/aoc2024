use anyhow::Result;
use std::collections::HashMap;
use std::fs;

/// Parses puzzle input and returns left and right lists separately
fn parse_input(content: &str) -> Result<(Vec<i32>, Vec<i32>)> {
  let mut left_list = Vec::new();
  let mut right_list = Vec::new();

  for line in content.lines() {
    let line = line.trim();
    if line.is_empty() {
      continue;
    }

    let parts: Vec<&str> = line.split_whitespace().collect();

    let left: i32 = parts[0].parse()?;
    let right: i32 = parts[1].parse()?;

    left_list.push(left);
    right_list.push(right);
  }

  Ok((left_list, right_list))
}

/// Calculates and returns total distance as instructed
/// Instruction: sort the two lists respectively, generate pairwise distances, sum up them
fn calculate_total_distance(left_list: &[i32], right_list: &[i32]) -> i32 {
  let mut sorted_left = left_list.to_vec();
  let mut sorted_right = right_list.to_vec();

  // Sort both lists
  sorted_left.sort();
  sorted_right.sort();

  sorted_left
    .iter()
    .zip(sorted_right.iter())
    .map(|(left, right)| (left - right).abs())
    .sum()
}

/// Calculate total similarity score
/// where similarity is defined as
/// how many times one element in the left list shows up in the right list.
fn calculate_similarity_score(left_list: &[i32], right_list: &[i32]) -> i32 {
  // Count occurrences of each number in the right list
  let mut right_counts: HashMap<i32, i32> = HashMap::new();
  for &num in right_list {
    *right_counts.entry(num).or_insert(0) += 1;
  }

  left_list
    .iter()
    .map(|&num| {
      let count = right_counts.get(&num).unwrap_or(&0);
      num * count
    })
    .sum()
}

fn solve(input: &str, part: u8) -> i32 {
  let (left_list, right_list) = parse_input(input).expect("Can't parse input.");
  match part {
    1 => calculate_total_distance(&left_list, &right_list),
    2 => calculate_similarity_score(&left_list, &right_list),
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
  print_result("input/day01_simple.txt", "Simple puzzle")?;
  print_result("input/day01_full.txt", "Full puzzle")?;
  Ok(())
}
