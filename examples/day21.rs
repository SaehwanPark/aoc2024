use std::collections::HashMap;

type Position = (i32, i32);

struct Keypad {
  buttons: HashMap<char, Position>,
  gap: Position,
}

impl Keypad {
  fn numeric() -> Self {
    let mut buttons = HashMap::new();
    buttons.insert('7', (0, 0));
    buttons.insert('8', (0, 1));
    buttons.insert('9', (0, 2));
    buttons.insert('4', (1, 0));
    buttons.insert('5', (1, 1));
    buttons.insert('6', (1, 2));
    buttons.insert('1', (2, 0));
    buttons.insert('2', (2, 1));
    buttons.insert('3', (2, 2));
    buttons.insert('0', (3, 1));
    buttons.insert('A', (3, 2));

    Self {
      buttons,
      gap: (3, 0),
    }
  }

  fn directional() -> Self {
    let mut buttons = HashMap::new();
    buttons.insert('^', (0, 1));
    buttons.insert('A', (0, 2));
    buttons.insert('<', (1, 0));
    buttons.insert('v', (1, 1));
    buttons.insert('>', (1, 2));

    Self {
      buttons,
      gap: (0, 0),
    }
  }

  fn get_paths(&self, from: char, to: char) -> Vec<String> {
    if from == to {
      return vec![String::new()];
    }

    let (r1, c1) = self.buttons[&from];
    let (r2, c2) = self.buttons[&to];

    let dr = r2 - r1;
    let dc = c2 - c1;

    let mut vertical = String::new();
    let mut horizontal = String::new();

    if dr > 0 {
      vertical = "v".repeat(dr as usize);
    } else if dr < 0 {
      vertical = "^".repeat((-dr) as usize);
    }

    if dc > 0 {
      horizontal = ">".repeat(dc as usize);
    } else if dc < 0 {
      horizontal = "<".repeat((-dc) as usize);
    }

    let mut paths = Vec::new();

    // Try vertical first, then horizontal
    if self.is_valid_path((r1, c1), (r2, c2), true) {
      paths.push(format!("{vertical}{horizontal}"));
    }

    // Try horizontal first, then vertical (avoid duplicates)
    if self.is_valid_path((r1, c1), (r2, c2), false)
      && !(vertical.is_empty() || horizontal.is_empty())
    {
      paths.push(format!("{horizontal}{vertical}"));
    }

    if paths.is_empty() {
      paths.push(format!("{vertical}{horizontal}"));
    }

    paths
  }

  fn is_valid_path(&self, from: Position, to: Position, vertical_first: bool) -> bool {
    let (r1, c1) = from;
    let (r2, c2) = to;

    if vertical_first {
      // Check intermediate position after vertical move
      (r2, c1) != self.gap
    } else {
      // Check intermediate position after horizontal move
      (r1, c2) != self.gap
    }
  }
}

fn min_sequence_length(
  sequence: &str,
  depth: usize,
  max_depth: usize,
  memo: &mut HashMap<(String, usize), usize>,
) -> usize {
  // Check memoization cache
  if let Some(&cached) = memo.get(&(sequence.to_string(), depth)) {
    return cached;
  }

  // Base case: at my level (depth 0), just return sequence length
  if depth == 0 {
    return sequence.len();
  }

  // Choose keypad based on depth
  // Numeric keypad is at the maximum depth, all others are directional
  let keypad = if depth == max_depth {
    Keypad::numeric()
  } else {
    Keypad::directional()
  };

  let mut current_button = 'A';
  let mut total_length = 0;

  for target_button in sequence.chars() {
    let possible_paths = keypad.get_paths(current_button, target_button);

    // Find minimum cost among all possible paths
    let min_cost = possible_paths
      .iter()
      .map(|path| {
        let full_sequence = format!("{path}A"); // Add 'A' to press the button
        min_sequence_length(&full_sequence, depth - 1, max_depth, memo)
      })
      .min()
      .unwrap_or(0);

    total_length += min_cost;
    current_button = target_button;
  }

  // Cache the result
  memo.insert((sequence.to_string(), depth), total_length);
  total_length
}

fn solve_part1(codes: &[String]) -> usize {
  solve_with_depth(codes, 3)
}

fn solve_part2(codes: &[String]) -> usize {
  solve_with_depth(codes, 26)
}

fn solve_with_depth(codes: &[String], depth: usize) -> usize {
  let mut memo = HashMap::new();
  let mut total_complexity = 0;

  for code in codes {
    let sequence_length = min_sequence_length(code, depth, depth, &mut memo);

    let numeric_part: usize = code
      .chars()
      .filter(|c| c.is_ascii_digit())
      .collect::<String>()
      .parse()
      .unwrap_or(0);

    let complexity = sequence_length * numeric_part;
    total_complexity += complexity;

    println!(
      "Code: {code}, Length: {sequence_length}, Numeric: {numeric_part}, Complexity: {complexity}",
    );
  }

  total_complexity
}

fn main() {
  // Test with simple example
  let simple_input =
    std::fs::read_to_string("input/day21_simple.txt").expect("Failed to read simple input file");
  let simple_codes: Vec<String> = simple_input.lines().map(|s| s.to_string()).collect();

  println!("=== Simple Example ===");
  println!("Part 1 (Simple): {}", solve_part1(&simple_codes));
  println!("Part 2 (Simple): {}", solve_part2(&simple_codes));

  // Test with full input
  if let Ok(full_input) = std::fs::read_to_string("input/day21_full.txt") {
    let full_codes: Vec<String> = full_input.lines().map(|s| s.to_string()).collect();

    println!("\n=== Full Input ===");
    println!("Part 1 (Full): {}", solve_part1(&full_codes));
    println!("Part 2 (Full): {}", solve_part2(&full_codes));
  } else {
    println!("\nFull input file not found, skipping...");
  }
}
