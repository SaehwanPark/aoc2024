use anyhow::{Context, Result, bail};
use clap::Parser;
use std::{collections::HashSet, fs::read_to_string, path::PathBuf};

/// AoC 2024 – Day 17 (Chronospatial Computer)
#[derive(Parser)]
struct Cli {
  /// Optional path to custom puzzle input
  #[arg()]
  input: Option<PathBuf>,
}

/// CPU registers
#[derive(Clone, Copy, Debug)]
struct Regs {
  a: i128,
  b: i128,
  c: i128,
}

impl Regs {
  /// Evaluate a *combo* operand (0‑6 → value, 7 never used).
  #[inline]
  fn combo(&self, op: u8) -> i128 {
    match op {
      0..=3 => op as i128,
      4 => self.a,
      5 => self.b,
      6 => self.c,
      _ => panic!("operand 7 is reserved"),
    }
  }
}

/// Parse the block that AoC gives us.
fn parse_input(txt: &str) -> Result<(Regs, Vec<u8>)> {
  let mut a = None;
  let mut b = None;
  let mut c = None;
  let mut program = Vec::new();

  for line in txt.lines().filter(|l| !l.trim().is_empty()) {
    let l = line.trim();
    if let Some(rest) = l.strip_prefix("Register A:") {
      a = Some(rest.trim().parse()?);
    } else if let Some(rest) = l.strip_prefix("Register B:") {
      b = Some(rest.trim().parse()?);
    } else if let Some(rest) = l.strip_prefix("Register C:") {
      c = Some(rest.trim().parse()?);
    } else if let Some(rest) = l.strip_prefix("Program:") {
      program = rest
        .split(',')
        .map(|t| t.trim().parse::<u8>())
        .collect::<Result<_, _>>()?;
    }
  }

  Ok((
    Regs {
      a: a.context("missing Register A")?,
      b: b.context("missing Register B")?,
      c: c.context("missing Register C")?,
    },
    program,
  ))
}

/// Run the full program and return everything the `out` instruction emits.
fn exec(mut regs: Regs, prog: &[u8]) -> Result<Vec<u8>> {
  let mut pc = 0usize;
  let mut out = Vec::new();

  while pc < prog.len() {
    let opcode = prog[pc];
    let operand = *prog
      .get(pc + 1)
      .context("dangling opcode at end of program")?;

    match opcode {
      0 | 6 | 7 => {
        // adv/bdv/cdv instructions
        let exp = regs.combo(operand);
        if !(0..=126).contains(&exp) {
          bail!("exponent {exp} is out of range");
        }
        let denom = 1_i128 << exp;
        let result = regs.a.div_euclid(denom);

        match opcode {
          0 => regs.a = result,
          6 => regs.b = result,
          7 => regs.c = result,
          _ => unreachable!(),
        }
      }
      1 => regs.b ^= operand as i128,
      2 => regs.b = regs.combo(operand) & 7,
      3 => {
        if regs.a != 0 {
          pc = operand as usize;
          continue;
        }
      }
      4 => regs.b ^= regs.c,
      5 => out.push((regs.combo(operand) & 7) as u8),
      _ => bail!("unknown opcode {opcode}"),
    }

    pc += 2;
  }
  Ok(out)
}

/// Simulate **exactly one loop iteration** of the program.
/// Returns `(digit_emitted, next_A)` where next_A is the value of register A
/// after one complete iteration of the program loop.
///
/// This is more robust than running until first output because it properly
/// detects when the program loops back to the beginning or halts.
fn step_once(a0: i128, init_b: i128, init_c: i128, prog: &[u8]) -> Result<(u8, i128)> {
  let (mut a, mut b, mut c) = (a0, init_b, init_c);
  let mut pc = 0usize;
  let mut digit = 0u8;
  let mut first_pass = true;

  loop {
    // If we've returned to the beginning and it's not our first time, we've completed one iteration
    if pc == 0 && !first_pass {
      return Ok((digit, a));
    }
    first_pass = false;

    if pc >= prog.len() {
      // Program halted naturally
      return Ok((digit, 0));
    }

    let opcode = prog[pc];
    let operand = *prog
      .get(pc + 1)
      .context("dangling opcode at end of program")?;

    // Helper for combo operands within this iteration
    let combo = |op: u8| -> i128 {
      match op {
        0..=3 => op as i128,
        4 => a,
        5 => b,
        6 => c,
        _ => panic!("operand 7 is reserved"),
      }
    };

    match opcode {
      0 => {
        let exp = combo(operand);
        if !(0..=126).contains(&exp) {
          bail!("exponent {exp} is out of range in adv");
        }
        a = a.div_euclid(1_i128 << exp);
      }
      1 => b ^= operand as i128,
      2 => b = combo(operand) & 7,
      3 => {
        if a != 0 {
          pc = operand as usize;
          continue;
        } else {
          // Program will halt after this iteration
          return Ok((digit, 0));
        }
      }
      4 => b ^= c,
      5 => digit = (combo(operand) & 7) as u8,
      6 => {
        let exp = combo(operand);
        if !(0..=126).contains(&exp) {
          bail!("exponent {exp} is out of range in bdv");
        }
        b = a.div_euclid(1_i128 << exp);
      }
      7 => {
        let exp = combo(operand);
        if !(0..=126).contains(&exp) {
          bail!("exponent {exp} is out of range in cdv");
        }
        c = a.div_euclid(1_i128 << exp);
      }
      _ => bail!("unknown opcode {opcode}"),
    }
    pc += 2;
  }
}

