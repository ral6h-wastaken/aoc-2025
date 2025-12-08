use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::RangeInclusive,
    vec,
};

const DASH: char = '-';

fn main() {
    let lines = BufReader::new(File::open("./resources/day05/input.txt").unwrap())
        .lines()
        .map(|l| l.unwrap());

    println!("stage1 -> {}", solution(lines));
}

fn solution<T>(lines: T) -> u64
where
    T: Iterator<Item = String>,
{
    let mut fresh_ranges: Vec<RangeInclusive<u64>> = vec![];
    let mut count = 0_u64;

    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        } //separator line

        match line.split_once(DASH) {
            Some((min, max)) => {
                //TODO: dummy implementation
                let min = min.parse().expect("input promised :(");
                let max = max.parse().expect("input promised :(");

                fresh_ranges.push(min..=max);
            }
            None => {
                let candidate = line.parse().expect(format!("input promised :( line: {}", line).as_str());

                for ran in &fresh_ranges {
                    if ran.contains(&candidate) {
                        count += 1;
                        break;
                    }
                }
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_stg_1() {
        use super::*;

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

    //     #[test]
    //     fn test_stg_2() {
    //         use super::stage2::*;
    //
    //         let input = "..@@.@@@@.
    // @@@.@.@.@@
    // @@@@@.@.@@
    // @.@@@@..@.
    // @@.@@@@.@@
    // .@@@@@@@.@
    // .@.@.@.@@@
    // @.@@@.@@@@
    // .@@@@@@@@.
    // @.@.@@@.@.";
    //
    //         assert_eq!(solution(input.lines().map(|l| l.into())), 43);
    //     }
}
