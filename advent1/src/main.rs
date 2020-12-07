use std::fs;
extern crate failure;

use failure::{err_msg, Error};

fn find_two_to_sum(mut nums: Vec<i32>, sum: i32) -> Result<(i32, i32), Error> {
    // Algorithm is horribly inefficient, but that's not the point right now :)
    loop {
        let num = match nums.pop() {
            None => break,
            Some(x) => x,
        };
        for i in &nums {
            if num + i == sum {
                return Ok((num, *i));
            }
        }
    }
    return Err(err_msg("No Combination Found"));
}

// advent-2
fn find_three_to_sum(nums: Vec<i32>, sum: i32) -> Result<(i32, i32, i32), Error> {
    // Again, slower than a snail, but it's okay
    for i in 0..nums.len() {
        for j in i..nums.len() {
            for k in j..nums.len() {
                if nums[i] + nums[j] + nums[k] == sum {
                    return Ok((nums[i], nums[j], nums[k]));
                }
            }
        }
    }
    return Err(err_msg("No Combination Found"));
}

fn get_input_vec(filename: String) -> Result<Vec<i32>, Error> {
    let contents = fs::read_to_string(filename)?;

    let mut nums: Vec<i32> = vec![];

    for num_str in contents.split("\n") {
        if num_str.is_empty() {
            continue;
        }
        let n = num_str.parse::<i32>()?;
        nums.push(n);
    }
    Ok(nums)
}

#[cfg(test)]
mod tests {
    use super::find_two_to_sum;
    use super::find_three_to_sum;

    #[test]
    fn test_sum() {
        let sample_vec = vec![1721, 979, 366, 299, 675, 1456];
        let (x, y) = find_two_to_sum(sample_vec.clone(), 2020).unwrap();
        assert_eq!(514579, x * y);

        let (x, y, z) = find_three_to_sum(sample_vec.clone(), 2020).unwrap();
        assert_eq!(241861950, x*y*z);

        let sample_vec2 = vec![1720, 979, 366, 299, 675, 1456];
        assert_eq!(true, find_two_to_sum(sample_vec2, 2020).is_err());
    }
}

fn main() {
    let nums = get_input_vec("input.txt".to_string()).expect("Unable to read the input file");
    let (x, y) = match find_two_to_sum(nums.clone(), 2020) {
        Ok(t) => t,
        Err(_e) => panic! {"No Combination Found!"},
    };
    println!("Two Nums: {}", x * y);

    let (x, y, z) = match find_three_to_sum(nums.clone(), 2020) {
        Ok(t) => t,
        Err(_e) => panic! {"No Combination Found for Three Numbers!"},
    };
    println!("Three Nums: {}", x * y *z);

}
