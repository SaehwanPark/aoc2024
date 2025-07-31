use anyhow::Result;
use std::fs;

#[derive(Debug, Clone)]
struct Equation {
  test_value: u64,
  numbers: Vec<u64>,
}

#[derive(Debug, Clone, Copy)]
enum Operator {
  Add,
  Multiply,
  Concatenate,
}

impl Equation {
  fn from_line(line: &str) -> Option<Self> {
    let (test_part, numbers_part) = line.split_once(": ")?;
    let test_value = test_part.parse().ok()?;
    let numbers = numbers_part
      .split_whitespace()
      .filter_map(|s| s.parse().ok())
      .collect();

    Some(Equation {
      test_value,
      numbers,
    })
  }

  fn can_be_solved(&self) -> bool {
    self.can_be_solved_with_operators(&[Operator::Add, Operator::Multiply])
  }

  fn can_be_solved_with_concatenation(&self) -> bool {
    self.can_be_solved_with_operators(&[Operator::Add, Operator::Multiply, Operator::Concatenate])
  }

  fn can_be_solved_with_operators(&self, available_operators: &[Operator]) -> bool {
    if self.numbers.len() < 2 {
      return self.numbers.first() == Some(&self.test_value);
    }

    let operator_count = self.numbers.len() - 1;
    let operator_base = available_operators.len();
    let total_combinations = operator_base.pow(operator_count as u32);

    for combination in 0..total_combinations {
      let mut result = self.numbers[0];
      let mut temp_combination = combination;

      for i in 0..operator_count {
        let operator_index = temp_combination % operator_base;
        temp_combination /= operator_base;
        let operator = available_operators[operator_index];

        result = match operator {
          Operator::Add => result + self.numbers[i + 1],
          Operator::Multiply => result * self.numbers[i + 1],
          Operator::Concatenate => concatenate_numbers(result, self.numbers[i + 1]),
        };

        // Early termination if result exceeds test_value (optimization)
        if result > self.test_value {
          break;
        }
      }

      if result == self.test_value {
        return true;
      }
    }

    false
  }
}

fn concatenate_numbers(left: u64, right: u64) -> u64 {
  let right_digits = if right == 0 { 1 } else { right.ilog10() + 1 };
  left * 10_u64.pow(right_digits) + right
}

fn get_total_calibration_result(input: &str) -> u64 {
  input
    .lines()
    .filter_map(Equation::from_line)
    .filter(|eq| eq.can_be_solved())
    .map(|eq| eq.test_value)
    .sum()
}

fn get_total_calibration_result_with_concatenation(input: &str) -> u64 {
  input
    .lines()
    .filter_map(Equation::from_line)
    .filter(|eq| eq.can_be_solved_with_concatenation())
    .map(|eq| eq.test_value)
    .sum()
}

fn solve(input: &str, part: u8) -> u64 {
  match part {
    1 => get_total_calibration_result(input),
    2 => get_total_calibration_result_with_concatenation(input),
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
  print_result("input/day07_simple.txt", "Simple puzzle")?;
  print_result("input/day07_full.txt", "Full puzzle")?;
  Ok(())
}
