use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let lines = BufReader::new(File::open("./resources/day08/input.txt").unwrap())
        .lines()
        .map(|l| l.unwrap());

    println!("stage1 -> {}", stage1::solution(lines, 1000));
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
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";
        let ans = 40;

        assert_eq!(solution(input.lines().filter(|l| !l.trim().is_empty()).map(|l| l.into()), 10), ans);
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
