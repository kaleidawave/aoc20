use std::fs;
use itertools::Itertools;

/// Day 1 - Part 1 
/// Read in list and find the two entries which sum is equal to 2020
/// Return the product of these
pub fn part1() {
    println!("Product is: {}", find_num(2));
}

/// Day 1 - Part 2
/// Part 1 but with THREE entries
pub fn part2() {
    println!("Product is: {}", find_num(3));
}

pub fn find_num(num_combinations: usize) -> i32 {
    let target = 2020;
    let number_string = fs::read_to_string("files/1-input.txt").expect("Could not find files/1-input.txt");

    number_string
        .split("\n")
        .filter_map(|entry| 
            entry.parse::<i32>().ok()
        )
        .combinations(num_combinations)
        .find(|comb|
            comb.iter().fold(0, |acc, x| acc + x) == target 
        )
        .map(|found_comb| 
            found_comb.iter().fold(1, |acc, x| acc * x)
        )
        .expect("Could not find numbers to product target")
}