use std::iter::{self, repeat_with};

use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day12)]
fn parse(input: &str) -> Vec<RowSprings> {
  input
    .lines()
    .map(|line| match line.split_once(' ') {
      Some((springs, counts)) => RowSprings {
        springs: springs
          .chars()
          .map(|char| match char {
            '.' => Condition::Operational,
            '#' => Condition::Damaged,
            '?' => Condition::Unknown,
            _ => panic!("Unknown character"),
          })
          .collect(),
        counts: counts
          .split(',')
          .map(|x| x.parse::<usize>().unwrap())
          .collect(),
      },
      None => panic!("Unable to parse input"),
    })
    .collect::<Vec<RowSprings>>()
}

fn count_arrangements(row_springs: RowSprings) -> usize {
  let mut springs: Vec<Condition> = row_springs.springs.clone();
  springs.push(Condition::Operational);
  let mut counts: Vec<usize> = row_springs.counts;
  counts.push(springs.len());

  let positions = vec![0; springs.len() + 1];
  let groups = vec![positions; counts.len() + 2];
  let mut lengths = vec![groups; springs.len() + 2];

  lengths[0][0][0] = 1;

  for position in 0..springs.len() {
    for (group, &_max_count) in counts.iter().enumerate() {
      for length in 0..springs.len() + 1 {
        match &lengths
          .get(position)
          .and_then(|groups| groups.get(group))
          .and_then(|lenghts| lenghts.get(length))
        {
          Some(&current) if current > 0 => {
            let spring = &springs[position];
            if *spring == Condition::Operational || *spring == Condition::Unknown {
              if length == 0 || length == counts[group - 1] {
                lengths[position + 1][group][0] += current;
              }
            }
            if *spring == Condition::Damaged || *spring == Condition::Unknown {
              let modifier = if length == 0 { 1 } else { 0 };
              lengths[position + 1][group + modifier][length + 1] += current;
            }
          }
          _ => continue,
        }
      }
    }
  }

  lengths[springs.len()][counts.len() - 1][0]
}

#[derive(Debug, PartialEq, Clone)]
pub struct RowSprings {
  springs: Vec<Condition>,
  counts: Vec<usize>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Condition {
  Operational,
  Damaged,
  Unknown,
}

#[aoc(day12, part1)]
fn part1(row_springs: &Vec<RowSprings>) -> usize {
  row_springs
    .iter()
    .map(|row| count_arrangements(row.clone()))
    .sum()
}

fn repeat<A: Clone>(vec: Vec<A>, amount: usize) -> Vec<A> {
  iter::repeat(vec).take(amount).flatten().collect()
}

fn repeat_with_separator<A: Clone>(vec: Vec<A>, separator: A, amount: usize) -> Vec<A> {
  repeat_with(|| vec.clone())
    .take(amount)
    .flat_map(|v| v.into_iter().chain(Some(separator.clone())))
    .take((vec.len() + 1) * amount - 1)
    .collect::<Vec<A>>()
}

#[aoc(day12, part2)]
fn part2(row_springs: &Vec<RowSprings>) -> usize {
  row_springs
    .iter()
    .map(|row| {
      count_arrangements(RowSprings {
        springs: repeat_with_separator(row.springs.clone(), Condition::Unknown, 5),
        counts: repeat(row.counts.clone(), 5),
      })
    })
    .sum()
}

#[cfg(test)]
mod tests {
  use super::*;

  fn equal<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
    let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
    matching == a.len() && matching == b.len()
  }

  #[test]
  fn repeats_with_separator() {
    let input = ".# 1";
    let parsed = parse(input);
    let unfolded: Vec<Condition> = parsed
      .iter()
      .flat_map(|row| repeat_with_separator(row.springs.clone(), Condition::Unknown, 5))
      .collect();
    let expected: Vec<_> = vec![
      Condition::Operational,
      Condition::Damaged,
      Condition::Unknown,
      Condition::Operational,
      Condition::Damaged,
      Condition::Unknown,
      Condition::Operational,
      Condition::Damaged,
      Condition::Unknown,
      Condition::Operational,
      Condition::Damaged,
      Condition::Unknown,
      Condition::Operational,
      Condition::Damaged,
    ];
    assert!(equal(&unfolded, &expected));
  }

  #[test]
  fn repeats() {
    let input = ".# 1";
    let parsed = parse(input);
    let unfolded: Vec<_> = parsed
      .iter()
      .flat_map(|row| repeat(row.counts.clone(), 5))
      .collect();
    let expected: Vec<_> = vec![1, 1, 1, 1, 1];
    assert!(equal(&unfolded, &expected));
  }

  #[test]
  fn row1() {
    let input = "#.#.### 1,1,3";
    let parsed = parse(input);
    assert_eq!(count_arrangements(parsed.first().unwrap().clone()), 1);
  }

  #[test]
  fn row2() {
    let input = ".??..??...?##. 1,1,3";
    let parsed = parse(input);
    assert_eq!(count_arrangements(parsed.first().unwrap().clone()), 4);
  }

  #[test]
  fn row3() {
    let input = "?#?#?#?#?#?#?#? 1,3,1,6";
    let parsed = parse(input);
    assert_eq!(count_arrangements(parsed.first().unwrap().clone()), 1);
  }

  #[test]
  fn row4() {
    let input = "????.#...#... 4,1,1";
    let parsed = parse(input);
    assert_eq!(count_arrangements(parsed.first().unwrap().clone()), 1);
  }

  #[test]
  fn row5() {
    let input = "????.######..#####. 1,6,5";
    let parsed = parse(input);
    assert_eq!(count_arrangements(parsed.first().unwrap().clone()), 4);
  }

  #[test]
  fn row6() {
    let input = "?###???????? 3,2,1";
    let parsed = parse(input);
    assert_eq!(count_arrangements(parsed.first().unwrap().clone()), 10);
  }
}
