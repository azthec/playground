use std::{collections::HashSet, str::Lines};

use nom::Input;

#[derive(Debug)]
enum Operation {
  Start, // S
  Splitter, // ^
}

#[derive(Debug)]
struct Item {
  marker: Operation,
  col: usize,
}

// go down the lines one by one and create a hashset of beams for each one
// if a splitter is hit, increase count and insert beams into the next lines hashset
#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<usize> {
  fn split(item: &Item, max_width: usize) -> Vec<usize> {
    let mut neighbors = Vec::new();
    if item.col > 0 {
      neighbors.push(item.col - 1);
    }
    if item.col + 1 < max_width {
      neighbors.push(item.col + 1);
    }
    neighbors
  }

  let lines_str = _input.lines();
  let max_width = lines_str.clone().map(|l| l.len()).max().unwrap();
  let lines: Vec<Vec<Item>> = parse(lines_str);

  let mut beams: HashSet<usize> = HashSet::new();

  let mut res = 0;
  for line in lines {
    let mut next_beams: HashSet<usize> = HashSet::new();

    for item in &line {
      match item.marker {
        Operation::Start => {
          next_beams.insert(item.col);
        }
        Operation::Splitter if beams.contains(&item.col) => {
          res += 1;
          next_beams.extend(split(&item, max_width));
        }
        _ => {}
      }
    }

    next_beams.extend(beams.difference(&line.iter().map(|i| i.col).collect()));

    beams = next_beams;
  }

  Ok(res)
}

fn parse(lines: Lines) -> Vec<Vec<Item>> {
  lines
    .map(|line| {
      line
        .iter_elements()
        .enumerate()
        .filter_map(|(col, c)| match c {
          'S' => Some(Item {
            marker: Operation::Start,
            col,
          }),
          '^' => Some(Item {
            marker: Operation::Splitter,
            col,
          }),
          _ => None,
        })
        .collect()
    })
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_process() -> miette::Result<()> {
    let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
    assert_eq!(21, process(input)?);
    Ok(())
  }
}
