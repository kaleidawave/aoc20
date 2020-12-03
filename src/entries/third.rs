use std::fs;

/// Day 3 - Part 1
/// https://adventofcode.com/2020/day/3
/// Read in wrapping map, given a direction count the number of trees (denoted with a #)
/// the toboggan encounters
pub fn part1() {
    let direction = (1, 3);
    println!("Tree collisions: {}", toboggan_hill(&direction));
}

/// Day 3 - Part 2
/// https://adventofcode.com/2020/day/3#part2
/// Part 1 but do for several directions. Take product of collisions
pub fn part2() {
    let directions = vec![
        (1, 1),
        (1, 3),
        (1, 5),
        (1, 7),
        (2, 1)
    ];
    
    let collisions_product = directions
        .iter()
        .map(|dir| toboggan_hill(dir))
        .fold(1, |acc, x| acc * x);

    println!("Product of tree collisions: {}", collisions_product); 
}

/// Toboggan hill return the number of collisions with trees
pub fn toboggan_hill(direction: &(usize, usize)) -> usize {
    let mut collisions = 0;
    let map = fs::read_to_string("files/3-input.txt").expect("Could not find files/3-input.txt");
    let width = map.lines().next().expect("Expected initial line").len();
    let mut x = 0;
    let rows = map
        .lines()
        .enumerate()
        .filter_map(|(pos, value)| 
            if pos % direction.0 == 0 {
                Some(value)
            } else {
                None
            }
        );

    for row in rows {
        if row.chars().nth(x).expect("Off the map") == '#' {
            collisions += 1;
        }
        x = (x + direction.1) % width;
    }
    collisions
}