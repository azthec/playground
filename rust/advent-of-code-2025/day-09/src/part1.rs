use itertools::Itertools;
use nom::{
  IResult, Parser,
  character::complete::{self, char, line_ending, space0},
  multi::separated_list1,
  sequence::{delimited, separated_pair},
};

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<usize> {
  let (_, tiles) = parse(_input).unwrap();
  let result = tiles
    .iter()
    .tuple_combinations()
    .map(|(&(lhs_x, lhs_y), &(rhs_x, rhs_y))| {
      (lhs_x.abs_diff(rhs_x) + 1) * (lhs_y.abs_diff(rhs_y) + 1)
    })
    .max();
  Ok(result.unwrap())
}

fn parse(input: &str) -> IResult<&str, Vec<(usize, usize)>> {
  separated_list1(
    line_ending,
    delimited(
      space0,
      separated_pair(complete::usize, char(','), complete::usize),
      space0,
    ),
  )
  .parse(input)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_process() -> miette::Result<()> {
    let input = "7,1
                 11,1
                 11,7
                 9,7
                 9,5
                 2,5
                 2,3
                 7,3";
    assert_eq!(50, process(input)?);
    Ok(())
  }
}
