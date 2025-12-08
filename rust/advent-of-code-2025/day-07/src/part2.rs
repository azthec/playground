use std::{
  collections::{HashMap, HashSet},
  str::Lines,
};

use nom::Input;

#[derive(Debug)]
enum Operation {
  Start,    // S
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
  fn split(col: usize, max_width: usize) -> Vec<usize> {
    let mut neighbors = Vec::new();
    if col > 0 {
      neighbors.push(col - 1);
    }
    if col + 1 < max_width {
      neighbors.push(col + 1);
    }
    neighbors
  }

  let lines_str = _input.lines();
  let max_width = lines_str.clone().map(|l| l.len()).max().unwrap();
  let lines: Vec<Vec<Item>> = parse(lines_str);

  // make it a hashmap instead to count the number of possible paths
  // to a single beam
  let mut beam_counts: HashMap<usize, usize> = HashMap::new();

  for line in lines {
    let mut next_beam_counts: HashMap<usize, usize> = HashMap::new();

    for item in &line {
      match item.marker {
        Operation::Start => {
          *next_beam_counts.entry(item.col).or_insert(0) += 1;
        }
        Operation::Splitter if beam_counts.contains_key(&item.col) => {
          let count = beam_counts[&item.col];
          for new_col in split(item.col, max_width) {
            *next_beam_counts.entry(new_col).or_insert(0) += count;
          }
        }
        _ => {}
      }
    }

    // beams that didn't hit anything
    let item_cols: HashSet<usize> = line.iter().map(|i| i.col).collect();
    for (col, count) in beam_counts {
      if !item_cols.contains(&col) {
        *next_beam_counts.entry(col).or_insert(0) += count;
      }
    }

    beam_counts = next_beam_counts;
  }

  Ok(beam_counts.values().sum())
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
    assert_eq!(40, process(input)?);
    Ok(())
  }
}

