use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const DASH: char = '-';

fn main() {
    let lines = BufReader::new(File::open("./resources/day05/input.txt").unwrap())
        .lines()
        .map(|l| l.unwrap());

    // println!("stage1 -> {}", solution(lines));
    println!("stage2 -> {}", stage2::solution(lines));
}

mod stage1;
mod stage2;
mod common;

#[cfg(test)]
mod tests {
    #[test]
    fn test_stg_1() {
        use super::stage1::*;

        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

        assert_eq!(solution(input.lines().map(|l| l.into())), 3);
    }

    #[test]
    fn test_stg_2() {
        use super::stage2::*;

        let input = "3-5
10-14
16-20
12-18
11-13
21-23
19-22


1
5
8
11
17
32";

        assert_eq!(solution(input.lines().map(|l| l.into())), 17);
    }
}
