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

fn unique_chars(input: &str) -> Vec<char> {
    let mut chars: Vec<char> = input.chars().collect();
    chars.sort();
    chars.dedup();
    chars
}

fn get_same_chars_in_both(a: &Vec<char>, b: &Vec<char>) -> Vec<char> {
    let bigger = if a.len() > b.len() { a } else { b };
    let smaller = if a.len() > b.len() { b } else { a };

    let mut same: Vec<char> = Vec::new();
    for c in bigger {
        if smaller.contains(c) {
            same.push(*c);
        }
    }
    same
}

fn find_shared_item(rucksack: &str) -> char {
    let size = rucksack.len();
    let (left, right) = rucksack.split_at(size / 2);
    let left = unique_chars(left);
    let right = unique_chars(right);

    let same = get_same_chars_in_both(&left, &right);
    same[0]
}

fn find_group_item(rucksacks: &Vec<String>) -> char {
    let r1 = unique_chars(rucksacks[0].as_str());
    let r2 = unique_chars(rucksacks[1].as_str());
    let r3 = unique_chars(rucksacks[2].as_str());

    let same_in_r1_and_r2 = get_same_chars_in_both(&r1, &r2);
    get_same_chars_in_both(&same_in_r1_and_r2, &r3)[0]
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
