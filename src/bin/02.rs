// day 02

// #![allow(warnings)]
// use debug_rs::debug;
use regex::Regex;

type Round = (u32, u32);
type Input = Vec<Round>;

fn parse_input(input: &str, safe: bool) -> Input {
  let regex = Regex::new(r"^[A-Z] [A-Z]$").unwrap();
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
      let bytes = line.as_bytes();
      Some(((bytes.first()? - 65) as u32, (bytes.last()? - 88) as u32))
    })
    .collect::<Input>()
}

fn compute_score((a, b): Round) -> u32 {
  let score = ((3 - ((2 + a - b) % 3)) % 3) * 3;
  return score + b + 1;
}

pub fn part_one(input: &str) -> Option<u32> {
  Some(
    parse_input(input, false)
      .iter()
      .map(|&round| compute_score(round))
      .sum(),
  )
}

pub fn part_two(input: &str) -> Option<u32> {
  Some(
    parse_input(input, false)
      .iter()
      .map(|&(a, r)| {
        let b = match r {
          0 => (a + 2) % 3,
          1 => a,
          2 => (a + 1) % 3,
          _ => unreachable!(),
        };
        compute_score((a, b))
      })
      .sum(),
  )
}

fn main() {
  let input = &advent_of_code::read_file("inputs", 2);
  advent_of_code::solve!(1, part_one, input);
  advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let input = advent_of_code::read_file("examples", 2);
    assert_eq!(part_one(&input), Some(15));
  }

  #[test]
  fn test_part_two() {
    let input = advent_of_code::read_file("examples", 2);
    assert_eq!(part_two(&input), Some(12));
  }
}
