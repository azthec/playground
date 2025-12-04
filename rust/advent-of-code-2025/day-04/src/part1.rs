use nom::{
  IResult, Parser,
  branch::alt,
  character::complete::{char, line_ending, space0},
  combinator::map,
  multi::{many1, separated_list1},
  sequence::delimited,
};

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<usize> {
  let grid = parse(_input).unwrap().1;
  Ok(
    grid
      .iter()
      .enumerate()
      .map(|(i, row)| {
        row
          .iter()
          .enumerate()
          .filter(|&(j, &cell)| cell && count_neighbours(&grid, i, j) < 4)
          .count()
      })
      .sum(),
  )
}

fn count_neighbours(matrix: &Vec<Vec<bool>>, row: usize, col: usize) -> usize {
  let mut count = 0;
  for dr in -1..=1_i32 {
    for dc in -1..=1_i32 {
      if dr == 0 && dc == 0 {
        continue;
      }

      let new_row = row.wrapping_add(dr as usize);
      let new_col = col.wrapping_add(dc as usize);

      if let Some(row_data) = matrix.get(new_row) {
        if let Some(&cell) = row_data.get(new_col) {
          if cell {
            count += 1;
          }
        }
      }
    }
  }
  count
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<bool>>> {
  fn cell(input: &str) -> IResult<&str, bool> {
    alt((map(char('.'), |_| false), map(char('@'), |_| true))).parse(input)
  }

  fn row(input: &str) -> IResult<&str, Vec<bool>> {
    delimited(space0, many1(cell), space0).parse(input)
  }

  separated_list1(line_ending, row).parse(input)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_process() -> miette::Result<()> {
    let input = "..@@.@@@@.
                 @@@.@.@.@@
                 @@@@@.@.@@
                 @.@@@@..@.
                 @@.@@@@.@@
                 .@@@@@@@.@
                 .@.@.@.@@@
                 @.@@@.@@@@
                 .@@@@@@@@.
                 @.@.@@@.@.";
    assert_eq!(13, process(input)?);
    Ok(())
  }
}
