use std::collections::HashMap;

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

#[aoc(day1, part2, Chars)]
pub fn part2_chars(input: &str) -> u32 {
    input
        .lines()
        .map(|line| line.chars().collect::<String>())
        .map(|line| parse_word_numbers(&line))
        .sum()
}

fn char_offset_matches_word(char: char, offset: i32, word: &str) -> bool {
    word.chars().nth(offset.try_into().unwrap()) == Some(char)
}

fn word_number_to_digit(word: &str) -> u32 {
    let digit_map: HashMap<&str, u32> = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]
    .iter()
    .cloned()
    .collect();

    digit_map
        .get(word)
        .map_or_else(|| word.parse().unwrap(), |&v| v)
}

fn parse_word_numbers(input: &str) -> u32 {
    let initial_words = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]
    .into_iter()
    .map(|word| (word, 0))
    .collect::<Vec<_>>();

    let (first, last, _, _) = input.chars().fold(
        (None, None, 0, initial_words),
        |(first, last, i, number_word_offsets), char| {
            // handle case where the character is a number
            let (first, last) = match char.is_digit(10) {
                true if first.is_none() => (Some(char.to_string()), Some(char.to_string())),
                true => (first, Some(char.to_string())),
                false => (first, last),
            };

            // determine whether character completes word-number or
            // is on the correct offset to continue a word-number,
            // else starts a new word-number
            let (first, last, updated_number_word_offsets): (_, _, Vec<(&str, i32)>) =
                number_word_offsets.iter().fold(
                    (first, last, Vec::new()),
                    |(mut first, mut last, mut updated_offsets), &(word, offset)| {
                        if char_offset_matches_word(char, offset, word) {
                            if offset == (word.len() - 1).try_into().unwrap() {
                                if first.is_none() {
                                    first = Some(word.to_string());
                                }
                                last = Some(word.to_string());
                                updated_offsets.push((word, 0));
                            } else {
                                updated_offsets.push((word, offset + 1));
                            }
                        } else {
                            let is_first_char = char == word.chars().next().unwrap_or_default();
                            updated_offsets.push((word, if is_first_char { 1 } else { 0 }));
                        }

                        (first, last, updated_offsets)
                    },
                );

            (first, last, i + 1, updated_number_word_offsets)
        },
    );

    match (first, last) {
        (Some(first), Some(last)) => {
            word_number_to_digit(&first) * 10 + word_number_to_digit(&last)
        }
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::y2023::day1::{parse_calibration, parse_word_numbers};

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

    #[test]
    fn sample_word1() {
        assert_eq!(parse_word_numbers("threeight"), 38);
    }

    #[test]
    fn sample_word2() {
        assert_eq!(parse_word_numbers("fiveight"), 58);
    }

    #[test]
    fn sample_word3() {
        assert_eq!(parse_word_numbers("two1nine"), 29);
    }

    #[test]
    fn sample_word4() {
        assert_eq!(parse_word_numbers("eightwothree"), 83);
    }

    #[test]
    fn sample_word5() {
        assert_eq!(parse_word_numbers("abcone2threexyz"), 13);
    }

    #[test]
    fn sample_word6() {
        assert_eq!(parse_word_numbers("4nineight2"), 42);
    }

    #[test]
    fn sample_word7() {
        assert_eq!(parse_word_numbers("zoneight"), 18);
    }

    #[test]
    fn sample_word8() {
        assert_eq!(parse_word_numbers("7pqrstsixteen"), 76);
    }
}
