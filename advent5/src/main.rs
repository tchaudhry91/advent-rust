use std::fs;

use eyre::{Error, Result};

#[derive(Debug)]
struct Partition {
    start: u8,
    end: u8,
}

impl Partition {
    fn new(start: u8, end: u8) -> Partition {
        Partition { start, end }
    }

    fn rows() -> Partition {
        Partition { start: 0, end: 127 }
    }

    fn columns() -> Partition {
        Partition { start: 0, end: 7 }
    }

    fn split_lower(&self) -> Partition {
        Partition::new(self.start, (self.start + self.end) / 2)
    }

    fn split_upper(&self) -> Partition {
        Partition::new((self.start + self.end + 1) / 2, self.end)
    }
}
struct Seat {
    row: u8,
    column: u8,
}

impl Seat {
    fn new(row: u8, column: u8) -> Seat {
        Seat { row, column }
    }

    fn get_id(&self) -> u16 {
        ((self.row as u16 * 8) + self.column as u16) as u16
    }
}

fn decode_row(code: String) -> Result<u8> {
    let mut p = Partition::rows();
    let char_vec: Vec<char> = code.chars().collect();
    if char_vec.len() < 10 {
        return Err(Error::msg("Invalid Code Supplied. Length must be 10"));
    }
    for c in &char_vec[0..7] {
        if *c == 'F' {
            p = p.split_lower();
        } else if *c == 'B' {
            p = p.split_upper();
        }
    }
    // By the end, start and end should be equal
    if p.start != p.end {
        return Err(Error::msg(
            "Unable to decode the row, please check the input",
        ));
    }
    Ok(p.start)
}

fn decode_column(code: String) -> Result<u8> {
    let mut p = Partition::columns();
    let char_vec: Vec<char> = code.chars().collect();
    if char_vec.len() < 10 {
        return Err(Error::msg("Invalid Code Supplied. Length must be 10"));
    }
    for c in &char_vec[7..=9] {
        if *c == 'R' {
            p = p.split_upper();
        } else if *c == 'L' {
            p = p.split_lower();
        }
    }
    Ok(p.start)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split() {
        let p = Partition::new(0, 127);
        let p_lower = p.split_lower();
        assert_eq!(p_lower.start, 0);
        assert_eq!(p_lower.end, 63);
        let p_upper = p.split_upper();
        assert_eq!(p_upper.start, 64);
        assert_eq!(p_upper.end, 127);
    }

    #[test]
    fn test_decode() {
        let test_code: String = String::from("FBFBBFFRLR");
        assert_eq!(decode_row(test_code.clone()).unwrap(), 44);
        assert_eq!(decode_column(test_code).unwrap(), 5);
    }

    #[test]
    fn test_seat_id() {
        let seat = Seat::new(44, 5);
        assert_eq!(seat.get_id(), 357);
    }
}

fn get_boarding_passes(filename: String) -> Result<Vec<String>> {
    let mut bps: Vec<String> = vec![];
    for line in fs::read_to_string(filename)?.split('\n') {
        if line.len() != 10 {
            continue;
        }
        bps.push(line.to_string());
    }
    Ok(bps)
}

fn main() {
    let bps = match get_boarding_passes("input".to_string()) {
        Err(e) => panic!("Error Reading Boarding Passes {}", e),
        Ok(v) => v,
    };
    let mut highest_id = 0;
    let mut lowest_id = 1000;
    let mut all_ids: Vec<u16> = vec![];
    for b in bps {
        let row = decode_row(b.clone()).unwrap();
        let column = decode_column(b.clone()).unwrap();
        let s = Seat::new(row, column);
        if s.get_id() > highest_id {
            highest_id = s.get_id();
        }
        if s.get_id() < lowest_id {
            lowest_id = s.get_id();
        }
        all_ids.push(s.get_id());
    }
    println!("The Higest ID is {}", highest_id);

    for i in lowest_id..=highest_id {
        if !all_ids.iter().any(|&id| id == i) {
            println!("Gap found at {}", i);
            if all_ids.iter().any(|&id| id == i - 1) && all_ids.iter().any(|&id| id == i + 1) {
                println!("ID is viable: {}", i);
            }
        }
    }
}
