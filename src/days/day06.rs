use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn problem1(buffer: BufReader<File>) -> Result<String, String> {
    let file = buffer
        .lines()
        .into_iter()
        .map(|x| x.unwrap().to_string() + "\n")
        .collect::<String>();

    let result = index_first_4_unique(&file).expect("No index found") + 4;

    Ok(result.to_string())
}

pub fn problem2(_buffer: BufReader<File>) -> Result<String, String> {
    Err("Not implemented".to_string())
}

fn index_first_4_unique(input: &String) -> Option<usize> {
    let chars: Vec<char> = input.chars().collect();

    let mut index_of_first_4_unique: Option<usize> = None;
    for (pos, _) in chars.iter().enumerate() {
        let (_, interest) = chars.split_at(pos);
        let (head, _) = interest.split_at(4);
        let unique: HashSet<char> = HashSet::from_iter(head.iter().cloned());
        if unique.len() == 4 {
            index_of_first_4_unique = Some(pos);
            break;
        }
    }

    index_of_first_4_unique
}

#[cfg(test)]
mod tests {
    extern crate test_case;
    use test_case::test_case;

    use crate::days::read_file_to_buffer;

    #[test_case("day06-example-1.txt", "7"; "First example")]
    #[test_case("day06-example-2.txt", "5"; "Second example")]
    #[test_case("day06-example-3.txt", "6"; "Third example")]
    #[test_case("day06-example-4.txt", "10"; "Fourth example")]
    #[test_case("day06-example-5.txt", "11"; "Fifth example")]
    fn day06_part1(file: &str, expected: &str) {
        let path = format!("src/days/test-input/{}", file);
        let buffer = read_file_to_buffer(path);
        let result = super::problem1(buffer);
        assert!(
            result.is_ok(),
            "Problem returned error: {}",
            result.unwrap_err()
        );
        assert_eq!(result.unwrap(), expected);
    }
}
