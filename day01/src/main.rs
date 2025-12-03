use std::io::BufRead;
use std::{fs::*, io::BufReader};

fn main() {
    const INPUT: &str = "./resources/day01/input.txt";
    let iter = BufReader::new(File::open(INPUT).unwrap())
        .lines()
        .map(|r| r.unwrap());
    // println!("stage1 {}", stage1(iter));
    println!("stage2 {}", stage2(iter));
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

fn stage2<T>(it: T) -> u32
where
    T: Iterator<Item = String>,
{
    let mut result = 0u32;
    let mut state = 50i32;
    for line in it {
        let line = line.trim();
        let (dir, rest) = line.split_at(1);
        let rest = rest.parse::<u32>().unwrap();

        let quot = rest.div_euclid(100);
        let rem = rest.rem_euclid(100);
        result += quot;

        match dir {
            "L" => {
                // 0 <= rem < 100
                if state > 0 && state - (rem as i32) <= 0 {
                    result += 1
                }
                state -= rem as i32;
            }
            "R" => {
                // 0 <= state < 100
                state += rem as i32;
                if state >= 100 {
                    result += 1
                }
            }
            _ => panic!("Invalid start of line"),
        };

        state = state.rem_euclid(100);
    }

    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let input = "R150";
        let iter = input
            .lines()
            .map(|s| s.into())
            .collect::<Vec<String>>()
            .into_iter();
        use super::*;

        assert_eq!(2, stage2(iter));
    }

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
        let iter = input
            .lines()
            .map(|s| s.into())
            .collect::<Vec<String>>()
            .into_iter();
        use super::*;
        assert_eq!(3, stage1(iter.clone()));

        assert_eq!(6, stage2(iter));
    }
}
