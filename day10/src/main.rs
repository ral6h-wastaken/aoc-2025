use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let lines = BufReader::new(File::open("./resources/day10/input.txt").unwrap())
        .lines()
        .map(|l| l.unwrap());

    println!("stage1 -> {}", stage1::solution(lines));
    // println!("stage2 -> {}", stage2::solution(lines));
}

mod stage1;
mod stage2;
mod common;

#[cfg(test)]
mod tests {
    #[test]
    fn test_stg_1() {
        use super::stage1::*;

        let input = "
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";
        let ans = 7_u64;

        assert_eq!(solution(input.lines().filter(|l| !l.trim().is_empty()).map(|l| l.into())), ans);
    }

    #[test]
    fn test_stg_2() {
        use super::stage2::*;

        let input = "
";
        let ans = 0_u64;

        assert_eq!(solution(input.lines().filter(|l| !l.trim().is_empty()).map(|l| l.into())), ans);
    }
}
