use std::collections::HashMap;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Test with simple input first
  let simple_input = fs::read_to_string("input/day11_simple.txt")?;
  let simple_result_p1 = solve_part1(&simple_input, 25);
  println!("Simple input (25 blinks): {simple_result_p1}");

  let simple_result_p2 = solve_part2(&simple_input, 75);
  println!("Simple input (75 blinks): {simple_result_p2}");

  // Solve with full input
  let full_input = fs::read_to_string("input/day11_full.txt")?;
  let full_result_p1 = solve_part1(&full_input, 25);
  println!("Full input (25 blinks): {full_result_p1}");

  let full_result_p2 = solve_part2(&full_input, 75);
  println!("Full input (75 blinks): {full_result_p2}");

  Ok(())
}

fn solve_part1(input: &str, blinks: usize) -> u64 {
  // For consistency, use the optimized approach for part 1 too
  solve_part2(input, blinks)
}

fn solve_part2(input: &str, blinks: usize) -> u64 {
  let stones = parse_input(input);
  let mut memo = HashMap::new();

  stones
    .iter()
    .map(|&stone| count_stones_after_blinks(stone, blinks, &mut memo))
    .sum()
}

fn count_stones_after_blinks(
  stone: u64,
  blinks_remaining: usize,
  memo: &mut HashMap<(u64, usize), u64>,
) -> u64 {
  // Base case: no more blinks
  if blinks_remaining == 0 {
    return 1;
  }

  // Check memoization cache
  let key = (stone, blinks_remaining);
  if let Some(&result) = memo.get(&key) {
    return result;
  }

  // Calculate result based on transformation rules
  let result = if stone == 0 {
    // Rule 1: 0 becomes 1
    count_stones_after_blinks(1, blinks_remaining - 1, memo)
  } else {
    let digit_count = count_digits(stone);
    if digit_count % 2 == 0 {
      // Rule 2: Split even-digit numbers
      let (left, right) = split_number(stone, digit_count);
      count_stones_after_blinks(left, blinks_remaining - 1, memo)
        + count_stones_after_blinks(right, blinks_remaining - 1, memo)
    } else {
      // Rule 3: Multiply by 2024
      count_stones_after_blinks(stone * 2024, blinks_remaining - 1, memo)
    }
  };

  // Store in cache and return
  memo.insert(key, result);
  result
}

fn parse_input(input: &str) -> Vec<u64> {
  input
    .split_whitespace()
    .map(|s| s.parse().unwrap())
    .collect()
}

fn count_digits(mut num: u64) -> u32 {
  if num == 0 {
    return 1;
  }

  let mut count = 0;
  while num > 0 {
    num /= 10;
    count += 1;
  }
  count
}

fn split_number(num: u64, digit_count: u32) -> (u64, u64) {
  let half_digits = digit_count / 2;
  let divisor = 10_u64.pow(half_digits);

  let left = num / divisor;
  let right = num % divisor;

  (left, right)
}
