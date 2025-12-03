#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<u32> {
  let grid = parse_grid(_input);

  Ok(grid.iter().map(highest).sum())
}

fn highest(battery_bank: &Vec<u32>) -> u32 {
  battery_bank
    .iter()
    .enumerate()
    .fold((0, 0), |(max_joltage, best_first), (i, &digit)| {
      let new_joltage = best_first * 10 + digit;
      let new_max = max_joltage.max(new_joltage);

      let new_best_first = if i < battery_bank.len() - 1 {
        best_first.max(digit)
      } else {
        best_first
      };

      (new_max, new_best_first)
    })
    .0
}

fn parse_grid(input: &str) -> Vec<Vec<u32>> {
  input
    .lines()
    .filter(|line| !line.is_empty())
    .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect())
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_process() -> miette::Result<()> {
    let input = "987654321111111
                 811111111111119
                 234234234234278
                 818181911112111";
    assert_eq!(357, process(input)?);
    Ok(())
  }

  #[test]
  fn test_highest() -> miette::Result<()> {
    let battery_bank = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1];
    assert_eq!(98, highest(&battery_bank));
    let battery_bank = vec![8, 1, 1, 1, 1, 4, 3, 2, 1, 1, 1, 9];
    assert_eq!(89, highest(&battery_bank));
    let battery_bank = vec![8, 1, 1, 1, 1, 9, 3, 2, 1, 1, 1, 3];
    assert_eq!(93, highest(&battery_bank));
    Ok(())
  }
}
