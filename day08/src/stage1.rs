use std::str::FromStr;

use crate::common::Point;

pub fn solution<T>(mut lines: T) -> u64
where
    T: Iterator<Item = String>,
{
    let points: Vec<Point> = lines.map(|l| Point::from_str(&l).expect("input promised valid lines :(")).collect();
    println!("parsed points {points:?}");
    0
}
