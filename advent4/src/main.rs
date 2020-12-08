extern crate failure;
extern crate regex;

use failure::Error;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn parse_passport_line(line: String, pattern: &Regex) -> HashMap<String, String> {
    let mut passport = HashMap::new();
    for cap in pattern.captures_iter(&line) {
        passport.insert(cap[1].to_string(), cap[2].to_string());
    }
    passport
}

fn check_passport_validity(
    passport: HashMap<String, String>,
    hcl_re: &Regex,
    hgt_re: &Regex,
) -> Result<bool, Error> {
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
            return Ok(false);
        }
        let val = passport.get(&f).unwrap();
        match f.as_str() {
            "byr" => {
                if val.parse::<usize>()? < 1920 || val.parse::<usize>()? > 2002 {
                    return Ok(false);
                }
            }
            "iyr" => {
                if val.parse::<usize>()? < 2010 || val.parse::<usize>()? > 2020 {
                    return Ok(false);
                }
            }
            "eyr" => {
                if val.parse::<usize>()? < 2020 || val.parse::<usize>()? > 2030 {
                    return Ok(false);
                }
            }
            "pid" => {
                if val.chars().count() != 9 {
                    return Ok(false);
                }
                // Parsing is enough. Error will mean invalid
                val.parse::<usize>()?;
            }
            "ecl" => {
                if !vec!["amb", "blu", "brn", "grn", "hzl", "oth", "gry"].contains(&val.as_str()) {
                    return Ok(false);
                }
            }
            "hcl" => {
                if !hcl_re.is_match(val) {
                    return Ok(false);
                }
            }
            "hgt" => {
                let captures = match hgt_re.captures(val) {
                    None => return Ok(false),
                    Some(t) => t,
                };
                if captures.name("unit").unwrap().as_str() == "in" {
                    if captures.name("hgt").unwrap().as_str().parse::<usize>()? < 59
                        || captures.name("hgt").unwrap().as_str().parse::<usize>()? > 76
                    {
                        return Ok(false);
                    }
                }
                if captures.name("unit").unwrap().as_str() == "cm" {
                    if captures.name("hgt").unwrap().as_str().parse::<usize>()? < 150
                        || captures.name("hgt").unwrap().as_str().parse::<usize>()? > 193
                    {
                        return Ok(false);
                    }
                }
            }
            _ => {
                continue;
            }
        }
    }
    Ok(true)
}

fn read_input_file(filename: String) -> Result<Vec<String>, Error> {
    let mut passport_lines = vec![];
    let contents = fs::read_to_string(filename)?;
    for l in contents.split("\n\n") {
        if l.is_empty() {
            continue;
        }
        passport_lines.push(l.to_string());
    }
    Ok(passport_lines)
}

fn main() {
    let re = Regex::new(r"(\w{3}):(\S+)").unwrap();
    let passport_lines = read_input_file("input.txt".to_string()).unwrap();
    let mut valid_count = 0;

    let hcl_regex = Regex::new(r"#[0-9a-f]{6}").unwrap();
    let hgt_regex = Regex::new(r"(?P<hgt>\d+)(?P<unit>\w{2})").unwrap();
    for passport in passport_lines {
        let pp = parse_passport_line(passport, &re);
        match check_passport_validity(pp, &hcl_regex, &hgt_regex) {
            Err(e) => println! {"Err:{}", e},
            Ok(b) => {
                if b {
                    valid_count += 1;
                }
            }
        }
    }
    println!("Valid Passports: {}", valid_count);
}
