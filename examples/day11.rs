use anyhow::Result;
use std::collections::HashMap;
use std::fs;

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

/**
 * splits a number with even digit count into two halves
 */
fn split_number(num: u64, digit_count: u32) -> (u64, u64) {
  let half_digits = digit_count / 2;
  let divisor = 10_u64.pow(half_digits);

  (num / divisor, num % divisor)
}

/**
 * recursively counts stones after given number of blinks with memoization
 */
fn count_stones_after_blinks(
  stone: u64,
  blinks_remaining: usize,
  memo: &mut HashMap<(u64, usize), u64>,
) -> u64 {
  // base case: no more blinks
  if blinks_remaining == 0 {
    return 1;
  }

  // check memoizaiton cache
  let key = (stone, blinks_remaining);
  if let Some(&result) = memo.get(&key) {
    return result;
  }

  // calculate result based on transformation rules
  let result = if stone == 0 {
    // rule 1: 0 becomes 1
    count_stones_after_blinks(1, blinks_remaining - 1, memo)
  } else {
    let digit_count = count_digits(stone);
    if digit_count % 2 == 0 {
      // rule 2: split even-digit numbers
      let (left, right) = split_number(stone, digit_count);
      count_stones_after_blinks(left, blinks_remaining - 1, memo)
        + count_stones_after_blinks(right, blinks_remaining - 1, memo)
    } else {
      // rule 3: multiply by 2024
      count_stones_after_blinks(stone * 2024, blinks_remaining - 1, memo)
    }
  };

  // store in cache and return
  memo.insert(key, result);
  result
}

/**
 * solves the stone transformation problem for given number of blinks
 */
fn solve_stone_problem(input: &str, blinks: usize) -> u64 {
  let stones = parse_input(input);
  let mut memo = HashMap::new();

  stones
    .iter()
    .map(|&s| count_stones_after_blinks(s, blinks, &mut memo))
    .sum()
}

fn solve_problem(filepath: &str, kind: &str) -> Result<()> {
  let input = fs::read_to_string(filepath)?;

  println!("{kind}:");
  println!(
    "Part 1 results (25 blinks) = {}",
    solve_stone_problem(&input, 25)
  );
  println!(
    "Part 2 results (75 blinks) = {}",
    solve_stone_problem(&input, 75)
  );

  Ok(())
}

fn main() -> Result<()> {
  solve_problem("input/day11_simple.txt", "Simple puzzle input")?;
  solve_problem("input/day11_full.txt", "Full puzzle input")?;
  Ok(())
}
