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

pub fn problem2(buffer: BufReader<File>) -> Result<String, String> {
    let mut sum: u32 = 0;
    let mut rucksacks: Vec<String> = Vec::new();
    for line_result in buffer.lines() {
        let line = line_result.unwrap();
        rucksacks.push(line);
        if rucksacks.len() == 3 {
            sum += u32::from(char_to_priority(find_group_item(&rucksacks)));
            rucksacks.clear();
        }
    }
    Ok(sum.to_string())
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

fn find_group_item(rucksacks: &Vec<String>) -> char {
    let mut r1: Vec<char> = rucksacks[0].chars().collect();
    r1.sort();
    r1.dedup();
    let mut r2: Vec<char> = rucksacks[1].chars().collect();
    r2.sort();
    r2.dedup();
    let mut r3: Vec<char> = rucksacks[2].chars().collect();
    r3.sort();
    r3.dedup();

    let mut same_in_1_and_2 = Vec::new();
    if r1.len() > r2.len() {
        for c in r1 {
            if r2.contains(&c) {
                same_in_1_and_2.push(c);
            }
        }
    } else {
        for c in r2 {
            if r1.contains(&c) {
                same_in_1_and_2.push(c);
            }
        }
    }

    for c in r3 {
        if same_in_1_and_2.contains(&c) {
            return c;
        }
    }
    unreachable!("there must be on same");
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

    #[test]
    fn day03_part2() {
        let path = "src/days/test-input/day03-example.txt".to_string();
        let buffer = read_file_to_buffer(path);
        let result = super::problem2(buffer);
        assert!(
            result.is_ok(),
            "Problem returned error: {}",
            result.unwrap_err()
        );
        assert_eq!(result.unwrap(), "70")
    }
}
