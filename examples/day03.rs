use regex::Regex;
use std::fs;

fn solve_part1(input: &str) -> i32 {
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

fn solve_part2(input: &str) -> i32 {
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

fn main() {
  let input = fs::read_to_string("input/day03_full.txt").expect("Failed to read input file");
  let part1_result = solve_part1(&input);
  let part2_result = solve_part2(&input);
  println!("Part 1 Answer = {part1_result}");
  println!("Part 2 Answer = {part2_result}");
}
