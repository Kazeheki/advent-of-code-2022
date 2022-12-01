use std::{
    cmp::max,
    fs::File,
    io::{BufRead, BufReader},
};

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

pub fn problem2(_buffer: BufReader<File>) -> Result<String, String> {
    Err("Not implemented".to_string())
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
}
