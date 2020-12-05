use std::fs;
use std::collections::HashSet;
use std::iter::FromIterator;

/// Day 5 - Part 1
/// https://adventofcode.com/2020/day/5
pub fn part1() {
    println!("Highest seat id {}", get_seat_codes().iter().max().unwrap());
}

/// Day 5 - Part 2
/// https://adventofcode.com/2020/day/5#part2
pub fn part2() {
    let seats: HashSet<usize> = HashSet::from_iter(get_seat_codes().iter().cloned());
    let vacant_seat = seats.iter().find(|seat_id|
        !seats.contains(&(**seat_id + 1)) && seats.contains(&(**seat_id + 2))
    ).unwrap() + 1;
    println!("Vacant seat id {}", vacant_seat);
}

fn get_seat_codes() -> Vec<usize> {
    let input = fs::read_to_string("files/input-5.txt").expect("Could not find files/input-5.txt");
    let rows = input.split("\n").filter(|row| !row.is_empty());
    let mut seats: Vec<(usize, usize)> = Vec::new();

    for row in rows {
        let mut row_idx = 0;
        for (stage, row_code) in row.chars().take(7).enumerate() {
            if row_code == 'B' {
                row_idx += 2_usize.pow((6 - stage) as u32);
            }
        }
        let mut column_idx = 0;
        for (stage, column_code) in row.chars().skip(7).take(3).enumerate() {
            if column_code == 'R' {
                column_idx += 2_usize.pow((2 - stage) as u32);
            }
        }
        seats.push((row_idx, column_idx));
    }
    seats.iter().map(|seat| (seat.0 * 8) + seat.1).collect::<Vec<usize>>()
}