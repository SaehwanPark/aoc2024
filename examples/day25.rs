use anyhow::Result;
use std::fs;

fn parse_input(content: &str) -> (Vec<Vec<usize>>, Vec<Vec<usize>>, usize) {
  let schematics: Vec<Vec<String>> = content
    .trim()
    .split("\n\n")
    .map(|s| s.lines().map(String::from).collect())
    .collect();

  let mut locks = Vec::new();
  let mut keys = Vec::new();
  let available_space = schematics[0].len() - 2; // total height - 2 (top and bottom fixed rows)

  for schematic in schematics {
    let is_lock = schematic[0].chars().all(|c| c == '#');
    let heights = schematic_to_heights(&schematic);

    if is_lock {
      locks.push(heights);
    } else {
      keys.push(heights);
    }
  }

  (locks, keys, available_space)
}

fn schematic_to_heights(schematic: &[String]) -> Vec<usize> {
  let rows = schematic.len();
  let cols = schematic[0].len();

  (0..cols)
    .map(|col| {
      // count # symbols in middle rows (excluding first and last row)
      (1..rows - 1)
        .filter(|&row| schematic[row].chars().nth(col).unwrap() == '#')
        .count()
    })
    .collect()
}

fn fits(lock: &[usize], key: &[usize], available_space: usize) -> bool {
  lock
    .iter()
    .zip(key.iter())
    .all(|(&l, &k)| l + k <= available_space)
}

/// no part 2 for day 25!
fn solve(input: &str) -> usize {
  let (locks, keys, available_space) = parse_input(input);
  locks
    .iter()
    .flat_map(|lock| keys.iter().map(move |key| (lock, key)))
    .filter(|(lock, key)| fits(lock, key, available_space))
    .count()
}

fn print_result(filepath: &str, puzzle_kind: &str) -> Result<()> {
  let input = fs::read_to_string(filepath)?;
  println!("Input: {puzzle_kind}");
  println!("Part 1 result = {}", solve(&input));
  Ok(())
}

fn main() -> Result<()> {
  print_result("input/day25_simple.txt", "Simple puzzle")?;
  print_result("input/day25_full.txt", "Full puzzle")?;
  Ok(())
}
