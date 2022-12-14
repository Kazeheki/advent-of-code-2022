use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn problem1(buffer: BufReader<File>) -> Result<String, String> {
    // started with tree and failed..
    // this solution is influenced by ThePrimeagen noting in his stream solving AoC 07, that the input is alread DFS.
    // I feel so stupid right now...

    const MAX_SIZE: u32 = 100_000;
    const CD_COMAND: &str = "$ cd ";

    // (dir-name, dir-size) dir-name only for debugging
    let mut stack: Vec<(String, u32)> = vec![("/".to_string(), 0)];
    let mut sum: u32 = 0;

    for line_result in buffer.lines() {
        let mut line = line_result.unwrap();

        // "cd /" only happens at beginning in example and my input.
        if line == "$ cd /" || line == "$ ls" {
            continue;
        }

        if line.starts_with(CD_COMAND) {
            let dir = line.split_off(CD_COMAND.len());

            if dir == ".." {
                // pop off and add to parent.
                let (_, size) = stack.pop().unwrap();

                if size <= MAX_SIZE {
                    // fill the sum of all dirs under limit.
                    sum += size;
                }
                stack.last_mut().unwrap().1 += size;
            } else {
                // add new dir to stack.
                stack.push((dir, 0));
            }
        }

        let (size, _) = line.split_once(" ").unwrap();

        if let Ok(size) = size.parse::<u32>() {
            stack.last_mut().unwrap().1 += size;
        }
        // otherwise it would be information about dir.
    }

    Ok(sum.to_string())
}

pub fn problem2(buffer: BufReader<File>) -> Result<String, String> {
    const SYSTEM_SIZE: u32 = 70_000_000;
    const UPDATA_SIZE: u32 = 30_000_000;
    const CD_COMAND: &str = "$ cd ";

    // (dir-name, dir-size) dir-name only for debugging
    let mut stack: Vec<(String, u32)> = vec![("/".to_string(), 0)];
    let mut permanent: Vec<(String, u32)> = vec![];

    for line_result in buffer.lines() {
        let mut line = line_result.unwrap();

        // "cd /" only happens at beginning in example and my input.
        if line == "$ cd /" || line == "$ ls" {
            continue;
        }

        if line.starts_with(CD_COMAND) {
            let dir = line.split_off(CD_COMAND.len());

            if dir == ".." {
                // pop off and add to parent.
                let (name, size) = stack.pop().unwrap();

                stack.last_mut().unwrap().1 += size;

                permanent.push((name, size));
            } else {
                // add new dir to stack.
                stack.push((dir, 0));
            }
        }

        let (size, _) = line.split_once(" ").unwrap();

        if let Ok(size) = size.parse::<u32>() {
            stack.last_mut().unwrap().1 += size;
        }
        // otherwise it would be information about dir.
    }

    while stack.len() > 1 {
        // pop off and add to parent.
        let (name, size) = stack.pop().unwrap();

        stack.last_mut().unwrap().1 += size;

        permanent.push((name, size));
    }

    let used = stack.iter().map(|(_, s)| s).sum::<u32>();
    let unused = SYSTEM_SIZE - used;
    let still_needed = UPDATA_SIZE - unused;

    permanent.sort_by(|(_, s1), (_, s2)| s1.cmp(s2));

    let smaller = permanent
        .iter()
        .filter(|(_, s)| s >= &still_needed)
        .next()
        .unwrap()
        .1;

    Ok(smaller.to_string())
}

#[cfg(test)]
mod tests {
    use crate::days::read_file_to_buffer;

    const PATH: &str = "src/days/test-input/day07-example.txt";

    #[test]
    fn day07_part1() {
        let expected: Option<&str> = Some("95437");
        let buffer = read_file_to_buffer(PATH.to_string());
        let result = super::problem1(buffer);
        assert!(
            result.is_ok(),
            "Problem returned error: {}",
            result.unwrap_err()
        );
        assert_eq!(result.unwrap(), expected.unwrap());
    }

    #[test]
    fn day07_part2() {
        let expected: Option<&str> = Some("24933642");
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
