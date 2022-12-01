use clap::Parser;
use colored::Colorize;

mod days;

#[derive(clap::Parser)]
struct Args {
    #[arg(short, long)]
    day: u8,

    #[arg(short, long, value_enum, default_value_t = Mode::ALL)]
    mode: Mode,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum Mode {
    ALL,
    PROBLEM1,
    PROBLEM2,
}

fn main() {
    let arguments = Args::parse();

    let mut results: Vec<Result<String, String>> = vec![];

    match arguments.mode {
        Mode::ALL => {
            let result1 = problem1(arguments.day);
            let result2 = problem2(arguments.day);
            results.push(result1);
            results.push(result2);
        }
        Mode::PROBLEM1 => results.push(problem1(arguments.day)),
        Mode::PROBLEM2 => results.push(problem2(arguments.day)),
    };

    for (idx, result) in results.iter().enumerate() {
        println!("{} {}", "Part".yellow(), (idx + 1).to_string().yellow());
        if result.is_err() {
            eprintln!("Error: {}", result.as_ref().err().unwrap().bold().red());
        } else {
            println!("Result: {}", result.as_ref().unwrap().green());
        }
    }
}

fn problem1(day: u8) -> Result<String, String> {
    match day {
        d if d <= 0 => return Err("Invalid day, please use a value between 01 and 24".to_string()),
        d if d > 24 => return Err("Invalid day, please use a value between 01 and 24".to_string()),
        _ => {
            if !day_exists(day) {
                return Err("Day has no input yet".to_string());
            }
            days::delegate_problem1(day)
        }
    }
}

fn problem2(day: u8) -> Result<String, String> {
    match day {
        d if d <= 0 => return Err("Invalid day, please use a value between 01 and 24".to_string()),
        d if d > 24 => return Err("Invalid day, please use a value between 01 and 24".to_string()),
        _ => {
            if !day_exists(day) {
                return Err("Day has no input yet".to_string());
            }
            days::delegate_problem2(day)
        }
    }
}

fn day_exists(day: u8) -> bool {
    let path = format!("input/{:02}.txt", day);
    std::path::Path::new(&path).exists()
}
