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
      return self.numbers.get(0) == Some(&self.test_value);
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

fn solve_part1(input: &str) -> u64 {
  input
    .lines()
    .filter_map(Equation::from_line)
    .filter(|eq| eq.can_be_solved())
    .map(|eq| eq.test_value)
    .sum()
}

fn solve_part2(input: &str) -> u64 {
  input
    .lines()
    .filter_map(Equation::from_line)
    .filter(|eq| eq.can_be_solved_with_concatenation())
    .map(|eq| eq.test_value)
    .sum()
}

fn main() {
  // Test with simple example
  let simple_input =
    fs::read_to_string("input/day07_simple.txt").expect("Failed to read simple input file");

  let simple_result_part1 = solve_part1(&simple_input);
  println!("Part 1 (simple): {}", simple_result_part1);
  assert_eq!(simple_result_part1, 3749);

  let simple_result_part2 = solve_part2(&simple_input);
  println!("Part 2 (simple): {}", simple_result_part2);
  assert_eq!(simple_result_part2, 11387);

  // Solve with full input
  let full_input =
    fs::read_to_string("input/day07_full.txt").expect("Failed to read full input file");

  let full_result_part1 = solve_part1(&full_input);
  println!("Part 1 (full): {}", full_result_part1);

  let full_result_part2 = solve_part2(&full_input);
  println!("Part 2 (full): {}", full_result_part2);
}
