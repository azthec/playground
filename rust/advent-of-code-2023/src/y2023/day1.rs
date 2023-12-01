#[aoc(day1, part1, Chars)]
pub fn part1_chars(input: &str) -> i32 {
  input
    .lines()
    .map(|line| line.chars().collect::<String>())
    .map(|line| parse_calibration(&line))
    .sum()
}

fn parse_calibration(input: &str) -> i32 {
  let digits: Vec<_> = input.chars().filter(|&char| char.is_digit(10)).collect();

  if let (Some(first_digit), Some(last_digit)) = (digits.first(), digits.last()) {
    let result: String = vec![*first_digit, *last_digit].iter().collect();
    result.parse().unwrap()
  } else {
    unreachable!()
  }
}

#[cfg(test)]
mod tests {
  use crate::y2023::day1::parse_calibration;

  #[test]
  fn sample1() {
    assert_eq!(parse_calibration("1abc2"), 12);
  }

  #[test]
  fn sample2() {
    assert_eq!(parse_calibration("pqr3stu8vwx"), 38);
  }

  #[test]
  fn sample3() {
    assert_eq!(parse_calibration("a1b2c3d4e5f"), 15);
  }

  #[test]
  fn sample4() {
    assert_eq!(parse_calibration("treb7uchet"), 77);
  }
}
