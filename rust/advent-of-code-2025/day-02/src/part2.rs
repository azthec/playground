use itertools::Itertools;
use nom::{
  IResult, Parser, bytes::complete::tag, character::complete, multi::separated_list1,
  sequence::separated_pair,
};

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<u64> {
  let ranges = parse(_input).unwrap();

  let mut total = 0;

  for (low, high) in ranges.1 {
    for id in low..=high {
      if is_invalid(id) {
        total += id;
      }
    }
  }

  Ok(total)
}

fn is_invalid(id: u64) -> bool {
  let id_str = id.to_string();
  let len = id_str.len();

  for pattern_len in 1..=(len / 2) {
    if len % pattern_len == 0 {
      let chunks = substrings(&id_str, pattern_len);
      if chunks.iter().all(|chunk| chunk == &chunks[0]) {
        return true;
      }
    }
  }

  false
}

fn parse(input: &str) -> IResult<&str, Vec<(u64, u64)>> {
  separated_list1(
    tag(","),
    separated_pair(complete::u64, tag("-"), complete::u64),
  )
  .parse(input)
}

fn substrings(source: &str, size: usize) -> Vec<String> {
  source
    .chars()
    .chunks(size)
    .into_iter()
    .map(|chunk| chunk.collect::<String>())
    .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_process() -> miette::Result<()> {
    let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    assert_eq!(4174379265, process(input)?);
    Ok(())
  }
}
