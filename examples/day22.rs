use anyhow::Result;
use std::collections::HashMap;
use std::fs;

fn mix(value: u64, secret: u64) -> u64 {
  value ^ secret
}

fn prune(secret: u64) -> u64 {
  secret % 16777216
}

fn next_secret(mut secret: u64) -> u64 {
  // Step 1: multiply by 64, mix, prune
  let result1 = secret * 64;
  secret = mix(result1, secret);
  secret = prune(secret);

  // Step 2: divide by 32 (round down), mix, prune
  let result2 = secret / 32;
  secret = mix(result2, secret);
  secret = prune(secret);

  // Step 3: multiply by 2048, mix, prune
  let result3 = secret * 2048;
  secret = mix(result3, secret);
  secret = prune(secret);

  secret
}

fn simulate_buyer(initial_secret: u64, iterations: usize) -> u64 {
  let mut secret = initial_secret;
  for _ in 0..iterations {
    secret = next_secret(secret);
  }
  secret
}

fn sum_of_2000th_secret_nums(input: &str) -> u64 {
  input
    .lines()
    .map(|line| line.trim().parse::<u64>().unwrap())
    .map(|initial_secret| simulate_buyer(initial_secret, 2000))
    .sum()
}

fn generate_prices_and_changes(initial_secret: u64, iterations: usize) -> (Vec<u8>, Vec<i8>) {
  let mut secret = initial_secret;
  let mut prices = Vec::with_capacity(iterations + 1);

  // Initial price (ones digit of initial secret)
  prices.push((secret % 10) as u8);

  // Generate subsequent prices
  for _ in 0..iterations {
    secret = next_secret(secret);
    prices.push((secret % 10) as u8);
  }

  // Calculate changes between consecutive prices
  let changes: Vec<i8> = prices
    .windows(2)
    .map(|window| window[1] as i8 - window[0] as i8)
    .collect();

  (prices, changes)
}

fn maximize_bananas_to_get(input: &str) -> u64 {
  let initial_secrets: Vec<u64> = input
    .lines()
    .map(|line| line.trim().parse::<u64>().unwrap())
    .collect();

  // Generate prices and changes for all buyers
  let buyers_data: Vec<(Vec<u8>, Vec<i8>)> = initial_secrets
    .into_iter()
    .map(|secret| generate_prices_and_changes(secret, 2000))
    .collect();

  // For each possible sequence of 4 changes, calculate total bananas
  let mut sequence_totals: HashMap<[i8; 4], u64> = HashMap::new();

  for (prices, changes) in &buyers_data {
    let mut seen_sequences = HashMap::new();

    // Go through all possible 4-change sequences for this buyer
    for (i, window) in changes.windows(4).enumerate() {
      let sequence: [_; 4] = window.try_into().unwrap();

      // only process if this is the first time we've seen this sequence
      if let std::collections::hash_map::Entry::Vacant(entry) = seen_sequences.entry(sequence) {
        let price = prices[i + 4];
        entry.insert(price);
        *sequence_totals.entry(sequence).or_insert(0) += price as u64;
      }
    }
  }

  // Find the sequence with maximum total bananas
  sequence_totals.values().max().copied().unwrap_or(0)
}

fn solve(input: &str, part: u8) -> u64 {
  match part {
    1 => sum_of_2000th_secret_nums(input),
    2 => maximize_bananas_to_get(input),
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
  print_result("input/day22_simple.txt", "Simple puzzle")?;
  print_result("input/day22_full.txt", "Full puzzle")?;
  Ok(())
}
