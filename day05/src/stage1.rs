use std::{ops::RangeInclusive, vec};

use crate::common;

pub fn solution<T>(lines: T) -> u64
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

        match line.split_once(crate::DASH) {
            Some((min, max)) => {
                let min = min.parse().expect("input promised :(");
                let max = max.parse().expect("input promised :(");

                common::update_ranges(&mut fresh_ranges, (min, max));
            }
            None => {
                let candidate = line
                    .parse()
                    .expect(format!("input promised :( line: {}", line).as_str());

                for ran in &fresh_ranges {
                    if ran.contains(&candidate) {
                        count += 1;
                        break;
                    }
                }
            }
        }
    }

    println!("ranges: {:?}", fresh_ranges);

    count
}

