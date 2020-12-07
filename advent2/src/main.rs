extern crate failure;
extern crate regex;

use failure::Error;
use regex::Regex;
use std::fs;

#[derive(Debug)]
struct PolicyPassword {
    password: String,
    policy_char: char,
    pos1: usize,
    pos2: usize,
}

impl PolicyPassword {
    fn valid_old(&self) -> bool {
        let mut occur = 0;
        // Get Char Iterator
        for c in self.password.chars() {
            if c == self.policy_char {
                occur += 1;
            }
        }
        if occur <= self.pos2 && occur >= self.pos1 {
            return true;
        }
        false
    }

    fn valid(&self) -> bool {
        let mut chars = self.password.chars();
        let char1 = match chars.nth(self.pos1 - 1) {
            Some(s) => s,
            None => return false,
        };
        let mut chars = self.password.chars();
        let char2 = match chars.nth(self.pos2 - 1) {
            Some(s) => s,
            None => return false,
        };

        if (char1 == self.policy_char) == (char2 == self.policy_char) {
            return false;
        }
        true
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
                pos1: cap[1].parse::<usize>()?,
                pos2: cap[2].parse::<usize>()?,
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
    let mut valid_old_passes = 0;

    for p in input {
        if p.valid() {
            valid_passes += 1;
        }
        if p.valid_old() {
            valid_old_passes += 1;
        }
    }

    println!("Number of Valid Password: {}", valid_passes);
    println!("Number of Valid Old Password: {}", valid_old_passes);
}
