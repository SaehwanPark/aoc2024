use std::collections::HashMap;
use std::fs;

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

fn solve_day1(file_path: &str) -> Result<(i32, i32), Box<dyn std::error::Error>> {
  let content = fs::read_to_string(file_path)?;
  let (left_list, right_list) = parse_input(&content)?;

  let part1 = solve_part1(&left_list, &right_list);
  let part2 = solve_part2(&left_list, &right_list);

  Ok((part1, part2))
}

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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example() {
    let example_input = "3   4
4   3
2   5
1   3
3   9
3   3";

    let (left_list, right_list) = parse_input(example_input).unwrap();

    // Test Part 1
    let part1_result = solve_part1(&left_list, &right_list);
    assert_eq!(part1_result, 11);

    // Test Part 2
    let part2_result = solve_part2(&left_list, &right_list);
    assert_eq!(part2_result, 31);
  }

  #[test]
  fn test_parse_input() {
    let input = "3   4\n4   3\n2   5";
    let (left, right) = parse_input(input).unwrap();

    assert_eq!(left, vec![3, 4, 2]);
    assert_eq!(right, vec![4, 3, 5]);
  }

  #[test]
  fn test_part1_sorting() {
    let left = vec![3, 4, 2, 1, 3, 3];
    let right = vec![4, 3, 5, 3, 9, 3];

    let result = solve_part1(&left, &right);
    // After sorting: left=[1,2,3,3,3,4], right=[3,3,3,4,5,9]
    // Distances: |1-3|=2, |2-3|=1, |3-3|=0, |3-4|=1, |3-5|=2, |4-9|=5
    // Total: 2+1+0+1+2+5 = 11
    assert_eq!(result, 11);
  }

  #[test]
  fn test_part2_similarity() {
    let left = vec![3, 4, 2, 1, 3, 3];
    let right = vec![4, 3, 5, 3, 9, 3];

    let result = solve_part2(&left, &right);
    // 3 appears 3 times: 3*3=9
    // 4 appears 1 time: 4*1=4
    // 2 appears 0 times: 2*0=0
    // 1 appears 0 times: 1*0=0
    // 3 appears 3 times: 3*3=9
    // 3 appears 3 times: 3*3=9
    // Total: 9+4+0+0+9+9 = 31
    assert_eq!(result, 31);
  }
}
