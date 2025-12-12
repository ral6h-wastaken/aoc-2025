use std::{cmp, str::FromStr};

use crate::common::Point;

pub fn solution<T>(lines: T) -> u64
where
    T: Iterator<Item = String>,
{
    let points: Vec<Point> = lines.map(|l| Point::from_str(&l).unwrap()).collect();
    let mut max_area = 0;

    for (i, p) in points.iter().enumerate() {
        for (j, q) in points.iter().enumerate() {
            if j > i {
                max_area = cmp::max(max_area, p.rectangle_area(&q))
            }
        }
    }

    max_area
}
