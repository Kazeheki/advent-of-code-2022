use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn problem1(buffer: BufReader<File>) -> Result<String, String> {
    let top_three = get_top_three_calories(buffer);
    Ok(top_three[0].to_string())
}

pub fn problem2(buffer: BufReader<File>) -> Result<String, String> {
    let top_three = get_top_three_calories(buffer);
    let sum: u32 = top_three.iter().sum();
    Ok(sum.to_string())
}

fn adjust_top_three(top_three: &mut Vec<u32>, current_sum: u32) {
    let mut sum = current_sum;
    if top_three[0] < sum {
        let tmp = top_three[0];
        top_three[0] = sum;
        sum = tmp;
    }
    if top_three[1] < sum {
        let tmp = top_three[1];
        top_three[1] = sum;
        sum = tmp;
    }
    if top_three[2] < sum {
        top_three[2] = sum;
    }
}

fn get_top_three_calories(buffer: BufReader<File>) -> Vec<u32> {
    let mut top_three: Vec<u32> = vec![0; 3];
    let mut tmp_sum: u32 = 0;
    for line_result in buffer.lines() {
        let line = line_result.unwrap();
        if line.trim().is_empty() {
            adjust_top_three(&mut top_three, tmp_sum);
            tmp_sum = 0;
        } else {
            tmp_sum = tmp_sum + line.parse::<u32>().unwrap();
        }
    }
    adjust_top_three(&mut top_three, tmp_sum);
    top_three
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
