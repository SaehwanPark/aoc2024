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

fn solve_part1(locks: &[Vec<usize>], keys: &[Vec<usize>], available_space: usize) -> usize {
  locks
    .iter()
    .flat_map(|lock| keys.iter().map(move |key| (lock, key)))
    .filter(|(lock, key)| fits(lock, key, available_space))
    .count()
}

fn main() {
  // test with simple input
  let content = fs::read_to_string("input/day25_simple.txt").expect("failed to read input");
  let (locks, keys, available_space) = parse_input(&content);
  let result = solve_part1(&locks, &keys, available_space);
  println!("Part 1 (simple): {result}");

  // solve with full input
  let content = fs::read_to_string("input/day25_full.txt").expect("failed to read input");
  let (locks, keys, available_space) = parse_input(&content);
  let result = solve_part1(&locks, &keys, available_space);
  println!("Part 1 (full): {result}");
}
