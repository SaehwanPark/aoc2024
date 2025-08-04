use anyhow::Result;
use std::collections::{HashMap, VecDeque};
use std::fs;

#[derive(Debug, Clone, PartialEq)]
enum Operation {
  And,
  Or,
  Xor,
}

impl Operation {
  fn from_str(s: &str) -> Option<Self> {
    match s {
      "AND" => Some(Operation::And),
      "OR" => Some(Operation::Or),
      "XOR" => Some(Operation::Xor),
      _ => None,
    }
  }

  fn apply(&self, op1: i32, op2: i32) -> i32 {
    match self {
      Operation::And => op1 & op2,
      Operation::Or => op1 | op2,
      Operation::Xor => op1 ^ op2,
    }
  }
}

#[derive(Debug, Clone)]
struct GateOperation {
  input1: String,
  input2: String,
  output: String,
  operation: Operation,
}

fn parse_input(content: &str) -> Result<(HashMap<String, i32>, Vec<GateOperation>), String> {
  let mut wires = HashMap::new();
  let mut operations = Vec::new();

  for line in content.lines() {
    let line = line.trim();
    if line.is_empty() {
      continue;
    }

    if line.contains(':') {
      // parse initial wire values
      let parts: Vec<&str> = line.split(": ").collect();
      if parts.len() != 2 {
        return Err(format!("invalid wire format: {line}"));
      }
      let wire = parts[0].to_string();
      let value = parts[1]
        .parse::<i32>()
        .map_err(|_| format!("invalid wire value: {}", parts[1]))?;
      wires.insert(wire, value);
    } else if line.contains("->") {
      // parse gate operations
      let parts: Vec<&str> = line.split(' ').collect();
      if parts.len() != 5 {
        return Err(format!("invalid operation format: {line}"));
      }

      let input1 = parts[0].to_string();
      let operation =
        Operation::from_str(parts[1]).ok_or_else(|| format!("unknown operation: {}", parts[1]))?;
      let input2 = parts[2].to_string();
      // parts[3] is "->"
      let output = parts[4].to_string();

      operations.push(GateOperation {
        input1,
        input2,
        output,
        operation,
      });
    }
  }

  Ok((wires, operations))
}

fn find_highest_z_wire(operations: &[GateOperation]) -> String {
  operations
    .iter()
    .filter(|op| op.output.starts_with('z'))
    .map(|op| &op.output)
    .max_by_key(|wire| wire[1..].parse::<u32>().unwrap_or(0))
    .unwrap_or(&"z00".to_string())
    .clone()
}

fn find_wrong_wires(operations: &[GateOperation]) -> Vec<String> {
  let mut wrong = Vec::new();
  let highest_z = find_highest_z_wire(operations);

  for op in operations {
    // rule 1: z wires (except highest) should use XOR
    if op.output.starts_with('z') && op.operation != Operation::Xor && op.output != highest_z {
      wrong.push(op.output.clone());
    }

    // rule 2: XOR between non-input wires shouldn't exist in certain contexts
    if op.operation == Operation::Xor
      && !op.output.starts_with('x')
      && !op.output.starts_with('y')
      && !op.output.starts_with('z')
      && !op.input1.starts_with('x')
      && !op.input1.starts_with('y')
      && !op.input1.starts_with('z')
      && !op.input2.starts_with('x')
      && !op.input2.starts_with('y')
      && !op.input2.starts_with('z')
    {
      wrong.push(op.output.clone());
    }

    // rule 3: AND operations (except x00) should feed into OR operations
    if op.operation == Operation::And && op.input1 != "x00" && op.input2 != "x00" {
      for sub_op in operations {
        if (op.output == sub_op.input1 || op.output == sub_op.input2)
          && sub_op.operation != Operation::Or
        {
          wrong.push(op.output.clone());
          break;
        }
      }
    }

    // rule 4: XOR operations shouldn't feed into OR operations
    if op.operation == Operation::Xor {
      for sub_op in operations {
        if (op.output == sub_op.input1 || op.output == sub_op.input2)
          && sub_op.operation == Operation::Or
        {
          wrong.push(op.output.clone());
          break;
        }
      }
    }
  }

  wrong.sort();
  wrong.dedup();
  wrong
}

fn simulate_circuit(
  mut wires: HashMap<String, i32>,
  operations: Vec<GateOperation>,
) -> HashMap<String, i32> {
  let mut queue: VecDeque<GateOperation> = operations.into();

  while let Some(op) = queue.pop_front() {
    if let (Some(&val1), Some(&val2)) = (wires.get(&op.input1), wires.get(&op.input2)) {
      let result = op.operation.apply(val1, val2);
      wires.insert(op.output, result);
    } else {
      // inputs not ready yet, put back at end of queue
      queue.push_back(op);
    }
  }

  wires
}

fn calculate_z_output(wires: &HashMap<String, i32>) -> u64 {
  let mut z_wires: Vec<_> = wires
    .iter()
    .filter(|(wire, _)| wire.starts_with('z'))
    .collect();

  z_wires.sort_by_key(|(wire, _)| wire.as_str());
  z_wires.reverse();

  let binary_string: String = z_wires
    .iter()
    .map(|&(_, &value)| value.to_string())
    .collect();

  u64::from_str_radix(&binary_string, 2).unwrap_or(0)
}

fn solve(input: &str, part: u8) -> String {
  let (wires, operations) = parse_input(input).expect("Parsing failed.");
  match part {
    1 => {
      let final_wires = simulate_circuit(wires, operations);
      calculate_z_output(&final_wires).to_string()
    }
    2 => find_wrong_wires(&operations).join(","),
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
  print_result("input/day24_simple.txt", "Simple puzzle")?;
  print_result("input/day24_full.txt", "Full puzzle")?;
  Ok(())
}
