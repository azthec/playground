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
  let (fresh_ranges, ingredients) = parse(_input).unwrap().1;

  let count = ingredients
    .iter()
    .filter(|&ingredient| fresh_ranges.iter().any(|range| range.contains(ingredient)))
    .count();

  Ok(count)
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
    assert_eq!(3, process(input)?);
    Ok(())
  }
}
