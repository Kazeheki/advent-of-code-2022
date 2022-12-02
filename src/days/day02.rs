use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use colored::Colorize;

pub fn problem1(buffer: BufReader<File>) -> Result<String, String> {
    let score = get_score(buffer);
    Ok(score.to_string())
}

pub fn problem2(_buffer: BufReader<File>) -> Result<String, String> {
    Err("Not implemented".to_string())
}

fn get_score(buffer: BufReader<File>) -> u32 {
    let mut score: u32 = 0;
    for line_result in buffer.lines() {
        let line = line_result.unwrap();
        let mut split_iter = line.split_ascii_whitespace();
        let (other, me) = (split_iter.next().unwrap(), split_iter.next().unwrap());
        let won = is_win_for_me(other, me);
        let draw = is_draw(other, me);
        let score_for_line = calc_score(me, won, draw);
        score += score_for_line;
    }
    score
}

fn is_draw(other: &str, me: &str) -> bool {
    match other {
        "A" => me == "X",
        "B" => me == "Y",
        "C" => me == "Z",
        _ => panic!("Unknown input"),
    }
}

fn is_win_for_me(other: &str, me: &str) -> bool {
    match other {
        "A" => me == "Y",
        "B" => me == "Z",
        "C" => me == "X",
        _ => panic!("Unknown input"),
    }
}

fn calc_score(choice: &str, was_win: bool, was_draw: bool) -> u32 {
    let mut score: u32 = if was_win { 6 } else { 0 };
    score += if was_draw { 3 } else { 0 };
    match choice {
        "X" => score += 1,
        "Y" => score += 2,
        "Z" => score += 3,
        _ => panic!("Unknown input"),
    }
    score
}

#[cfg(test)]
mod tests {
    use crate::days::read_file_to_buffer;

    #[test]
    fn day02_part1() {
        let path = "src/days/test-input/day02-example.txt".to_string();
        let buffer = read_file_to_buffer(path);
        let result = super::problem1(buffer);
        assert!(
            result.is_ok(),
            "Problem returned error: {}",
            result.unwrap_err()
        );
        assert_eq!(result.unwrap(), "15")
    }
}
