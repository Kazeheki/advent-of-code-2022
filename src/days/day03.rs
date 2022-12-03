use std::{fs::File, io::BufRead, io::BufReader};

pub fn problem1(buffer: BufReader<File>) -> Result<String, String> {
    let mut sum: u32 = 0;
    for line_result in buffer.lines() {
        let line = line_result.unwrap();
        let rucksack = line.as_str();
        let shared_item = find_shared_item(rucksack);
        let priority = char_to_priority(shared_item);
        sum += u32::from(priority);
    }
    Ok(sum.to_string())
}

pub fn problem2(_buffer: BufReader<File>) -> Result<String, String> {
    Err("Not implemented".to_string())
}

fn find_shared_item(rucksack: &str) -> char {
    let size = rucksack.len();
    let (left, right) = rucksack.split_at(size / 2);
    let mut left = left.chars().collect::<Vec<char>>();
    left.sort();
    left.dedup();
    let mut right = right.chars().collect::<Vec<char>>();
    right.sort();
    right.dedup();

    if left.len() > right.len() {
        for c in left {
            if right.contains(&c) {
                return c;
            }
        }
    } else {
        for c in right {
            if left.contains(&c) {
                return c;
            }
        }
    }
    unreachable!("There must be at one that is the same in each compartment");
}

fn char_to_priority(c: char) -> u8 {
    if c.is_lowercase() {
        (c as u8) - (char::from('a') as u8) + 1
    } else {
        (c as u8) - (char::from('A') as u8) + 27
    }
}

#[cfg(test)]
mod tests {
    use crate::days::read_file_to_buffer;

    #[test]
    fn day03_part1() {
        let path = "src/days/test-input/day03-example.txt".to_string();
        let buffer = read_file_to_buffer(path);
        let result = super::problem1(buffer);
        assert!(
            result.is_ok(),
            "Problem returned error: {}",
            result.unwrap_err()
        );
        assert_eq!(result.unwrap(), "157")
    }
}
