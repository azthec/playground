const BANK_SIZE: usize = 12;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<u64> {
  let grid = parse_grid(_input);

  Ok(grid.iter().map(|bank| highest(bank, BANK_SIZE)).sum())
}

// we scan once left to right, when we find a digit higher than what's on the
// current k position in the result, we can set that new max and zero out
// everything after it, so long as there are enough remaining numbers in
// the bank to fill the resulting number to its desired size
fn highest(battery_bank: &Vec<u64>, bank_size: usize) -> u64 {
  let mut maximums: Vec<u64> = vec![0; bank_size];

  for (idx, &digit) in battery_bank.iter().enumerate() {
    let digits_remaining = battery_bank.len() - idx - 1;

    for k in 0..bank_size {
      let positions_after = bank_size - k - 1;

      if digits_remaining >= positions_after && digit > maximums[k] {
        maximums[k..].fill(0);
        maximums[k] = digit;
        break;
      }
    }
  }

  maximums.iter().fold(0u64, |acc, &digit| acc * 10 + digit)
}

fn parse_grid(input: &str) -> Vec<Vec<u64>> {
  input
    .lines()
    .filter(|line| !line.is_empty())
    .map(|line| {
      line
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as u64)
        .collect()
    })
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
    assert_eq!(3121910778619, process(input)?);
    Ok(())
  }

  #[test]
  fn test_highest() -> miette::Result<()> {
    let battery_bank = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1];
    assert_eq!(98, highest(&battery_bank, 2));
    let battery_bank = vec![8, 1, 1, 1, 1, 4, 3, 2, 1, 1, 1, 9];
    assert_eq!(89, highest(&battery_bank, 2));
    let battery_bank = vec![8, 1, 1, 1, 1, 9, 3, 2, 1, 1, 1, 3];
    assert_eq!(93, highest(&battery_bank, 2));
    let battery_bank = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1];
    assert_eq!(987654321111, highest(&battery_bank, 12));
    let battery_bank = vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9];
    assert_eq!(811111111119, highest(&battery_bank, 12));
    let battery_bank = vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8];
    assert_eq!(434234234278, highest(&battery_bank, 12));
    let battery_bank = vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1];
    assert_eq!(888911112111, highest(&battery_bank, 12));
    Ok(())
  }
}
