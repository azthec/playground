use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day3)]
fn parse(input: &str) -> Vec<String> {
    input
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>()
}

fn adjacent_lines(input: &Vec<String>) -> Vec<(Option<String>, String, Option<String>)> {
    input
        .iter()
        .enumerate()
        .map(|(index, line)| {
            let previous_line = if index > 0 {
                Some(input[index - 1].clone())
            } else {
                None
            };
            let current_line = line.clone();
            let next_line = if index < input.len() - 1 {
                Some(input[index + 1].clone())
            } else {
                None
            };

            (previous_line, current_line, next_line)
        })
        .collect()
}

fn get_surrounding_range(
    line: Option<&str>,
    number_start: usize,
    number_end: usize,
) -> Option<(usize, usize)> {
    if let Some(line) = line {
        let start = if number_start > 0 {
            number_start - 1
        } else {
            0
        };
        let end = if number_end + 1 < line.len() {
            number_end + 1
        } else {
            line.len() - 1
        };
        Some((start, end))
    } else {
        None
    }
}

fn is_symbol(c: char) -> bool {
    !c.is_ascii_digit() && c != '.'
}

fn contains_symbol(maybe_line: Option<&str>, range: Option<(usize, usize)>) -> bool {
    match maybe_line {
        Some(line) => match range {
            Some((low, high)) => {
                line[low..=high].chars().any(is_symbol)
            }
            None => false,
        },
        None => false,
    }
}

fn range_contains_symbol(maybe_line: Option<&str>, start: usize, end: usize) -> bool {
    contains_symbol(maybe_line, get_surrounding_range(maybe_line, start, end))
}

fn is_part_number(
    previous_line: Option<&str>,
    current_line: &str,
    next_line: Option<&str>,
    number_start: usize,
    number_end: usize,
) -> bool {
    range_contains_symbol(previous_line, number_start, number_end)
        || range_contains_symbol(Some(current_line), number_start, number_end)
        || range_contains_symbol(next_line, number_start, number_end)
}

fn parse_number_from_str(s: &str, start: usize, end: usize) -> i32 {
    s[start..end].parse::<i32>().unwrap()
}

fn get_part_numbers(
    previous_line: Option<&str>,
    current_line: &str,
    next_line: Option<&str>,
) -> Vec<i32> {
    let mut numbers = Vec::new();
    let mut maybe_number_start = None;
    for (c_index, c) in current_line.chars().enumerate() {
        if c.is_ascii_digit() {
            if maybe_number_start.is_none() {
                maybe_number_start = Some(c_index);
            }
        } else if let Some(number_start) = maybe_number_start {
            let number_end = c_index - 1;
            if is_part_number(
                previous_line,
                current_line,
                next_line,
                number_start,
                number_end,
            ) {
                numbers.push(parse_number_from_str(current_line, number_start, c_index));
            }
            maybe_number_start = None;
        }
    }
    if let Some(number_start) = maybe_number_start {
        let number_end = current_line.len();
        if is_part_number(
            previous_line,
            current_line,
            next_line,
            number_start,
            number_end,
        ) {
            numbers.push(parse_number_from_str(
                current_line,
                number_start,
                number_end,
            ));
        }
    }
    numbers
}

#[aoc(day3, part1)]
fn part1(input: &Vec<String>) -> i32 {
    let adjacent_triples = adjacent_lines(&input);
    let numbers = adjacent_triples
        .iter()
        .flat_map(|(previous_line, current_line, next_line)| {
            get_part_numbers(previous_line.as_deref(), current_line, next_line.as_deref())
        })
        .collect::<Vec<i32>>();
    numbers.iter().sum::<i32>()
}

