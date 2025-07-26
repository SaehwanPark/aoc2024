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

  fn solve_part1(&self) -> u32 {
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

  fn solve_part2(&self) -> u32 {
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

fn main() {
  let simple_input =
    fs::read_to_string("input/day05_simple.txt").expect("Failed to read simple input file");
  let print_queue = PrintQueue::from_input(&simple_input);
  let result_p1 = print_queue.solve_part1();
  let result_p2 = print_queue.solve_part2();
  println!("Simple input result: {result_p1} and {result_p2}");
  assert_eq!(result_p1, 143, "Simple input should yield 143 for part 1");
  assert_eq!(result_p2, 123, "Simple input should yield 143 for part 2");

  let full_input =
    fs::read_to_string("input/day05_full.txt").expect("Failed to read simple input file");
  let print_queue_full = PrintQueue::from_input(&full_input);
  let full_result_p1 = print_queue_full.solve_part1();
  let full_result_p2 = print_queue_full.solve_part2();
  println!("Full input result: {full_result_p1} and {full_result_p2}");
}