/// Find the smallest positive initial value for register A that causes the
/// program to output a copy of itself (a quine).
///
/// Uses reverse search: starting from the final state (A=0), work backwards
/// through each program digit to find all possible A values that could
/// produce that digit, then select the minimum.
fn find_quine_value(init_b: i128, init_c: i128, prog: &[u8]) -> Result<i128> {
  // Each element represents a possible value of A *after* one iteration
  let mut frontier: HashSet<i128> = [0].into_iter().collect();

  // Work backwards through the program digits
  for (step, &required_digit) in prog.iter().rev().enumerate() {
    let mut next_frontier = HashSet::new();

    for &next_a in &frontier {
      // Try all possible 3-bit extensions (since A is typically divided by 8 each iteration)
      for r in 0..8 {
        let candidate_a = next_a * 8 + r;

        // Test if this candidate produces the required digit and transitions to next_a
        match step_once(candidate_a, init_b, init_c, prog) {
          Ok((digit, after_a)) => {
            if digit == required_digit && after_a == next_a {
              next_frontier.insert(candidate_a);
            }
          }
          Err(_) => {
            // Skip invalid candidates that cause simulation errors
            continue;
          }
        }
      }
    }

    if next_frontier.is_empty() {
      bail!(
        "No valid candidates found for step {} (digit {})",
        step,
        required_digit
      );
    }

    frontier = next_frontier;
  }

  // Get the minimum candidate
  let best_a = *frontier.iter().min().context("No valid candidates found")?;

  // Validate the solution by running the complete program
  let test_regs = Regs {
    a: best_a,
    b: init_b,
    c: init_c,
  };
  let full_output = exec(test_regs, prog)?;

  if full_output.len() != prog.len() || full_output != prog {
    bail!(
      "Validation failed: output {:?} doesn't match program {:?}",
      full_output,
      prog
    );
  }

  Ok(best_a)
}

/// Process a single input file and return results
fn process_input(input_path: &PathBuf) -> Result<(String, Option<i128>)> {
  let txt =
    read_to_string(input_path).with_context(|| format!("reading {}", input_path.display()))?;
  let (init_regs, prog) = parse_input(&txt)?;

  // Part 1
  let part1 = exec(init_regs, &prog)?
    .into_iter()
    .map(|d| d.to_string())
    .collect::<Vec<_>>()
    .join(",");

  // Part 2 - attempt to find quine value
  let part2 = match find_quine_value(init_regs.b, init_regs.c, &prog) {
    Ok(value) => {
      // Verify Part 2 solution
      let verification_regs = Regs {
        a: value,
        b: init_regs.b,
        c: init_regs.c,
      };
      let verification_output = exec(verification_regs, &prog)?;
      if prog != verification_output {
        bail!("Part 2 verification failed for {}", input_path.display());
      }
      Some(value)
    }
    Err(_) => None, // This program cannot be a quine
  };

  Ok((part1, part2))
}

fn main() -> Result<()> {
  let cli = Cli::parse();

  // If custom input provided, use it; otherwise use both standard inputs
  if let Some(custom_input) = cli.input {
    let (part1, part2) = process_input(&custom_input)?;
    println!("Input: {}", custom_input.display());
    println!("Part 1 = {part1}");
    match part2 {
      Some(value) => println!("Part 2 = {value}"),
      None => println!("Part 2 = N/A (program cannot be a quine)"),
    }
    return Ok(());
  }

  // Process both standard inputs
  let simple_path = PathBuf::from("input/day17_simple.txt");
  let full_path = PathBuf::from("input/day17_full.txt");

  // Try to process both inputs, but don't fail if one is missing
  let simple_result = process_input(&simple_path);
  let full_result = process_input(&full_path);

  // Part 1 Results
  println!("Part 1 Results:");
  match &simple_result {
    Ok((part1, _)) => println!("  Simple: {part1}"),
    Err(e) => println!("  Simple: Error - {e}"),
  }
  match &full_result {
    Ok((part1, _)) => println!("  Full:   {part1}"),
    Err(e) => println!("  Full:   Error - {e}"),
  }
  println!();

  // Part 2 Results
  println!("Part 2 Results:");
  match &simple_result {
    Ok((_, Some(part2))) => println!("  Simple: {part2}"),
    Ok((_, None)) => println!("  Simple: N/A (cannot be quine)"),
    Err(e) => println!("  Simple: Error - {e}"),
  }
  match &full_result {
    Ok((_, Some(part2))) => println!("  Full:   {part2}"),
    Ok((_, None)) => println!("  Full:   N/A (cannot be quine)"),
    Err(e) => println!("  Full:   Error - {e}"),
  }

  Ok(())
}
