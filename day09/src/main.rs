use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let lines = BufReader::new(File::open("./resources/day09/input.txt").unwrap())
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
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";
        let ans = 50_u64;

        assert_eq!(solution(input.lines().filter(|l| !l.trim().is_empty()).map(|l| l.into())), ans);
    }

    #[test]
    fn test_stg_2() {
        use super::stage2::*;

        let input = "
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";
        let ans = 0_u64;

        assert_eq!(solution(input.lines().filter(|l| !l.trim().is_empty()).map(|l| l.into())), ans);
    }
}
