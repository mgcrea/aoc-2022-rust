// https://github.com/fspoettel/advent-of-code-2022/blob/main/src/bin/01.rs

use debug_rs::debug;
use std::cmp::Reverse;

type Input = Vec<u32>;

fn parse(input: &str) -> Input {
  input
    .split("\n\n")
    .map(|c| c.lines().filter_map(|l| l.parse::<u32>().ok()).sum())
    .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
  let parsed = parse(input);
  parsed.iter().max().copied()
}

pub fn part_two(input: &str) -> Option<u32> {
  let mut parsed = parse(input);
  parsed.sort_by_key(|k| Reverse(*k));
  let slice = &parsed[0..3];
  debug!();
  return Some(slice.iter().sum::<u32>());
}

fn main() {
  let input = &advent_of_code::read_file("inputs", 1);
  advent_of_code::solve!(1, part_one, input);
  advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let input = advent_of_code::read_file("examples", 1);
    assert_eq!(part_one(&input), Some(24000));
  }

  #[test]
  fn test_part_two() {
    let input = advent_of_code::read_file("examples", 1);
    assert_eq!(part_two(&input), Some(45000));
  }
}
