use regex::Regex;
use std::fs;

/// Day 2 - Part 1
/// https://adventofcode.com/2020/day/2
/// Read in list and from syntax find how many lines are valid
/// Line is valid if character appears between a and b times in the sequence
pub fn part1() {
    let mut valid = 0;
    let list = fs::read_to_string("files/2-input.txt").expect("Could not find files/2-input.txt");

    let match_expression =
        Regex::new(r"(?P<start>\d+)-(?P<end>\d+) (?P<character>.): (?P<sequence>.+)").unwrap();

    for line in list.lines() {
        let matches = match_expression.captures(line);
        if let Some(matches) = matches {
            let start = &matches["start"]
                .parse::<i32>()
                .expect("Invalid start character");
            let end = &matches["end"]
                .parse::<i32>()
                .expect("Invalid end character");
            let character = &matches["character"]
                .chars()
                .next()
                .expect("Expected character");
            let sequence = &matches["sequence"];
            let mut matches_of_char = 0;
            for sequence_char in sequence.chars() {
                if &sequence_char == character {
                    matches_of_char += 1;
                }
            }

            if &matches_of_char >= start && &matches_of_char <= end {
                valid += 1;
            }
        } else {
            continue;
        }
    }
    println!("Valid lines: {}", valid);
}

/// Day 2 - Part 2
/// https://adventofcode.com/2020/day/2#part2
/// Read in list and from syntax find how many lines are valid
/// Line is valid if character appears exactly one time at a or b in the sequence
pub fn part2() {
    let mut valid = 0;
    let list = fs::read_to_string("files/2-input.txt").expect("Could not find files/2-input.txt");

    let match_expression =
        Regex::new(r"(?P<position1>\d+)-(?P<position2>\d+) (?P<character>.): (?P<sequence>.+)")
            .unwrap();

    for line in list.lines() {
        let matches = match_expression.captures(line);
        if let Some(matches) = matches {
            let position1 = &matches["position1"]
                .parse::<i32>()
                .expect("Invalid pos1 character");
            let position2 = &matches["position2"]
                .parse::<i32>()
                .expect("Invalid pos2 character");
            let character = &matches["character"]
                .chars()
                .next()
                .expect("Expected character");
            let sequence = &matches["sequence"];

            let mut matches_of_char = 0;
            for (pos, sequence_char) in sequence.chars().enumerate() {
                if (pos + 1 == *position1 as usize || pos + 1 == *position2 as usize)
                    && &sequence_char == character
                {
                    matches_of_char += 1;
                }
            }
            if matches_of_char == 1 {
                valid += 1;
            }
        } else {
            continue;
        }
    }
    println!("Valid lines: {}", valid);
}
