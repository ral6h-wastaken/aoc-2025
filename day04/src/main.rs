use std::{
    fs::File,
    io::{BufRead, BufReader},
};

mod common;
mod stage1;
mod stage2;

fn main() {
    let lines = BufReader::new(File::open("./resources/day04/input.txt").unwrap())
        .lines()
        .map(|l| l.unwrap());

    //println!("stage1 -> {}", stage1::solution(lines));
    println!("stage2 -> {}", stage2::solution(lines));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_stg_1() {
        use super::stage1::*;

        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

        assert_eq!(solution(input.lines().map(|l| l.into())), 13);
    }

    #[test]
    fn test_stg_2() {
        use super::stage2::*;

        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

        assert_eq!(solution(input.lines().map(|l| l.into())), 43);
    }
}
