use std::{
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader},
};

// checkout fasterthanli.me
// my solution is heavily influenced by their solution.
// I learned quite some stuff and used their logic.
// (https://fasterthanli.me/series/advent-of-code-2022/part-8)

pub fn problem1(buffer: BufReader<File>) -> Result<String, String> {
    let forest = parse_forest(buffer).unwrap();

    // learned: You can easily iterate over two ranges.
    let all_coords = (0..forest.height).into_iter().flat_map(|y| {
        (0..forest.width)
            .into_iter()
            .map(move |x| Coord::from((x, y)))
    });

    let directions: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    let num_visible = all_coords
        .filter(|coord| {
            let tree_height = forest.tree(coord).unwrap();
            directions.iter().any(|(dx, dy)| {
                // learned: map_while is awesome!
                // Mapping until there is an error, then stop without an error
                // and return the mapped stuff.
                let mut in_line = (1..).into_iter().map_while(|i| {
                    let to_check = Coord {
                        x: (coord.x as isize + (dx * i)) as usize,
                        y: (coord.y as isize + (dy * i)) as usize,
                    };
                    forest.tree(&to_check)
                });
                in_line.all(|t| t < tree_height)
            })
        })
        .count();

    Ok(num_visible.to_string())
}

pub fn problem2(_buffer: BufReader<File>) -> Result<String, String> {
    Err("Not implemented".to_string())
}

fn parse_forest(buffer: BufReader<File>) -> Option<Forest> {
    let mut trees: Vec<usize> = vec![];

    let mut i: usize = 0;
    let mut width: usize = 0;
    for line_result in buffer.lines() {
        let line = line_result.unwrap();
        let chars: Vec<usize> = line
            .chars()
            .map(|c: char| c.to_digit(10).unwrap() as usize)
            .collect();

        if width == 0 {
            width = chars.len();
        }

        trees.extend(chars);

        i = i + 1;
    }

    Some(Forest::new(trees, width))
}

struct Coord {
    x: usize,
    y: usize,
}
impl Debug for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "({}|{})", self.x, self.y)
    }
}
impl From<(usize, usize)> for Coord {
    fn from(tuple: (usize, usize)) -> Self {
        Coord {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

#[derive(Debug)]
struct Forest {
    width: usize,
    height: usize,
    trees: Vec<usize>,
}
impl Forest {
    fn new(trees: Vec<usize>, width: usize) -> Self {
        Forest {
            width,
            height: trees.len() / width,
            trees,
        }
    }

    fn inside_forest(&self, coord: &Coord) -> bool {
        coord.x < self.width && coord.y < self.height
    }

    fn tree(&self, coord: &Coord) -> Option<&usize> {
        if !self.inside_forest(coord) {
            return None;
        }
        return Some(&self.trees[coord.y * self.width + coord.x]);
    }
}

#[cfg(test)]
mod tests {
    use crate::days::read_file_to_buffer;

    const PATH: &str = "src/days/test-input/day08-example.txt";

    #[test]
    fn day08_part1() {
        let expected: Option<&str> = Some("21");
        let buffer = read_file_to_buffer(PATH.to_string());
        let result = super::problem1(buffer);
        assert!(
            result.is_ok(),
            "Problem returned error: {}",
            result.unwrap_err()
        );
        assert_eq!(result.unwrap(), expected.unwrap());
    }

    #[allow(dead_code)]
    fn day08_part2() {
        let expected: Option<&str> = None;
        let buffer = read_file_to_buffer(PATH.to_string());
        let result = super::problem2(buffer);
        assert!(
            result.is_ok(),
            "Problem returned error: {}",
            result.unwrap_err()
        );
        assert_eq!(result.unwrap(), expected.unwrap());
    }
}
