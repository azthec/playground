pub struct Game {
  id: u32,
  cubes: Vec<DrawnCubes>,
}

impl Game {
  pub fn validate(&self, max_red: u32, max_green: u32, max_blue: u32) -> bool {
    self
      .cubes
      .iter()
      .all(|drawn_cubes| drawn_cubes.validate(max_red, max_green, max_blue))
  }
}

pub struct DrawnCubes {
  red: u32,
  blue: u32,
  green: u32,
}

impl DrawnCubes {
  pub fn validate(&self, max_red: u32, max_green: u32, max_blue: u32) -> bool {
    self.red <= max_red && self.green <= max_green && self.blue <= max_blue
  }
}

fn parse_game_id(input: &str) -> u32 {
  input
    .strip_prefix("Game ")
    .map(|id| id.parse().ok().unwrap())
    .unwrap()
}

fn parse_color(input: &str) -> (&str, u32) {
  match input.split_once(" ") {
    Some((count, color)) => match count.parse().ok() {
      Some(count) => (color, count),
      None => (color, 0),
    },
    None => unreachable!(),
  }
}

fn parse_colors(input: &str) -> DrawnCubes {
  let colors: HashMap<&str, u32> = input
    .split(",")
    .map(str::trim)
    .map(parse_color)
    .collect::<HashMap<&str, u32>>();
  DrawnCubes {
    red: *colors.get("red").unwrap_or(&0),
    blue: *colors.get("blue").unwrap_or(&0),
    green: *colors.get("green").unwrap_or(&0),
  }
}

use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<Game> {
  input
    .lines()
    .map(|line| {
      let (id, draws) = line.split_once(":").unwrap();
      Game {
        id: parse_game_id(&id),
        cubes: draws.split(";").map(str::trim).map(parse_colors).collect(),
      }
    })
    .collect()
}

#[aoc(day2, part1)]
pub fn part1(games: &Vec<Game>) -> u32 {
  games
    .iter()
    .filter(|game| game.validate(12, 13, 14))
    .map(|game| game.id)
    .sum()
}

#[aoc(day2, part2)]
pub fn part2(games: &Vec<Game>) -> u32 {
  todo!()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part1_example() {
    assert_eq!(part1(&parse("TODO")), 2);
  }

  #[test]
  fn part2_example() {
    assert_eq!(part2(&parse("TODO")), 2);
  }
}

