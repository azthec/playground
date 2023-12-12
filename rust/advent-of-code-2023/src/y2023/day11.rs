use itertools::Itertools;
use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day11)]
fn parse(input: &str) -> Vec<String> {
  input
    .lines()
    .map(|line| line.to_string())
    .collect::<Vec<String>>()
}

#[derive(Debug, PartialEq, Clone)]
pub struct Galaxy {
  x: usize,
  y: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CosmicExpansions {
  horizontal: HashSet<usize>,
  vertical: HashSet<usize>,
}

fn cosmic_expansions(
  galaxies: &Vec<Galaxy>,
  horizontal_size: usize,
  vertical_size: usize,
) -> CosmicExpansions {
  CosmicExpansions {
    horizontal: (0..horizontal_size)
      .filter(|&x| galaxies.iter().all(|g| g.x != x))
      .collect(),
    vertical: (0..vertical_size)
      .filter(|&y| galaxies.iter().all(|g| g.y != y))
      .collect(),
  }
}

fn cosmic_expansions_between_galaxies(
  galaxy1: &Galaxy,
  galaxy2: &Galaxy,
  cosmic_expansions: &CosmicExpansions,
) -> (usize, usize) {
  let min_x = galaxy1.x.min(galaxy2.x);
  let max_x = galaxy1.x.max(galaxy2.x);
  let min_y = galaxy1.y.min(galaxy2.y);
  let max_y = galaxy1.y.max(galaxy2.y);
  (
    (min_x..max_x)
      .filter(|&x| cosmic_expansions.horizontal.contains(&x))
      .collect::<Vec<usize>>()
      .len(),
    (min_y..max_y)
      .filter(|&y| cosmic_expansions.vertical.contains(&y))
      .collect::<Vec<usize>>()
      .len(),
  )
}

fn galaxies(input: &Vec<String>) -> Vec<Galaxy> {
  input
    .iter()
    .enumerate()
    .flat_map(|(line_idx, line)| {
      let _galaxies: Vec<Galaxy> = line
        .char_indices()
        .filter_map(|(char_idx, char)| {
          if char == '#' {
            Some(Galaxy {
              x: char_idx,
              y: line_idx,
            })
          } else {
            None
          }
        })
        .collect();
      _galaxies
    })
    .collect()
}

fn distance_between_galaxies(
  galaxy1: &Galaxy,
  galaxy2: &Galaxy,
  cosmic_expansions: &CosmicExpansions,
  expansion_modifier: usize,
) -> (usize, usize) {
  let (cosmic_expansions_x, cosmic_expansions_y) =
    cosmic_expansions_between_galaxies(&galaxy1, &galaxy2, cosmic_expansions);
  (
    (galaxy1.x.abs_diff(galaxy2.x)) + (cosmic_expansions_x * expansion_modifier),
    (galaxy1.y.abs_diff(galaxy2.y)) + (cosmic_expansions_y * expansion_modifier),
  )
}

fn reduce_distances(galaxy_distances: &Vec<(usize, usize)>) -> usize {
  galaxy_distances.iter().map(|(a, b)| a + b).sum::<usize>()
}

fn calculate_distances(input: &Vec<String>, expansion_modifier: usize) -> usize {
  let horizontal_size = input.first().map(|l| l.len()).unwrap_or(0);
  let vertical_size = input.len();
  let galaxies = galaxies(input);
  let cosmic_expansions = cosmic_expansions(&galaxies, horizontal_size, vertical_size);
  let galaxy_pairs: Vec<(&Galaxy, &Galaxy)> = galaxies
    .iter()
    .combinations(2)
    .map(|pair| (pair[0], pair[1]))
    .collect();
  let galaxy_distances: Vec<(usize, usize)> = galaxy_pairs
    .iter()
    .map(|(galaxy1, galaxy2)| {
      distance_between_galaxies(galaxy1, galaxy2, &cosmic_expansions, expansion_modifier)
    })
    .collect();
  reduce_distances(&galaxy_distances)
}

#[aoc(day11, part1)]
fn part1(input: &Vec<String>) -> usize {
  calculate_distances(input, 1)
}

#[aoc(day11, part2)]
fn part2(input: &Vec<String>) -> usize {
  calculate_distances(input, 999_999)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sum_galaxy_distance() {
    let lines = vec![".#........", "..........", ".......#.."]
      .iter()
      .map(|s| s.to_string())
      .collect::<Vec<String>>();
    assert_eq!(part1(&lines), 14);
  }

  #[test]
  fn sum_galaxy_distance2() {
    let lines = vec![
      "...#......",
      ".......#..",
      "#.........",
      "..........",
      "......#...",
      ".#........",
      ".........#",
      "..........",
      ".......#..",
      "#...#.....",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect::<Vec<String>>();
    assert_eq!(part1(&lines), 374);
  }

  #[test]
  fn space_expands() {
    let input = vec!["...#......", "..........", "......#..."]
      .iter()
      .map(|s| s.to_string())
      .collect::<Vec<String>>();
    let galaxies = galaxies(&input);
    let cosmic_expansions = cosmic_expansions(&galaxies, 10, 3);
    assert_eq!(
      cosmic_expansions,
      CosmicExpansions {
        horizontal: HashSet::from([0, 1, 2, 4, 5, 7, 8, 9]),
        vertical: HashSet::from([1])
      }
    );
  }

  #[test]
  fn space_expands2() {
    let lines = vec![
      "...#......",
      ".......#..",
      "#.........",
      "..........",
      "......#...",
      ".#........",
      ".........#",
      "..........",
      ".......#..",
      "#...#.....",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect::<Vec<String>>();
    let galaxies = galaxies(&lines);
    let cosmic_expansions = cosmic_expansions(&galaxies, 10, 10);
    assert_eq!(
      cosmic_expansions,
      CosmicExpansions {
        horizontal: HashSet::from([2, 5, 8]),
        vertical: HashSet::from([3, 7]),
      }
    );
  }

  #[test]
  fn galaxy_distance_to_itself() {
    let galaxy1 = Galaxy { x: 0, y: 0 };
    let cosmic_expansions = CosmicExpansions {
      horizontal: HashSet::new(),
      vertical: HashSet::new(),
    };
    let (x, y) = distance_between_galaxies(&galaxy1, &galaxy1, &cosmic_expansions, 1);
    assert_eq!(x + y, 0)
  }

  #[test]
  fn galaxy_distance_to_below() {
    let galaxy1 = Galaxy { x: 0, y: 0 };
    let galaxy2 = Galaxy { x: 0, y: 2 };
    let cosmic_expansions = CosmicExpansions {
      horizontal: HashSet::new(),
      vertical: HashSet::new(),
    };
    let (x, y) = distance_between_galaxies(&galaxy1, &galaxy2, &cosmic_expansions, 1);
    assert_eq!(x + y, 2)
  }

  #[test]
  fn galaxy_distance_to_above() {
    let galaxy1 = Galaxy { x: 0, y: 5 };
    let galaxy2 = Galaxy { x: 0, y: 0 };
    let cosmic_expansions = CosmicExpansions {
      horizontal: HashSet::new(),
      vertical: HashSet::new(),
    };
    let (x, y) = distance_between_galaxies(&galaxy1, &galaxy2, &cosmic_expansions, 1);
    assert_eq!(x + y, 5)
  }

  #[test]
  fn galaxy_distance_to_right() {
    let galaxy1 = Galaxy { x: 0, y: 0 };
    let galaxy2 = Galaxy { x: 3, y: 0 };
    let cosmic_expansions = CosmicExpansions {
      horizontal: HashSet::new(),
      vertical: HashSet::new(),
    };
    let (x, y) = distance_between_galaxies(&galaxy1, &galaxy2, &cosmic_expansions, 1);
    assert_eq!(x + y, 3)
  }

  #[test]
  fn galaxy_distance_to_left() {
    let galaxy1 = Galaxy { x: 8, y: 0 };
    let galaxy2 = Galaxy { x: 0, y: 0 };
    let cosmic_expansions = CosmicExpansions {
      horizontal: HashSet::new(),
      vertical: HashSet::new(),
    };
    let (x, y) = distance_between_galaxies(&galaxy1, &galaxy2, &cosmic_expansions, 1);
    assert_eq!(x + y, 8)
  }

  #[test]
  fn galaxy_distance_to_right_with_horizontal_space() {
    let galaxy1 = Galaxy { x: 0, y: 0 };
    let galaxy2 = Galaxy { x: 4, y: 0 };
    let cosmic_expansions = CosmicExpansions {
      horizontal: HashSet::from([1]),
      vertical: HashSet::new(),
    };
    let (x, y) = distance_between_galaxies(&galaxy1, &galaxy2, &cosmic_expansions, 1);
    assert_eq!(x + y, 5)
  }

  #[test]
  fn galaxy_distance_to_right_with_vertical_space() {
    let galaxy1 = Galaxy { x: 0, y: 0 };
    let galaxy2 = Galaxy { x: 0, y: 4 };
    let cosmic_expansions = CosmicExpansions {
      horizontal: HashSet::new(),
      vertical: HashSet::from([1, 5]),
    };
    let (x, y) = distance_between_galaxies(&galaxy1, &galaxy2, &cosmic_expansions, 1);
    assert_eq!(x + y, 5)
  }
}
