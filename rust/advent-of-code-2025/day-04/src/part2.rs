use std::iter::successors;

use rayon::prelude::*;

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

  let total: usize = successors(Some((grid, 0)), |(g, _)| {
    let (new_grid, removed) = remove_accessible(g);
    if removed > 0 {
      Some((new_grid, removed))
    } else {
      None
    }
  })
  .map(|(_, count)| count)
  .sum();

  Ok(total)
}

fn remove_accessible(grid: &Vec<Vec<bool>>) -> (Vec<Vec<bool>>, usize) {
  let (new_grid, removed): (Vec<Vec<bool>>, Vec<usize>) = grid
    .par_iter() // easy parallelism optimisation, works fine with iter
    .enumerate()
    .map(|(i, row)| {
      let (new_row, new_removed): (Vec<bool>, Vec<usize>) = row
        .iter()
        .enumerate()
        .map(|(j, &cell)| {
          if cell && count_neighbours(&grid, i, j) < 4 {
            (false, 1)
          } else {
            (cell, 0)
          }
        })
        .unzip();

      (new_row, new_removed.iter().sum::<usize>())
    })
    .unzip();
  (new_grid, removed.iter().sum::<usize>())
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
    assert_eq!(43, process(input)?);
    Ok(())
  }
}
