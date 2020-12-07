use std::fs;
extern crate failure;

use failure::{Error, err_msg};

fn find_sum_twenty_twenty(mut nums: Vec<i32>) -> Result<(i32, i32), Error> {
    loop {
        let num = match nums.pop(){
            None => break,
            Some(x) => x,
        };
        for i in &nums {
            if num + i == 2020 {
                return Ok((num, *i))
            }
        }

    }
    return Err(err_msg("No Combination Found"))
}

fn get_input_vec(filename: String) -> Result<Vec<i32>, Error> {
    let contents = fs::read_to_string(filename)?;

    let mut nums: Vec<i32> = vec![];

    for num_str in contents.split("\n") {
        if num_str.is_empty() {
            continue
        }
        let n = num_str.parse::<i32>()?;
        nums.push(n);
    }
    Ok(nums)

}

#[cfg(test)]
mod tests{
    use super::find_sum_twenty_twenty;

    #[test]
    fn test_sum() {
        let sample_vec = vec![1721, 979, 366, 299, 675, 1456];
        let (x, y) = find_sum_twenty_twenty(sample_vec).unwrap();
        assert_eq!(514579, x*y);

        let sample_vec2 = vec![1720, 979, 366, 299, 675, 1456];
        assert_eq!(true, find_sum_twenty_twenty(sample_vec2).is_err());
    }
}

fn main() {
    let nums = get_input_vec("input.txt".to_string()).expect("Unable to read the input file");
    let (x,y) = match find_sum_twenty_twenty(nums) {
        Ok(t) => t,
        Err(_e) => panic!{"No Combination Found!"}
    };
    println!("{}", x*y)
}
