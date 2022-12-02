use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(PartialEq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq)]
enum Context {
    NeedsToLoose,
    NeedsToDraw,
    NeedsToWin,
}

pub fn problem1(buffer: BufReader<File>) -> Result<String, String> {
    let score = get_score(buffer, false);
    Ok(score.to_string())
}

pub fn problem2(buffer: BufReader<File>) -> Result<String, String> {
    let score = get_score(buffer, true);
    Ok(score.to_string())
}

fn get_score(buffer: BufReader<File>, context_aware_choice: bool) -> u32 {
    let mut score: u32 = 0;
    for line_result in buffer.lines() {
        let line = line_result.unwrap();
        let mut split_iter = line.split_ascii_whitespace();
        let (other, me) = (split_iter.next().unwrap(), split_iter.next().unwrap());

        let opponent_choice = transform_input_to_choice(other);
        let my_choice = if context_aware_choice {
            transform_context_aware_to_choice(&opponent_choice, &transform_input_to_context(me))
        } else {
            transform_input_to_choice(me)
        };

        let won = is_win_for_me(&opponent_choice, &my_choice);
        let draw = is_draw(&opponent_choice, &my_choice);
        let score_for_line = calc_score(&my_choice, won, draw);
        score += score_for_line;
    }
    score
}

fn transform_input_to_context(input: &str) -> Context {
    match input {
        "X" => Context::NeedsToLoose,
        "Y" => Context::NeedsToDraw,
        "Z" => Context::NeedsToWin,
        _ => panic!("Unknown input"),
    }
}

fn transform_input_to_choice(input: &str) -> Choice {
    match input {
        "A" | "X" => Choice::Rock,
        "B" | "Y" => Choice::Paper,
        "C" | "Z" => Choice::Scissors,
        _ => panic!("Unknown input"),
    }
}

fn transform_context_aware_to_choice(opponent_choice: &Choice, context: &Context) -> Choice {
    match opponent_choice {
        Choice::Rock => match context {
            Context::NeedsToLoose => Choice::Scissors,
            Context::NeedsToDraw => Choice::Rock,
            Context::NeedsToWin => Choice::Paper,
        },
        Choice::Paper => match context {
            Context::NeedsToLoose => Choice::Rock,
            Context::NeedsToDraw => Choice::Paper,
            Context::NeedsToWin => Choice::Scissors,
        },
        Choice::Scissors => match context {
            Context::NeedsToLoose => Choice::Paper,
            Context::NeedsToDraw => Choice::Scissors,
            Context::NeedsToWin => Choice::Rock,
        },
    }
}

fn is_draw(other: &Choice, me: &Choice) -> bool {
    other == me
}

fn is_win_for_me(other: &Choice, me: &Choice) -> bool {
    match other {
        Choice::Rock => &Choice::Paper == me,
        Choice::Paper => &Choice::Scissors == me,
        Choice::Scissors => &Choice::Rock == me,
    }
}

fn calc_score(choice: &Choice, was_win: bool, was_draw: bool) -> u32 {
    let mut score: u32 = if was_win { 6 } else { 0 };
    score += if was_draw { 3 } else { 0 };
    match choice {
        Choice::Rock => score += 1,
        Choice::Paper => score += 2,
        Choice::Scissors => score += 3,
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

    #[test]
    fn day02_part2() {
        let path = "src/days/test-input/day02-example.txt".to_string();
        let buffer = read_file_to_buffer(path);
        let result = super::problem2(buffer);
        assert!(
            result.is_ok(),
            "Problem returned error: {}",
            result.unwrap_err()
        );
        assert_eq!(result.unwrap(), "12")
    }
}
