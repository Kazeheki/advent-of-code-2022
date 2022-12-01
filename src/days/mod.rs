use std::{fs::File, io::BufReader};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

pub fn delegate_problem1(day: u8) -> Result<String, String> {
    let buffer: BufReader<File> = input_buffer_for_day(day);
    match day {
        1 => day01::problem1(buffer),
        2 => day02::problem1(buffer),
        3 => day03::problem1(buffer),
        4 => day04::problem1(buffer),
        5 => day05::problem1(buffer),
        6 => day06::problem1(buffer),
        7 => day07::problem1(buffer),
        8 => day08::problem1(buffer),
        9 => day09::problem1(buffer),
        10 => day10::problem1(buffer),
        11 => day11::problem1(buffer),
        12 => day12::problem1(buffer),
        13 => day13::problem1(buffer),
        14 => day14::problem1(buffer),
        15 => day15::problem1(buffer),
        16 => day16::problem1(buffer),
        17 => day17::problem1(buffer),
        18 => day18::problem1(buffer),
        19 => day19::problem1(buffer),
        20 => day20::problem1(buffer),
        21 => day21::problem1(buffer),
        22 => day22::problem1(buffer),
        23 => day23::problem1(buffer),
        24 => day24::problem1(buffer),
        25 => day25::problem1(buffer),
        _ => Err("Invalid day".to_string()),
    }
}

pub fn delegate_problem2(day: u8) -> Result<String, String> {
    let buffer: BufReader<File> = input_buffer_for_day(day);
    match day {
        1 => day01::problem2(buffer),
        2 => day02::problem2(buffer),
        3 => day03::problem2(buffer),
        4 => day04::problem2(buffer),
        5 => day05::problem2(buffer),
        6 => day06::problem2(buffer),
        7 => day07::problem2(buffer),
        8 => day08::problem2(buffer),
        9 => day09::problem2(buffer),
        10 => day10::problem2(buffer),
        11 => day11::problem2(buffer),
        12 => day12::problem2(buffer),
        13 => day13::problem2(buffer),
        14 => day14::problem2(buffer),
        15 => day15::problem2(buffer),
        16 => day16::problem2(buffer),
        17 => day17::problem2(buffer),
        18 => day18::problem2(buffer),
        19 => day19::problem2(buffer),
        20 => day20::problem2(buffer),
        21 => day21::problem2(buffer),
        22 => day22::problem2(buffer),
        23 => day23::problem2(buffer),
        24 => day24::problem2(buffer),
        25 => day25::problem2(buffer),
        _ => Err("Invalid day".to_string()),
    }
}

fn read_file_to_buffer(path: String) -> BufReader<File> {
    let file = File::open(path).expect("file does not exist");
    BufReader::new(file)
}

fn input_buffer_for_day(day: u8) -> BufReader<File> {
    let path: String = format!("input/{:02}.txt", day);
    read_file_to_buffer(path)
}
