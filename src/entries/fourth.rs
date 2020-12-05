use std::fs;
use std::collections::HashMap;
use regex::Regex;

/// Day 4 - Part 1
/// https://adventofcode.com/2020/day/4
pub fn part1() {
    let passports = get_passports();
    let expected_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        
    let valid_passports = passports
        .iter()
        .filter(|passport| 
            expected_fields.iter().all(|field_name| passport.contains_key(&field_name.to_string()))
        )
        .count();

    println!("Valid passports: {}", valid_passports);
}

/// Day 4 - Part 2
/// https://adventofcode.com/2020/day/4#part2
pub fn part2() {
    let passports = get_passports();
    let expected_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        
    let valid_passports = passports
        .iter()
        .filter(|passport| 
            expected_fields.iter().all(|field_name| passport.contains_key(&field_name.to_string())) &&
            passport_valid(passport)
        )
        .count();

    println!("Valid passports: {}", valid_passports);
}

fn passport_valid(passport: &HashMap<String, String>) -> bool {
    let byr = passport.get("byr").unwrap().parse::<i32>().unwrap();
    if !(1920..=2002).contains(&byr) {
        return false;
    }
    let iyr = passport.get("iyr").unwrap().parse::<i32>().unwrap();
    if !(2010..=2020).contains(&iyr) {
        return false;
    }
    let eyr = passport.get("eyr").unwrap().parse::<i32>().unwrap();
    if !(2020..=2030).contains(&eyr) {
        return false;
    }
    let hgt = passport.get("hgt").unwrap();
    let matches = Regex::new(r"(\d+)(cm|in)").unwrap().captures(hgt);
    if matches.is_none() {
        return false;
    }
    let matches = matches.unwrap();
    let hgt_value = matches.get(1).unwrap().as_str().parse::<i32>();
    if hgt_value.is_err() {
        return false;
    }
    let hgt_value = hgt_value.unwrap();
    let hgt_unit = matches.get(2).unwrap().as_str();
    if hgt_unit == "cm" && !(150..=193).contains(&hgt_value) {
        return false;
    } else if hgt_unit == "in" && !(59..=76).contains(&hgt_value) {
        return false;
    }
    let hcl = passport.get("hcl").unwrap();
    if hcl.len() != 7 || !Regex::new(r"#[a-f0-9]{6}").unwrap().is_match(hcl) {
        return false;
    }
    let valid_eye_colors = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    if !valid_eye_colors.iter().any(|color| color == &passport.get("ecl").unwrap()) {
        return false;
    }
    let pid = passport.get("pid").unwrap();
    if pid.len() != 9 || !pid.chars().all(|digit| digit.is_digit(10)) {
        return false;
    }
    true
}

fn get_passports() -> Vec<HashMap<String, String>> {
    let passports_string = fs::read_to_string("files/input-4.txt").expect("Could not find files/input-4.txt");
    let mut cur_passport = HashMap::new();
    let mut passports = Vec::new();
    for line in passports_string.lines() {
        if line.is_empty() {
            passports.push(cur_passport);
            cur_passport = HashMap::new();
        } else {
            for pair in line.split(" ").map(|pair| pair.split(":").collect::<Vec<_>>()) {
                cur_passport.insert(pair[0].to_string(), pair[1].to_string());
            }
        }
    }
    passports
}