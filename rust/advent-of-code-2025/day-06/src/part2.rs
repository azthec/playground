use itertools::Itertools;
use nom::{
  IResult, Parser,
  branch::alt,
  character::complete::{char, space0},
  combinator::map,
  multi::separated_list1,
};

#[derive(Debug)]
enum Ops {
  PLUS,
  MULTI,
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<usize> {
  let (nums, ops) = parse(_input);

  let result = nums
    .iter()
    .zip(ops)
    .map(|(op_nums, op)| match op {
      Ops::PLUS => op_nums.iter().sum::<usize>(),
      Ops::MULTI => op_nums.iter().product::<usize>(),
    })
    .sum();
  Ok(result)
}

fn parse(input: &str) -> (Vec<Vec<usize>>, Vec<Ops>) {
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

  let lines: Vec<&str> = input.lines().collect();
  let (num_lines, op_line) = lines.split_at(lines.len() - 1);

  let max_width = num_lines.iter().map(|l| l.len()).max().unwrap();

  let nums: Vec<Vec<usize>> = (0..max_width)
    .map(|col| {
      num_lines
        .iter()
        .filter_map(|line| line.chars().nth(col))
        .collect::<String>()
    })
    .chunk_by(|col| col.trim().is_empty())
    .into_iter()
    .filter_map(|(is_empty, group)| {
      if is_empty {
        None
      } else {
        Some(
          group
            .map(|col| col.trim().parse::<usize>().unwrap())
            .collect::<Vec<usize>>(),
        )
      }
    })
    .collect();

  let (_, ops) = parse_ops(op_line.first().unwrap()).unwrap();

  (nums, ops)
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
    assert_eq!(3263827, process(input)?);
    Ok(())
  }
}
