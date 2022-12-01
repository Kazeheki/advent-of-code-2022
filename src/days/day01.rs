use std::{
    cmp::max,
    fs::File,
    io::{BufRead, BufReader},
};

use colored::Colorize;

pub fn problem1(buffer: BufReader<File>) -> Result<String, String> {
    let mut max_value: u32 = 0;
    let mut tmp_sum: u32 = 0;
    for line_result in buffer.lines() {
        let line = line_result.unwrap();
        if line.trim().is_empty() {
            max_value = max(tmp_sum, max_value);
            tmp_sum = 0;
        } else {
            tmp_sum = tmp_sum + line.parse::<u32>().unwrap();
        }
    }
    Ok(max_value.to_string())
}

pub fn problem2(buffer: BufReader<File>) -> Result<String, String> {
    let mut top_three: Vec<u32> = vec![0; 3];
    let mut tmp_sum: u32 = 0;
    for line_result in buffer.lines() {
        let line = line_result.unwrap();
        if line.trim().is_empty() {
            if top_three[0] < tmp_sum {
                let tmp = top_three[0];
                top_three[0] = tmp_sum;
                tmp_sum = tmp;
            }
            if top_three[1] < tmp_sum {
                let tmp = top_three[1];
                top_three[1] = tmp_sum;
                tmp_sum = tmp;
            }
            if top_three[2] < tmp_sum {
                let tmp = top_three[2];
                top_three[2] = tmp_sum;
                tmp_sum = tmp;
            }
            tmp_sum = 0;
        } else {
            tmp_sum = tmp_sum + line.parse::<u32>().unwrap();
        }
    }

    if top_three[0] < tmp_sum {
        let tmp = top_three[0];
        top_three[0] = tmp_sum;
        tmp_sum = tmp;
    }
    if top_three[1] < tmp_sum {
        let tmp = top_three[1];
        top_three[1] = tmp_sum;
        tmp_sum = tmp;
    }
    if top_three[2] < tmp_sum {
        let tmp = top_three[2];
        top_three[2] = tmp_sum;
        tmp_sum = tmp;
    }
    let sum: u32 = top_three.iter().sum();
    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use crate::days::read_file_to_buffer;

    #[test]
    fn day01_part1() {
        let path = "src/days/test-input/day01-example.txt".to_string();
        let buffer = read_file_to_buffer(path);
        let result = super::problem1(buffer);
        assert!(
            result.is_ok(),
            "Problem returned error: {}",
            result.unwrap_err()
        );
        assert_eq!(result.unwrap(), "24000")
    }

    #[test]
    fn day01_part2() {
        let path = "src/days/test-input/day01-example.txt".to_string();
        let buffer = read_file_to_buffer(path);
        let result = super::problem2(buffer);
        assert!(
            result.is_ok(),
            "Problem returned error: {}",
            result.unwrap_err()
        );
        assert_eq!(result.unwrap(), "45000")
    }
}
