use anyhow::Result;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Block {
  File(u32), // File ID
  Free,
}

impl Block {
  fn is_free(self) -> bool {
    matches!(self, Block::Free)
  }

  fn file_id(self) -> Option<u32> {
    match self {
      Block::File(id) => Some(id),
      Block::Free => None,
    }
  }
}

#[derive(Debug)]
struct Disk {
  blocks: Vec<Block>,
}

impl Disk {
  fn from_disk_map(disk_map: &str) -> Self {
    let mut blocks = Vec::new();
    let mut file_id = 0u32;
    let mut is_file = true;

    for digit_char in disk_map.trim().chars() {
      let length = digit_char.to_digit(10).expect("Invalid digit in disk map") as usize;

      if is_file {
        // Add file blocks
        blocks.extend(std::iter::repeat_n(Block::File(file_id), length));
        file_id += 1;
      } else {
        // Add free space blocks
        blocks.extend(std::iter::repeat_n(Block::Free, length));
      }

      is_file = !is_file;
    }

    Self { blocks }
  }

  fn compact(&mut self) {
    let mut left = 0;
    let mut right = self.blocks.len().saturating_sub(1);

    while left < right {
      // Find next free space from left
      while left < self.blocks.len() && !self.blocks[left].is_free() {
        left += 1;
      }

      // Find next file block from right
      while right > 0 && self.blocks[right].is_free() {
        right = right.saturating_sub(1);
      }

      // If we found both a free space and a file block, swap them
      if left < right {
        self.blocks.swap(left, right);
        left += 1;
        right = right.saturating_sub(1);
      }
    }
  }

  fn compact_whole_files(&mut self) {
    // Get the highest file ID
    let max_file_id = self
      .blocks
      .iter()
      .filter_map(|block| block.file_id())
      .max()
      .unwrap_or(0);

    // Process files in decreasing order of file ID
    for file_id in (0..=max_file_id).rev() {
      if let Some((file_start, file_size)) = self.find_file(file_id) {
        if let Some(free_start) = self.find_free_space_before(file_start, file_size) {
          // Move the entire file
          for i in 0..file_size {
            self.blocks[free_start + i] = Block::File(file_id);
            self.blocks[file_start + i] = Block::Free;
          }
        }
      }
    }
  }

  fn find_file(&self, file_id: u32) -> Option<(usize, usize)> {
    let start = self
      .blocks
      .iter()
      .position(|&block| block == Block::File(file_id))?;

    let size = self.blocks[start..]
      .iter()
      .take_while(|&&block| block == Block::File(file_id))
      .count();

    Some((start, size))
  }

  fn find_free_space_before(&self, before_position: usize, required_size: usize) -> Option<usize> {
    let mut current_free_start = None;
    let mut current_free_size = 0;

    for (i, &block) in self.blocks[..before_position].iter().enumerate() {
      if block.is_free() {
        if current_free_start.is_none() {
          current_free_start = Some(i);
          current_free_size = 1;
        } else {
          current_free_size += 1;
        }

        if current_free_size >= required_size {
          return current_free_start;
        }
      } else {
        current_free_start = None;
        current_free_size = 0;
      }
    }

    None
  }

  fn checksum(&self) -> u64 {
    self
      .blocks
      .iter()
      .enumerate()
      .filter_map(|(position, &block)| block.file_id().map(|id| position as u64 * id as u64))
      .sum()
  }

  #[allow(dead_code)]
  fn display(&self) -> String {
    self
      .blocks
      .iter()
      .map(|&block| match block {
        Block::File(id) => {
          if id < 10 {
            char::from_digit(id, 10).unwrap()
          } else {
            '?' // For IDs >= 10, use '?' as placeholder
          }
        }
        Block::Free => '.',
      })
      .collect()
  }
}

fn solve(input: &str, part: u8) -> u64 {
  let mut disk = Disk::from_disk_map(input);
  match part {
    1 => disk.compact(),
    2 => disk.compact_whole_files(),
    _ => panic!("Only parts 1 and 2."),
  };
  disk.checksum()
}

fn print_result(filepath: &str, puzzle_kind: &str) -> Result<()> {
  let input = fs::read_to_string(filepath)?;
  println!("Input: {puzzle_kind}");
  println!("Part 1 result = {}", solve(&input, 1));
  println!("Part 2 result = {}\n", solve(&input, 2));
  Ok(())
}

fn main() -> Result<()> {
  print_result("input/day09_simple.txt", "Simple puzzle")?;
  print_result("input/day09_full.txt", "Full puzzle")?;
  Ok(())
}
