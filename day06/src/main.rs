use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let lines = BufReader::new(File::open("./resources/day06/input.txt").unwrap())
        .lines()
        .map(|l| l.unwrap());

    println!("stage1 -> {}", stage1::solution(lines));
    // println!("stage2 -> {}", stage2::solution(lines));
}

mod stage1;

mod stage2 {
    pub fn solution<T>(lines: T) -> u64
    where
        T: Iterator<Item = String>,
    {
        todo!()
    }
}

mod common;

#[cfg(test)]
mod tests {
    #[test]
    fn test_stg_1() {
        use super::stage1::*;

        let input = "123 328  51 64 
                     45 64  387 23 
                      6 98  215 314
                      *   +   *   +";
        let ans = 4277556_u64;

        assert_eq!(solution(input.lines().map(|l| l.into())), ans);
    }

    #[test]
    fn test_stg_2() {
        use super::stage2::*;

        let input = "";
        let ans = 0_u64;

        assert_eq!(solution(input.lines().map(|l| l.into())), ans);
    }
}
