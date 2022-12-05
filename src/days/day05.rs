use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
    string::ParseError,
};

pub fn problem1(buffer: BufReader<File>) -> Result<String, String> {
    let file = buffer
        .lines()
        .into_iter()
        .map(|s| s.unwrap().to_string() + "\n")
        .collect::<String>();

    let (crane, actions) = file.split_once("\n\n").unwrap();

    let mut crane: Crane = crane.parse::<Crane>().unwrap();
    let moves: Vec<Move> = actions
        .lines()
        .map(|x| x.parse::<Move>().unwrap())
        .collect::<Vec<_>>();
    crane.add_moves(moves);
    crane.execute_moves();
    let top_boxes: Vec<char> = crane.top_boxes();

    Ok(top_boxes.iter().collect::<String>())
}

pub fn problem2(_buffer: BufReader<File>) -> Result<String, String> {
    Err("Not implemented".to_string())
}

struct Crane {
    stacks: Vec<Vec<Option<char>>>,
    moves: Vec<Move>,
}

struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

impl FromStr for Crane {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // s is like:
        // [A]     [B]
        // [C] [D] [E] [F]
        //  1   2   3   4

        let s = s
            .lines()
            .rev()
            .map(|x| x.to_owned() + "\n")
            .collect::<String>();

        let (stack_names, contents) = s.split_once("\n").unwrap();

        let num_of_stacks = stack_names.split_whitespace().count();

        // contents now
        // [C] [D] [E] [F]
        // [A]     [B]

        let rows = contents
            .lines()
            .map(|line| {
                line.chars()
                    .collect::<Vec<char>>()
                    .chunks(4)
                    .map(|c| c.iter().collect::<String>())
                    .collect::<Vec<String>>()
                    .iter()
                    .map(|s| {
                        if s.starts_with("[") {
                            let name = s.trim().replace("[", "").replace("]", "");
                            return name.chars().next();
                        }
                        None
                    })
                    .collect::<Vec<Option<char>>>()
            })
            .collect::<Vec<Vec<Option<char>>>>();

        // [C D E F]
        // [A . B .]

        let mut stacks = vec![vec![]; num_of_stacks];
        for row in rows {
            let iter = row.iter();
            for (pos, el) in iter.enumerate() {
                stacks[pos].push(el.clone());
            }
        }
        let stacks = stacks
            .iter()
            .map(|stack| {
                stack
                    .iter()
                    .filter(|c| c.is_some())
                    .map(|c| c.to_owned())
                    .collect::<Vec<Option<char>>>()
            })
            .collect::<Vec<Vec<Option<char>>>>();

        // stack 1 = [C A]
        // stack 2 = [D .]
        // stack 3 = [E B]
        // stack 4 = [F .]

        Ok(Crane {
            stacks,
            moves: vec![],
        })
    }
}

impl FromStr for Move {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let move_values = s
            .split_whitespace()
            .enumerate()
            .filter(|&(idx, _)| idx % 2 != 0)
            .map(|(_, val)| val)
            .map(|val| val.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        Ok(Move {
            amount: move_values[0],
            from: move_values[1],
            to: move_values[2],
        })
    }
}

impl Crane {
    fn add_moves(&mut self, moves: Vec<Move>) {
        self.moves = moves;
    }

    fn execute_moves(&mut self) {
        self.moves.iter().for_each(|m| {
            let (amount, from_idx, to_idx) = (m.amount, m.from, m.to);
            for _ in 0..amount {
                let mut from = self.stacks[from_idx - 1].to_owned();
                let mut to = self.stacks[to_idx - 1].to_owned();
                to.push(from.pop().unwrap());
                self.stacks[from_idx - 1] = from;
                self.stacks[to_idx - 1] = to;
            }
        });
    }

    fn top_boxes(&mut self) -> Vec<char> {
        self.stacks
            .iter()
            .map(|stack| stack.last().unwrap())
            .map(|optional| optional.unwrap())
            .collect::<Vec<char>>()
    }
}

#[cfg(test)]
mod tests {
    use crate::days::read_file_to_buffer;

    #[test]
    fn day05_part1() {
        let path = "src/days/test-input/day05-example.txt".to_string();
        let buffer = read_file_to_buffer(path);
        let result = super::problem1(buffer);
        assert!(
            result.is_ok(),
            "Problem returned error: {}",
            result.unwrap_err()
        );
        assert_eq!(result.unwrap(), "CMZ");
    }

    #[allow(dead_code)]
    fn day05_part2() {
        let path = "src/days/test-input/day05-example.txt".to_string();
        let buffer = read_file_to_buffer(path);
        let result = super::problem2(buffer);
        assert!(
            result.is_ok(),
            "Problem returned error: {}",
            result.unwrap_err()
        );
        assert_eq!(result.unwrap(), "");
    }
}
