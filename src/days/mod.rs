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
    match day {
        1 => day01::problem1(),
        2 => day02::problem1(),
        3 => day03::problem1(),
        4 => day04::problem1(),
        5 => day05::problem1(),
        6 => day06::problem1(),
        7 => day07::problem1(),
        8 => day08::problem1(),
        9 => day09::problem1(),
        10 => day10::problem1(),
        11 => day11::problem1(),
        12 => day12::problem1(),
        13 => day13::problem1(),
        14 => day14::problem1(),
        15 => day15::problem1(),
        16 => day16::problem1(),
        17 => day17::problem1(),
        18 => day18::problem1(),
        19 => day19::problem1(),
        20 => day20::problem1(),
        21 => day21::problem1(),
        22 => day22::problem1(),
        23 => day23::problem1(),
        24 => day24::problem1(),
        25 => day25::problem1(),
        _ => Err("Invalid day".to_string()),
    }
}

pub fn delegate_problem2(day: u8) -> Result<String, String> {
    match day {
        1 => day01::problem2(),
        2 => day02::problem2(),
        3 => day03::problem2(),
        4 => day04::problem2(),
        5 => day05::problem2(),
        6 => day06::problem2(),
        7 => day07::problem2(),
        8 => day08::problem2(),
        9 => day09::problem2(),
        10 => day10::problem2(),
        11 => day11::problem2(),
        12 => day12::problem2(),
        13 => day13::problem2(),
        14 => day14::problem2(),
        15 => day15::problem2(),
        16 => day16::problem2(),
        17 => day17::problem2(),
        18 => day18::problem2(),
        19 => day19::problem2(),
        20 => day20::problem2(),
        21 => day21::problem2(),
        22 => day22::problem2(),
        23 => day23::problem2(),
        24 => day24::problem2(),
        25 => day25::problem2(),
        _ => Err("Invalid day".to_string()),
    }
}

fn read_file_to_buffer(path: String) -> BufReader<File> {
    let file = File::open(path).expect("file does not exist");
    BufReader::new(file)
}
