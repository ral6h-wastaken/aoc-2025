use std::{
    fs::File,
    io::{BufRead, BufReader},
};

mod stage1;
mod stage2;

fn main() {
    let lines: Vec<String> = BufReader::new(File::open("./resources/day03/input.txt").unwrap())
        .lines()
        .map(|l| l.unwrap())
        .collect();

    println!("stage 1 -> {}", stage1::solution(lines.clone()));
    println!("stage 2 -> {}", stage1::solution(lines));
}


#[cfg(test)]
mod test {
    #[test]
    fn test_stage_one() {
        use super::stage1::*;

        let input = vec![
            "987654321111111".into(),
            "811111111111119".into(),
            "234234234234278".into(),
            "818181911112111".into(),
        ];

        assert_eq!(solution(input), 357);
    }

    #[test]
    fn test_stage_two() {
        use super::stage2::*;

        let input = vec![
            "987654321111111".into(),
            "811111111111119".into(),
            "234234234234278".into(),
            "818181911112111".into(),
        ];

        assert_eq!(solution(input), 3121910778619);
    }
}
