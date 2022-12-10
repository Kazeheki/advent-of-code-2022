use std::{fs::File, io::BufReader};

pub fn problem1(_buffer: BufReader<File>) -> Result<String, String> {
    Err("Not implemented".to_string())
}

pub fn problem2(_buffer: BufReader<File>) -> Result<String, String> {
    Err("Not implemented".to_string())
}

#[cfg(test)]
mod tests {
    use crate::days::read_file_to_buffer;

    const PATH: &str = "src/days/test-input/day10-example.txt";

    #[test]
    fn day10_part1() {
        let expected: Option<&str> = Some("13140");
        let buffer = read_file_to_buffer(PATH.to_string());
        let result = super::problem1(buffer);
        assert!(
            result.is_ok(),
            "Problem returned error: {}",
            result.unwrap_err()
        );
        assert_eq!(result.unwrap(), expected.unwrap());
    }

    #[allow(dead_code)]
    fn day10_part2() {
        let expected: Option<&str> = None;
        let buffer = read_file_to_buffer(PATH.to_string());
        let result = super::problem2(buffer);
        assert!(
            result.is_ok(),
            "Problem returned error: {}",
            result.unwrap_err()
        );
        assert_eq!(result.unwrap(), expected.unwrap());
    }
}
