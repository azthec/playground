#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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

fn min_cubes(game: &Game) -> (u32, u32, u32) {
  game.cubes.iter().fold(
    (0, 0, 0),
    |(min_red, min_blue, min_green), drawn_cubes| {
      (
        min_red.max(drawn_cubes.red),
        min_blue.max(drawn_cubes.blue),
        min_green.max(drawn_cubes.green),
      )
    },
  )
}

#[aoc(day2, part2)]
pub fn part2(games: &Vec<Game>) -> u32 {
  games.iter().map(min_cubes).map(|(r, g, b)| r * g * b).sum()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parser_sample1() {
    assert_eq!(
      parse("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
      vec![Game {
        id: 1,
        cubes: vec![
          DrawnCubes {
            red: 4,
            blue: 3,
            green: 0
          },
          DrawnCubes {
            red: 1,
            blue: 6,
            green: 2
          },
          DrawnCubes {
            red: 0,
            blue: 0,
            green: 2
          }
        ]
      }]
    );
  }

  #[test]
  fn parser_sample2() {
    assert_eq!(
      parse("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
      vec![Game {
        id: 2,
        cubes: vec![
          DrawnCubes {
            red: 0,
            blue: 1,
            green: 2
          },
          DrawnCubes {
            red: 1,
            blue: 4,
            green: 3
          },
          DrawnCubes {
            red: 0,
            blue: 1,
            green: 1
          }
        ]
      }]
    );
  }

  #[test]
  fn parser_sample3() {
    assert_eq!(
      parse("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"),
      vec![Game {
        id: 3,
        cubes: vec![
          DrawnCubes {
            red: 20,
            blue: 6,
            green: 8
          },
          DrawnCubes {
            red: 4,
            blue: 5,
            green: 13
          },
          DrawnCubes {
            red: 1,
            blue: 0,
            green: 5
          }
        ]
      }]
    );
  }

  #[test]
  fn parser_sample4() {
    assert_eq!(
      parse("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"),
      vec![Game {
        id: 4,
        cubes: vec![
          DrawnCubes {
            red: 3,
            blue: 6,
            green: 1
          },
          DrawnCubes {
            red: 6,
            blue: 0,
            green: 3
          },
          DrawnCubes {
            red: 14,
            blue: 15,
            green: 3
          }
        ]
      }]
    );
  }

  #[test]
  fn parser_sample5() {
    assert_eq!(
      parse("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"),
      vec![Game {
        id: 5,
        cubes: vec![
          DrawnCubes {
            red: 6,
            blue: 1,
            green: 3
          },
          DrawnCubes {
            red: 1,
            blue: 2,
            green: 2
          }
        ]
      }]
    );
  }

  #[test]
  fn test_min_cubes() {
    let game = Game {
      id: 1,
      cubes: vec![
        DrawnCubes {
          red: 3,
          blue: 4,
          green: 0,
        },
        DrawnCubes {
          red: 1,
          blue: 2,
          green: 6,
        },
        DrawnCubes {
          red: 0,
          blue: 0,
          green: 2,
        },
      ],
    };

    assert_eq!(min_cubes(&game), (0, 0, 0));
  }

  #[test]
  fn test_validate1() {
    let game = Game {
      id: 1,
      cubes: vec![
        DrawnCubes {
          red: 3,
          blue: 4,
          green: 0,
        },
        DrawnCubes {
          red: 1,
          blue: 2,
          green: 6,
        },
        DrawnCubes {
          red: 0,
          blue: 0,
          green: 2,
        },
      ],
    };

    assert!(game.validate(5, 7, 5));
  }

  #[test]
  fn test_validate2() {
    let game = Game {
      id: 1,
      cubes: vec![
        DrawnCubes {
          red: 3,
          blue: 4,
          green: 0,
        },
        DrawnCubes {
          red: 1,
          blue: 2,
          green: 6,
        },
        DrawnCubes {
          red: 0,
          blue: 0,
          green: 2,
        },
      ],
    };

    assert!(!game.validate(5, 5, 5));
  }
}
