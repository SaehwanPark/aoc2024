use anyhow::Result;
use std::fs;

#[derive(Debug, Clone, Copy)]
struct ClawMachine {
  button_a: (i64, i64), // (dx, dy)
  button_b: (i64, i64), // (dx, dy)
  prize: (i64, i64),    // (x, y)
}

impl ClawMachine {
  fn solve(&self, max_presses: Option<i64>) -> Option<i64> {
    let (ax, ay) = self.button_a;
    let (bx, by) = self.button_b;
    let (px, py) = self.prize;

    // System of equations:
    // a * ax + b * bx = px
    // a * ay + b * by = py
    //
    // Using Cramer's rule:
    // determinant = ax * by - ay * bx
    // a = (px * by - py * bx) / determinant
    // b = (ax * py - ay * px) / determinant

    let determinant = ax * by - ay * bx;
    if determinant == 0 {
      return None; // No unique solution
    }

    let numerator_a = px * by - py * bx;
    let numerator_b = ax * py - ay * px;

    // Check if solutions are integers
    if numerator_a % determinant != 0 || numerator_b % determinant != 0 {
      return None;
    }

    let a = numerator_a / determinant;
    let b = numerator_b / determinant;

    // Check non-negativity
    if a < 0 || b < 0 {
      return None;
    }

    // Check max presses constraint if specified
    if let Some(max) = max_presses {
      if a > max || b > max {
        return None;
      }
    }

    // Verify solution (double-check)
    if a * ax + b * bx == px && a * ay + b * by == py {
      Some(3 * a + b) // Cost: 3 tokens per A press, 1 per B press
    } else {
      None
    }
  }
}

fn parse_input(input: &str) -> Vec<ClawMachine> {
  let mut machines = Vec::new();
  let lines: Vec<&str> = input.trim().lines().collect();

  let mut i = 0;
  while i < lines.len() {
    if lines[i].trim().is_empty() {
      i += 1;
      continue;
    }

    // Parse Button A line: "Button A: X+94, Y+34"
    let button_a_line = lines[i];
    let button_a_parts: Vec<&str> = button_a_line
      .strip_prefix("Button A: ")
      .unwrap()
      .split(", ")
      .collect();
    let ax: i64 = button_a_parts[0]
      .strip_prefix("X+")
      .unwrap()
      .parse()
      .unwrap();
    let ay: i64 = button_a_parts[1]
      .strip_prefix("Y+")
      .unwrap()
      .parse()
      .unwrap();

    // Parse Button B line: "Button B: X+22, Y+67"
    let button_b_line = lines[i + 1];
    let button_b_parts: Vec<&str> = button_b_line
      .strip_prefix("Button B: ")
      .unwrap()
      .split(", ")
      .collect();
    let bx: i64 = button_b_parts[0]
      .strip_prefix("X+")
      .unwrap()
      .parse()
      .unwrap();
    let by: i64 = button_b_parts[1]
      .strip_prefix("Y+")
      .unwrap()
      .parse()
      .unwrap();

    // Parse Prize line: "Prize: X=8400, Y=5400"
    let prize_line = lines[i + 2];
    let prize_parts: Vec<&str> = prize_line
      .strip_prefix("Prize: ")
      .unwrap()
      .split(", ")
      .collect();
    let px: i64 = prize_parts[0].strip_prefix("X=").unwrap().parse().unwrap();
    let py: i64 = prize_parts[1].strip_prefix("Y=").unwrap().parse().unwrap();

    machines.push(ClawMachine {
      button_a: (ax, ay),
      button_b: (bx, by),
      prize: (px, py),
    });

    i += 3;
  }

  machines
}

fn minimize_tokens_to_win_prizes(machines: &[ClawMachine]) -> i64 {
  machines
    .iter()
    .filter_map(|machine| machine.solve(Some(100)))
    .sum()
}

fn minimize_tokens_to_win_prizes_with_modified_positions(machines: &[ClawMachine]) -> i64 {
  // Part 2: Add 10000000000000 to prize coordinates and no button press limit
  machines
    .iter()
    .map(|machine| ClawMachine {
      button_a: machine.button_a,
      button_b: machine.button_b,
      prize: (
        machine.prize.0 + 10000000000000,
        machine.prize.1 + 10000000000000,
      ),
    })
    .filter_map(|machine| machine.solve(None))
    .sum()
}

fn solve(input: &str, part: u8) -> i64 {
  let machines = parse_input(input);
  match part {
    1 => minimize_tokens_to_win_prizes(&machines),
    2 => minimize_tokens_to_win_prizes_with_modified_positions(&machines),
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
  print_result("input/day13_simple.txt", "Simple puzzle")?;
  print_result("input/day13_full.txt", "Full puzzle")?;
  Ok(())
}
