#[derive(Debug)]
enum Direction {
  Left(i32),
  Right(i32),
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<i32> {
  let directions = parse_directions(_input);

  let mut dial = 50;
  let mut total_zeros = 0;

  directions.iter().for_each(|direction| {
    let steps = match direction {
      Direction::Left(value) => *value,
      Direction::Right(value) => *value,
    };

    let going_right = matches!(direction, Direction::Right(_));

    for _ in 0..steps {
      if going_right {
        dial = (dial + 1) % 100;
      } else {
        dial = if dial == 0 { 99 } else { dial - 1 };
      }

      if dial == 0 {
        total_zeros += 1;
      }
    }
  });

  return Ok(total_zeros);
}

fn parse_directions(input: &str) -> Vec<Direction> {
  input
    .lines()
    .filter(|line| !line.trim().is_empty())
    .map(|line| {
      let line = line.trim();
      let direction = &line[0..1];
      let amount = line[1..].parse::<i32>().unwrap();

      match direction {
        "L" => Direction::Left(amount),
        "R" => Direction::Right(amount),
        _ => panic!("unknown: {}", direction),
      }
    })
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_process() -> miette::Result<()> {
    let input = "L68
                 L30
                 R48
                 L5
                 R60
                 L55
                 L1
                 L99
                 R14
                 L82";
    assert_eq!(6, process(input)?);
    Ok(())
  }
}
