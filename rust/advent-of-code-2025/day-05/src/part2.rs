use itertools::Itertools;
use nom::{
  IResult, Parser,
  character::complete::{self, char, line_ending, multispace1, space0},
  combinator::map,
  multi::separated_list1,
  sequence::{delimited, separated_pair},
};
use std::ops::RangeInclusive;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<usize> {
  let (fresh_ranges, _ingredients) = parse(_input).unwrap().1;

  let merged_ranges = fresh_ranges.into_iter().sorted_by_key(|r| *r.start()).fold(
    vec![],
    |mut acc: Vec<RangeInclusive<usize>>, range| {
      match acc.last_mut() {
        Some(last) if range.start() <= &(last.end() + 1) => {
          *last = *last.start()..=(*last.end()).max(*range.end());
        }
        _ => acc.push(range),
      }
      acc
    },
  );

  let sum = merged_ranges
    .iter()
    .map(|range| range.end() - range.start() + 1)
    .sum();

  Ok(sum)
}

fn parse(input: &str) -> IResult<&str, (Vec<RangeInclusive<usize>>, Vec<usize>)> {
  fn parse_range(input: &str) -> IResult<&str, RangeInclusive<usize>> {
    delimited(
      space0,
      map(
        separated_pair(complete::usize, char('-'), complete::usize),
        |(start, end)| start..=end,
      ),
      space0,
    )
    .parse(input)
  }

  separated_pair(
    separated_list1(line_ending, parse_range),
    multispace1,
    separated_list1(line_ending, delimited(space0, complete::usize, space0)),
  )
  .parse(input)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_process() -> miette::Result<()> {
    let input = "3-5
                 10-14
                 16-20
                 12-18

                 1
                 5
                 8
                 11
                 17
                 32";
    assert_eq!(14, process(input)?);
    Ok(())
  }
}

