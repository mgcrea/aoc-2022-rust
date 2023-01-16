// day 03

#![allow(warnings)]
use debug_rs::debug;
use regex::Regex;
use std::collections::HashSet;

type Input = Vec<Vec<u8>>;

fn score_char(code: u8) -> u8 {
  code % 32 + (26 * (code <= 90) as u8)
}

fn parse_input(input: &str, safe: bool) -> Input {
  let regex = Regex::new(r"^[a-zA-Z]$").unwrap();
  input
    .lines()
    .filter(|line| {
      if safe {
        regex.is_match(line)
      } else {
        line.len() > 0
      }
    })
    .filter_map(|line| Some(line.as_bytes().iter().map(|&v| score_char(v)).collect()))
    .collect::<Input>()
}

pub fn part_one(input: &str) -> Option<u32> {
  Some(
    parse_input(input, false)
      .iter()
      .map(|bytes| bytes.split_at(bytes.len() / 2))
      .filter_map(|(left, right)| left.iter().find(|byte| right.contains(byte)))
      .map(|&v| v as u32)
      .sum(),
  )
}

pub fn part_two(input: &str) -> Option<u32> {
  Some(
    parse_input(input, false)
      .iter()
      .map(|bytes| bytes.iter().collect::<HashSet<&u8>>())
      .collect::<Vec<HashSet<&u8>>>()
      .chunks_exact(3)
      .filter_map(|slice| {
        let a = slice.first().unwrap();
        let b = slice.get(1).unwrap();
        let c = slice.last().unwrap();
        a.intersection(&b)
          .reduce(|acc, v| if c.contains(v) { v } else { acc })
          .copied()
      })
      .map(|&v| v as u32)
      .sum(),
  )
}

fn main() {
  let input = &advent_of_code::read_file("inputs", 3);
  advent_of_code::solve!(1, part_one, input);
  advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let input = advent_of_code::read_file("examples", 3);
    assert_eq!(part_one(&input), Some(157));
  }

  #[test]
  fn test_part_two() {
    let input = advent_of_code::read_file("examples", 3);
    assert_eq!(part_two(&input), Some(70));
  }
}
