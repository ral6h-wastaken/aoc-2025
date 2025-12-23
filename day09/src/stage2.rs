use std::{cmp, str::FromStr};

use crate::common::{Intersects, Point, Polygon};

pub fn solution<T>(lines: T) -> u64
where
    T: Iterator<Item = String>,
{
    let points: Vec<Point> = lines.map(|l| Point::from_str(&l).unwrap()).collect();
    let polygon = Polygon::try_from(&points).expect("Invalid set of points: ");

    // println!("Computed polygon {polygon:?}");

    let mut max_area = 0;

    for (i, p) in points.iter().enumerate() {
        for (j, q) in points.iter().enumerate() {
            if j > i && !p.rectangle(&q).intersects(&polygon) {
                max_area = cmp::max(max_area, p.rectangle_area(&q))
            }
        }
    }

    max_area
}
