use nom::{
  IResult, Parser,
  branch::alt,
  character::complete::{self, char, line_ending, space0},
  combinator::map,
  multi::separated_list1,
  sequence::{delimited, separated_pair},
};

#[derive(Debug)]
enum Ops {
  PLUS,
  MULTI,
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<usize> {
  let (_, (nums, ops)) = parse(_input).unwrap();

  let result: usize = ops
    .iter()
    .enumerate()
    .map(|(i, op)| match op {
      Ops::PLUS => nums.iter().map(|row| row[i]).sum::<usize>(),
      Ops::MULTI => nums.iter().map(|row| row[i]).product::<usize>(),
    })
    .sum();

  Ok(result)
}

fn parse(input: &str) -> IResult<&str, (Vec<Vec<usize>>, Vec<Ops>)> {
  fn parse_numbers(input: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(space0, complete::usize).parse(input)
  }

  fn parse_ops(input: &str) -> IResult<&str, Vec<Ops>> {
    separated_list1(
      space0,
      alt((
        map(char('*'), |_| Ops::MULTI),
        map(char('+'), |_| Ops::PLUS),
      )),
    )
    .parse(input)
  }

  separated_pair(
    separated_list1(line_ending, delimited(space0, parse_numbers, space0)),
    line_ending,
    delimited(space0, parse_ops, space0),
  )
  .parse(input)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_process() -> miette::Result<()> {
    let input = "123 328  51 64 
                  45 64  387 23 
                   6 98  215 314
                 *   +   *   +  ";
    assert_eq!(4277556, process(input)?);
    Ok(())
  }
}
