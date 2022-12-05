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

    #[test]
    fn day05_part1() {
        let path = "src/days/test-input/day05-example.txt".to_string();
        let buffer = read_file_to_buffer(path);
        let result = super::problem1(buffer);
        assert!(
            result.is_ok(),
            "Problem returned error: {}",
            result.unwrap_err()
        );
        assert_eq!(result.unwrap(), "CMZ");
    }

    #[allow(dead_code)]
    fn day05_part2() {
        let path = "src/days/test-input/day05-example.txt".to_string();
        let buffer = read_file_to_buffer(path);
        let result = super::problem2(buffer);
        assert!(
            result.is_ok(),
            "Problem returned error: {}",
            result.unwrap_err()
        );
        assert_eq!(result.unwrap(), "");
    }
}
