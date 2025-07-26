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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part1_example() {
    let test_input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    assert_eq!(solve_part1(test_input), 161);
  }

  #[test]
  fn test_valid_mul_instructions() {
    assert_eq!(solve_part1("mul(2,4)"), 8);
    assert_eq!(solve_part1("mul(5,5)"), 25);
    assert_eq!(solve_part1("mul(11,8)"), 88);
    assert_eq!(solve_part1("mul(8,5)"), 40);
  }

  #[test]
  fn test_invalid_mul_instructions() {
    // These should not match
    assert_eq!(solve_part1("mul(4*"), 0);
    assert_eq!(solve_part1("mul(6,9!"), 0);
    assert_eq!(solve_part1("?(12,34)"), 0);
    assert_eq!(solve_part1("mul ( 2 , 4 )"), 0);
    assert_eq!(solve_part1("mul[3,7]"), 0);
    assert_eq!(solve_part1("mul(32,64]"), 0);
  }

  #[test]
  fn test_part2_example() {
    let test_input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    assert_eq!(solve_part2(test_input), 48);
  }

  #[test]
  fn test_part2_do_dont_functionality() {
    // Test basic do/don't functionality
    assert_eq!(solve_part2("mul(2,4)don't()mul(5,5)do()mul(3,3)"), 17); // 8 + 9 = 17

    // Test starting with don't (mul disabled from start)
    assert_eq!(solve_part2("don't()mul(2,4)do()mul(3,3)"), 9); // only 3*3

    // Test multiple do/don't switches
    assert_eq!(
      solve_part2("mul(1,1)don't()mul(2,2)don't()mul(3,3)do()mul(4,4)"),
      17
    ); // 1 + 16 = 17
  }

  #[test]
  fn test_part2_edge_cases() {
    // Test with only do/don't instructions
    assert_eq!(solve_part2("do()don't()do()"), 0);

    // Test ending with don't
    assert_eq!(solve_part2("mul(2,4)don't()"), 8);

    // Test multiple consecutive do/don't
    assert_eq!(solve_part2("do()do()mul(2,4)don't()don't()mul(3,3)"), 8);
  }

  #[test]
  fn test_boundary_cases() {
    // Test 1-3 digit numbers
    assert_eq!(solve_part1("mul(1,1)"), 1);
    assert_eq!(solve_part1("mul(123,456)"), 56088);
    assert_eq!(solve_part1("mul(999,999)"), 998001);

    // Test 4 digit numbers (should not match)
    assert_eq!(solve_part1("mul(1234,5)"), 0);
    assert_eq!(solve_part1("mul(5,1234)"), 0);
  }
}
