extern crate failure;
extern crate regex;

use failure::Error;
use regex::Regex;
use std::fs;
use std::collections::HashMap;

fn parse_passport_line(line: String, pattern: &Regex) -> HashMap<String, String> {
    let mut passport = HashMap::new();
    for cap in pattern.captures_iter(&line) {
        passport.insert(cap[1].to_string(), cap[2].to_string());
    }
    passport
}

fn check_passport_validity(passport: HashMap<String, String>) -> bool {
    let required_fields = vec![
        String::from("byr"),
        String::from("iyr"),
        String::from("pid"),
        String::from("ecl"),
        String::from("hcl"),
        String::from("eyr"),
        String::from("hgt"),
    ];
    for f in required_fields {
        if !passport.contains_key(&f) {
            return false
        }
    }
    true
}

fn read_input_file(filename: String) -> Result<Vec<String>, Error> {
    let mut passport_lines = vec![];
    let contents = fs::read_to_string(filename)?;
    for l in contents.split("\n\n") {
        if l.is_empty(){
            continue
        }
        passport_lines.push(l.to_string());
    }
    Ok(passport_lines)
}

fn main() {
    let re = Regex::new(r"(\w{3}):(\S+)").unwrap();
    let passport_lines = read_input_file("input.txt".to_string()).unwrap();

    let mut valid_count = 0;
    for passport in passport_lines {
        let pp = parse_passport_line(passport, &re);
        if check_passport_validity(pp) {
            valid_count += 1;
        }
    }
    println!("Valid Passports: {}", valid_count);
}
