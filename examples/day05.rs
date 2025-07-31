use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug)]
struct PrintQueue {
  ordering_rules: HashMap<u32, HashSet<u32>>,
  updates: Vec<Vec<u32>>,
}

impl PrintQueue {
  fn from_input(input: &str) -> Self {
    let sections: Vec<&str> = input.trim().split("\n\n").collect();

    let mut ordering_rules: HashMap<u32, HashSet<u32>> = HashMap::new();

    // parse ordering rules
    // X|Y means X must come before Y
    for line in sections[0].lines() {
      if let Some((before, after)) = line.split_once('|') {
        let before_num: u32 = before.parse().expect("Invalid number");
        let after_num: u32 = after.parse().expect("Invalid number");

        ordering_rules
          .entry(before_num)
          .or_default()
          .insert(after_num);
      }
    }

    // parse updates
    let updates = sections[1]
      .lines()
      .map(|line| {
        line
          .split(',')
          .map(|num| num.parse().expect("Invalid number"))
          .collect()
      })
      .collect();

    Self {
      ordering_rules,
      updates,
    }
  }

  fn is_update_valid(&self, update: &[u32]) -> bool {
    // for each pair of pages in the updates, check if they violate any rules
    for (i, &page_a) in update.iter().enumerate() {
      for &page_b in &update[i + 1..] {
        // check if page_b should come before page_a
        if let Some(must_com_after) = self.ordering_rules.get(&page_b) {
          if must_com_after.contains(&page_a) {
            return false;
          }
        }
      }
    }
    true
  }

  fn get_middle_page(&self, update: &[u32]) -> u32 {
    update[update.len() / 2]
  }

  fn sum_middle_pages_of_valid_updates(&self) -> u32 {
    self
      .updates
      .iter()
      .filter(|u| self.is_update_valid(u))
      .map(|u| self.get_middle_page(u))
      .sum()
  }

  fn fix_update_order(&self, update: &[u32]) -> Vec<u32> {
    let mut pages = update.to_vec();

    let mut changed = true;
    while changed {
      changed = false;

      for i in 0..pages.len() - 1 {
        let page_a = pages[i];
        let page_b = pages[i + 1];

        // check if page_b should come before page_a
        if let Some(must_come_after) = self.ordering_rules.get(&page_b) {
          if must_come_after.contains(&page_a) {
            // swap them
            pages.swap(i, i + 1);
            changed = true;
          }
        }
      }
    }
    pages
  }

  fn sum_middle_pages_with_fixed_updates(&self) -> u32 {
    self
      .updates
      .iter()
      .filter(|u| !self.is_update_valid(u))
      .map(|u| {
        let fixed_update = self.fix_update_order(u);
        self.get_middle_page(&fixed_update)
      })
      .sum()
  }
}
fn solve(input: &str, part: u8) -> u32 {
  let print_queue = PrintQueue::from_input(input);
  match part {
    1 => print_queue.sum_middle_pages_of_valid_updates(),
    2 => print_queue.sum_middle_pages_with_fixed_updates(),
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
  print_result("input/day05_simple.txt", "Simple puzzle")?;
  print_result("input/day05_full.txt", "Full puzzle")?;
  Ok(())
}