#[aoc(day3, part2)]
fn part2(_input: &Vec<String>) -> String {
    "NOT IMPLEMENTED".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_part_numbers() {
        let lines = vec![
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
        assert_eq!(part1(&lines), 4361);
    }

    #[test]
    fn is_part_number1() {
        let previous: String = ".#.....".to_string();
        let current: String = "..234..".to_string();
        let next: String = ".......".to_string();
        assert!(is_part_number(Some(&previous), &current, Some(&next), 2, 4));
    }

    #[test]
    fn is_part_number2() {
        let previous: String = ".......".to_string();
        let current: String = "012#...".to_string();
        let next: String = ".......".to_string();
        assert!(is_part_number(Some(&previous), &current, Some(&next), 0, 2));
    }

    #[test]
    fn is_part_number3() {
        let previous: String = ".......".to_string();
        let current: String = "....456".to_string();
        let next: String = "......#".to_string();
        assert!(is_part_number(Some(&previous), &current, Some(&next), 4, 6));
    }

    #[test]
    fn is_part_number4() {
        let previous: String = ".......".to_string();
        let current: String = "..234..".to_string();
        let next: String = ".#.....".to_string();
        assert!(is_part_number(Some(&previous), &current, Some(&next), 2, 4));
    }

    #[test]
    fn is_part_number5() {
        let current = "..395...".to_string();
        let next = "......#.".to_string();

        // let current: String = "..234..".to_string();
        // let next: String = "......#".to_string();
        assert!(!is_part_number(None, &current, Some(&next), 2, 4));
    }

    #[test]
    fn adjacent_lines_zero_lines() {
        let input: Vec<String> = vec![];
        assert_eq!(adjacent_lines(&input), vec![]);
    }

    #[test]
    fn adjacent_lines_one_lines() {
        let line1: String = "line1".to_string();
        let input: Vec<String> = vec![line1.clone()];
        assert_eq!(adjacent_lines(&input), vec![(None, line1.clone(), None)]);
    }

    #[test]
    fn adjacent_lines_two_lines() {
        let line1: String = "line1".to_string();
        let line2: String = "line2".to_string();
        let input: Vec<String> = vec![line1.clone(), line2.clone()];
        assert_eq!(
            adjacent_lines(&input),
            vec![
                (None, line1.clone(), Some(line2.clone())),
                (Some(line1.clone()), line2.clone(), None)
            ]
        );
    }

    #[test]
    fn adjacent_lines_three_lines() {
        let line1: String = "line1".to_string();
        let line2: String = "line2".to_string();
        let line3: String = "line3".to_string();
        let input: Vec<String> = vec![line1.clone(), line2.clone(), line3.clone()];
        assert_eq!(
            adjacent_lines(&input),
            vec![
                (None, line1.clone(), Some(line2.clone())),
                (Some(line1.clone()), line2.clone(), Some(line3.clone())),
                (Some(line2.clone()), line3.clone(), None)
            ]
        );
    }

    #[test]
    fn surrounding_range1() {
        assert_eq!(get_surrounding_range(Some("012345"), 1, 4), Some((0, 5)))
    }

    #[test]
    fn surrounding_range2() {
        assert_eq!(get_surrounding_range(Some("012345"), 2, 5), Some((1, 5)))
    }

    #[test]
    fn surrounding_range3() {
        assert_eq!(get_surrounding_range(Some("012345"), 3, 3), Some((2, 4)))
    }

    #[test]
    fn surrounding_range4() {
        assert_eq!(get_surrounding_range(None, 3, 3), None)
    }

    #[test]
    fn is_symbol1() {
        assert_eq!(is_symbol('*'), true)
    }

    #[test]
    fn is_symbol2() {
        assert_eq!(is_symbol('.'), false)
    }

    #[test]
    fn is_symbol3() {
        assert_eq!(is_symbol('3'), false)
    }

    #[test]
    fn contains_symbol1() {
        assert!(!contains_symbol(Some("*12345"), Some((1, 3))))
    }

    #[test]
    fn contains_symbol2() {
        assert!(contains_symbol(Some("01&345"), Some((0, 3))))
    }

    #[test]
    fn contains_symbol3() {
        assert!(!contains_symbol(Some("0123!5"), Some((0, 3))))
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&vec![]), 2);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&vec![]), "");
    }
}
