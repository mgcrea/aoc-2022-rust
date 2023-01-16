// day 04

// #![allow(warnings)]
// use debug_rs::debug;
use regex::Regex;
use std::{cmp, ops::Range};

type Input = Vec<(Range<u8>, Range<u8>)>;

fn parse_input(input: &str, safe: bool) -> Input {
  let regex = Regex::new(r"^[0-9]+-[0-9]+,[0-9]+-[0-9]+$").unwrap();
  input
    .lines()
    .filter(|line| {
      if safe {
        regex.is_match(line)
      } else {
        line.len() > 0
      }
    })
    .filter_map(|line| {
      let (a, b) = line.split_once(",")?;
      let (a0, a1) = a.split_once("-")?;
      let (b0, b1) = b.split_once("-")?;
      Some((
        (a0.parse().ok()?..a1.parse().ok()?),
        (b0.parse().ok()?..b1.parse().ok()?),
      ))
    })
    .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
  Some(
    parse_input(input, false)
      .iter()
      .filter(|(a, b)| envelops(a, b))
      .count(),
  )
}

pub fn part_two(input: &str) -> Option<usize> {
  Some(
    parse_input(input, false)
      .iter()
      .filter(|(a, b)| overlaps(a, b))
      .count(),
  )
}

// helpers

fn envelops(a: &Range<u8>, b: &Range<u8>) -> bool {
  if a.len() > b.len() {
    b.start >= a.start && b.end <= a.end
  } else {
    a.start >= b.start && a.end <= b.end
  }
}

fn overlaps(a: &Range<u8>, b: &Range<u8>) -> bool {
  cmp::max(a.start, b.start) <= cmp::min(a.end, b.end)
}

// main

fn main() {
  let input = &advent_of_code::read_file("inputs", 4);
  advent_of_code::solve!(1, part_one, input);
  advent_of_code::solve!(2, part_two, input);
}

// tests

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let input = advent_of_code::read_file("examples", 4);
    assert_eq!(part_one(&input), Some(2));
  }

  #[test]
  fn test_part_two() {
    let input = advent_of_code::read_file("examples", 4);
    assert_eq!(part_two(&input), Some(4));
  }
}
