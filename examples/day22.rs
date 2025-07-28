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

fn solve_part1(input: &str) -> u64 {
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

fn solve_part2(input: &str) -> u64 {
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
    for i in 0..changes.len().saturating_sub(3) {
      let sequence = [changes[i], changes[i + 1], changes[i + 2], changes[i + 3]];

      // Only record the first occurrence of this sequence for this buyer
      if !seen_sequences.contains_key(&sequence) {
        let price = prices[i + 4]; // Price after the 4th change
        seen_sequences.insert(sequence, price);

        // Add to the total for this sequence across all buyers
        *sequence_totals.entry(sequence).or_insert(0) += price as u64;
      }
    }
  }

  // Find the sequence with maximum total bananas
  sequence_totals.values().max().copied().unwrap_or(0)
}

fn debug_part2_example() {
  let input = "1\n2\n3\n2024";
  let initial_secrets: Vec<u64> = input
    .lines()
    .map(|line| line.trim().parse::<u64>().unwrap())
    .collect();

  let target_sequence = [-2, 1, -1, 3];
  let mut total_bananas = 0;

  println!("Checking target sequence {:?}:", target_sequence);
  for (buyer_idx, &secret) in initial_secrets.iter().enumerate() {
    let (prices, changes) = generate_prices_and_changes(secret, 2000);

    // Find the first occurrence of the target sequence
    let mut found_price = None;
    for i in 0..changes.len().saturating_sub(3) {
      let sequence = [changes[i], changes[i + 1], changes[i + 2], changes[i + 3]];
      if sequence == target_sequence {
        found_price = Some(prices[i + 4]);
        break;
      }
    }

    let bananas = found_price.unwrap_or(0);
    total_bananas += bananas as u64;

    println!(
      "Buyer {} (secret {}): {} bananas",
      buyer_idx + 1,
      secret,
      bananas
    );
  }

  println!(
    "Total bananas for sequence {:?}: {}",
    target_sequence, total_bananas
  );

  // Now let's find what the actual optimal sequence is
  println!("\nFinding optimal sequence:");
  let buyers_data: Vec<(Vec<u8>, Vec<i8>)> = initial_secrets
    .into_iter()
    .map(|secret| generate_prices_and_changes(secret, 2000))
    .collect();

  let mut sequence_totals: HashMap<[i8; 4], u64> = HashMap::new();

  for (prices, changes) in &buyers_data {
    let mut seen_sequences = HashMap::new();

    for i in 0..changes.len().saturating_sub(3) {
      let sequence = [changes[i], changes[i + 1], changes[i + 2], changes[i + 3]];

      if !seen_sequences.contains_key(&sequence) {
        let price = prices[i + 4];
        seen_sequences.insert(sequence, price);
        *sequence_totals.entry(sequence).or_insert(0) += price as u64;
      }
    }
  }

  let (best_sequence, best_total) = sequence_totals
    .iter()
    .max_by_key(|&(_, total)| total)
    .map(|(&seq, &total)| (seq, total))
    .unwrap();

  println!(
    "Optimal sequence: {:?} with {} bananas",
    best_sequence, best_total
  );
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Test with simple input
  let simple_input = fs::read_to_string("input/day22_simple.txt")?;
  let simple_result_p1 = solve_part1(&simple_input);
  println!("Part 1 - Simple input result: {}", simple_result_p1);

  let simple_result_p2 = solve_part2(&simple_input);
  println!("Part 2 - Simple input result: {}", simple_result_p2);

  // Debug Part 2 example
  println!("\nDebugging Part 2 example:");
  debug_part2_example();

  // Solve with full input
  let full_input = fs::read_to_string("input/day22_full.txt")?;
  let full_result_p1 = solve_part1(&full_input);
  println!("\nPart 1 - Full input result: {}", full_result_p1);

  let full_result_p2 = solve_part2(&full_input);
  println!("Part 2 - Full input result: {}", full_result_p2);

  // Verify the Part 1 example from the problem description
  println!("\nVerifying Part 1 example:");
  let test_secrets = vec![1, 10, 100, 2024];
  let mut total = 0;
  for initial in test_secrets {
    let result = simulate_buyer(initial, 2000);
    println!("{}: {}", initial, result);
    total += result;
  }
  println!("Expected total: 37327623, Actual total: {}", total);

  Ok(())
}
