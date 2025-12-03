use std::io::BufRead;
use std::{fs::*, io::BufReader};

fn main() {
    const INPUT: &str = "./resources/input.txt";
    let iter = BufReader::new(File::open(INPUT).unwrap())
        .lines()
        .map(|r| r.unwrap());
    println!("{}", stage1(iter))
}

fn stage1<T>(it: T) -> u16
where
    T: Iterator<Item = String>,
{
    let mut result = 0;
    let mut state = 50i32;
    for line in it {
        let line = line.trim();
        match line.split_at(1) {
            ("L", rest) => state -= rest.parse::<i32>().unwrap(),
            ("R", rest) => state += rest.parse::<i32>().unwrap(),
            _ => panic!(),
        }
        if state.rem_euclid(100) == 0 {
            result += 1
        }
    }
    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";
        let iter = input.lines().map(|s| s.into()).collect::<Vec<String>>().into_iter();
        use super::*;
        assert_eq!(3, stage1(iter))
    }
}
