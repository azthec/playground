use std::cmp::Reverse;

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

  // the rectangle you choose still must have red tiles in opposite corners
  let mut sorted_rectangles_by_area = tiles
    .iter()
    .tuple_combinations()
    .map(|(&(lhs_x, lhs_y), &(rhs_x, rhs_y))| {
      (
        (lhs_x, lhs_y),
        (rhs_x, rhs_y),
        (lhs_x.abs_diff(rhs_x) + 1) * (lhs_y.abs_diff(rhs_y) + 1),
      )
    })
    .sorted_by_key(|(_lhs, _rhs, area)| Reverse(*area));

  let edges: Vec<((usize, usize), (usize, usize))> = tiles
    .windows(2)
    .map(|window| (window[0], window[1]))
    // chain here, just makes sure the last and first coordinates from
    // the input are also on the resulting edge vec
    .chain(std::iter::once((tiles[tiles.len() - 1], tiles[0])))
    .collect();

  let valid_rectangle = sorted_rectangles_by_area
    // find takes only the first valid one, they're already ordered by area
    .find(|(lhs, rhs, _)| valid_rectangle((*lhs, *rhs), &edges))
    .map(|(_, _, area)| area);

  Ok(valid_rectangle.unwrap())
}

// works by checking that the all polygon edges
// start and end on or outside of the rectangle
fn valid_rectangle(
  rect_corners: ((usize, usize), (usize, usize)),
  edges: &[((usize, usize), (usize, usize))],
) -> bool {
  let (a, b) = rect_corners;
  let rect_min_x = a.0.min(b.0);
  let rect_max_x = a.0.max(b.0);
  let rect_min_y = a.1.min(b.1);
  let rect_max_y = a.1.max(b.1);

  edges.iter().all(|&((x1, y1), (x2, y2))| {
    let edge_min_x = x1.min(x2);
    let edge_max_x = x1.max(x2);
    let edge_min_y = y1.min(y2);
    let edge_max_y = y1.max(y2);

    edge_max_x <= rect_min_x
      || edge_min_x >= rect_max_x
      || edge_max_y <= rect_min_y
      || edge_min_y >= rect_max_y
  })
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
    assert_eq!(24, process(input)?);
    Ok(())
  }
}
