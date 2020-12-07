extern crate failure;
extern crate regex;

use failure::Error;
use regex::Regex;
use std::fs;

#[derive(Debug)]
struct PolicyPassword {
    password: String,
    policy_char: char,
    repeat_min: isize,
    repeat_max: isize,
}

impl PolicyPassword {
    fn valid(&self) -> bool {
        let mut occur = 0;
        // Get Char Iterator
        for c in self.password.chars() {
            if c == self.policy_char {
                occur += 1;
            }
        }
        if occur <= self.repeat_max && occur >= self.repeat_min {
            return true;
        }
        return false;
    }
}

fn get_input_data(filename: String) -> Result<Vec<PolicyPassword>, Error> {
    let mut pass_vec = vec![];

    let re = Regex::new(r"(\d+)-(\d+)\s(\w{1}):\s(\w+)$")?;

    for line in fs::read_to_string(filename)?.split("\n") {
        for cap in re.captures_iter(line) {
            let pass = PolicyPassword {
                password: cap[4].to_string(),
                policy_char: cap[3].chars().next().unwrap(),
                repeat_min: cap[1].parse::<isize>()?,
                repeat_max: cap[2].parse::<isize>()?,
            };
            pass_vec.push(pass);
        }
    }

    return Ok(pass_vec);
}

fn main() {
    let input = match get_input_data("input.txt".to_string()) {
        Err(e) => panic!("Error Reading Input {}", e),
        Ok(v) => v,
    };

    let mut valid_passes = 0;

    for p in input {
        if p.valid() {
            valid_passes += 1;
        }
    }

    println!("Number of Valid Password: {}", valid_passes);
}
