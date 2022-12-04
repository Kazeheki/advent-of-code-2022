use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn problem1(buffer: BufReader<File>) -> Result<String, String> {
    let mut sections: Vec<(Section, Section)> = vec![];
    for line_result in buffer.lines() {
        let line = line_result.unwrap();
        let section_pair = line
            .split(",")
            .map(|s| Section::from(s))
            .collect::<Vec<Section>>();
        sections.push((section_pair[0], section_pair[1]));
    }
    let count = sections
        .iter()
        .map(|pair| {
            let (a, b) = pair;
            (a.min >= b.min && a.max <= b.max) || (b.min >= a.min && b.max <= a.max)
        })
        .filter(|b| *b)
        .count();
    Ok(count.to_string())
}

pub fn problem2(_buffer: BufReader<File>) -> Result<String, String> {
    Err("Not implemented".to_string())
}

#[derive(Clone, Copy)]
struct Section {
    min: u32,
    max: u32,
}

impl From<&str> for Section {
    fn from(s: &str) -> Self {
        s.split("-")
            .collect::<Vec<&str>>()
            .chunks(2)
            .map(|a| Section {
                min: a[0].parse::<u32>().unwrap(),
                max: a[1].parse::<u32>().unwrap(),
            })
            .collect::<Vec<Section>>()[0]
    }
}

#[cfg(test)]
mod tests {
    use crate::days::read_file_to_buffer;

    #[test]
    fn day04_part1() {
        let path = "src/days/test-input/day04-example.txt".to_string();
        let buffer = read_file_to_buffer(path);
        let result = super::problem1(buffer);
        assert!(
            result.is_ok(),
            "Problem returned error: {}",
            result.unwrap_err()
        );
        assert_eq!(result.unwrap(), "2")
    }

    // #[test]
    #[allow(dead_code)]
    fn day04_part2() {
        unimplemented!("no part 2 yet");
        // let path = "src/days/test-input/day04-example.txt".to_string();
        // let buffer = read_file_to_buffer(path);
        // let result = super::problem2(buffer);
        // assert!(
        //     result.is_ok(),
        //     "Problem returned error: {}",
        //     result.unwrap_err()
        // );
        // assert_eq!(result.unwrap(), "")
    }
}
