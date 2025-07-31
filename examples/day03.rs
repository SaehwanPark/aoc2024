use anyhow::Result;
use regex::Regex;
use std::fs;

fn calculate_sumproduct(input: &str) -> i32 {
  // pattern to match valid mul(X,Y) instruction where X,Y are 1-3 digits
  let mul_regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("Failed to compile regex");

  let mut total = 0;

  for captures in mul_regex.captures_iter(input) {
    let x: i32 = captures[1].parse().expect("Failed to parse first number");
    let y: i32 = captures[2].parse().expect("Failed to parse second number");
    total += x * y;
  }

  total
}

fn calculate_sumproduct_with_instruction(input: &str) -> i32 {
  let instruction_regex = Regex::new(r"(?:mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\))")
    .expect("Failed to compile regex");
  let mut total = 0;
  let mut mul_enabled = true; // enabled at the beginning

  // process all instructions in order
  for captures in instruction_regex.captures_iter(input) {
    let full_match = &captures[0];

    match full_match {
      "do()" => {
        mul_enabled = true;
      }
      "don't()" => {
        mul_enabled = false;
      }
      _ => {
        if mul_enabled {
          let x: i32 = captures[1].parse().expect("Failed to parse first number");
          let y: i32 = captures[2].parse().expect("Failed to parse second number");

          total += x * y;
        }
      }
    }
  }
  total
}

fn solve(input: &str, part: u8) -> i32 {
  match part {
    1 => calculate_sumproduct(input),
    2 => calculate_sumproduct_with_instruction(input),
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
  print_result("input/day03_full.txt", "Full puzzle")?;
  Ok(())
}
