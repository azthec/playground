use glam::{Vec3, vec3};
use itertools::Itertools;
use nom::{
  IResult, Parser,
  character::complete::{char, line_ending, space0},
  combinator::map,
  multi::separated_list1,
  number::complete::float,
  sequence::delimited,
};
use std::collections::{HashMap, HashSet};

type JBox = Vec3;
type BoxId = usize;
type CircuitId = usize;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<usize> {
  let (_, boxes) = parse(_input).unwrap();
  find_final_connection(&boxes)
}

fn find_final_connection(boxes: &[JBox]) -> miette::Result<usize> {
  fn merge_circuits(map: &mut HashMap<BoxId, CircuitId>, keep: CircuitId, replace: CircuitId) {
    for circuit in map.values_mut() {
      if *circuit == replace {
        *circuit = keep;
      }
    }
  }

  fn all_in_one_circuit(map: &HashMap<BoxId, CircuitId>) -> bool {
    let unique_circuits: HashSet<_> = map.values().collect();
    unique_circuits.len() == 1
  }

  let pairs = sort_by_distance(boxes);
  let mut map: HashMap<BoxId, CircuitId> = HashMap::new();
  let mut circuit_ids = 0..;
  let total_boxes = boxes.len();

  for (lhs, rhs, _distance) in pairs.iter() {
    match (map.get(lhs), map.get(rhs)) {
      (None, None) => {
        let new_id = circuit_ids.next().unwrap();
        map.insert(*lhs, new_id);
        map.insert(*rhs, new_id);
      }
      (None, Some(&circuit_rhs)) => {
        map.insert(*lhs, circuit_rhs);
      }
      (Some(&circuit_lhs), None) => {
        map.insert(*rhs, circuit_lhs);
      }
      (Some(&circuit_lhs), Some(&circuit_rhs)) if circuit_lhs != circuit_rhs => {
        merge_circuits(&mut map, circuit_lhs, circuit_rhs);
      }
      _ => continue,
    };

    if map.len() == total_boxes && all_in_one_circuit(&map) {
      let x_lhs = boxes[*lhs].x as usize;
      let x_rhs = boxes[*rhs].x as usize;
      return Ok(x_lhs * x_rhs);
    }
  }

  Err(miette::miette!("Never merged into single circuit"))
}

fn sort_by_distance(boxes: &[JBox]) -> Vec<(BoxId, BoxId, f32)> {
  boxes
    .iter()
    .enumerate()
    .flat_map(|(i, box1)| {
      boxes
        .iter()
        .enumerate()
        .skip(i + 1)
        .map(move |(j, box2)| (i, j, box1.distance(*box2)))
    })
    .sorted_by(|a, b| a.2.partial_cmp(&b.2).unwrap())
    .collect()
}

fn parse(input: &str) -> IResult<&str, Vec<JBox>> {
  separated_list1(
    line_ending,
    delimited(
      space0,
      map(
        (float, char(','), float, char(','), float),
        |(a, _, b, _, c)| vec3(a, b, c),
      ),
      space0,
    ),
  )
  .parse(input)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_process() -> miette::Result<()> {
    let input = "162,817,812
                 57,618,57
                 906,360,560
                 592,479,940
                 352,342,300
                 466,668,158
                 542,29,236
                 431,825,988
                 739,650,466
                 52,470,668
                 216,146,977
                 819,987,18
                 117,168,530
                 805,96,715
                 346,949,466
                 970,615,88
                 941,993,340
                 862,61,35
                 984,92,344
                 425,690,689";
    assert_eq!(25272, process(input)?);
    Ok(())
  }
}

