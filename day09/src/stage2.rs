//solution is entirely based on this one by Todd Ginsberg: https://todd.ginsberg.com/post/advent-of-code/2025/day9/
use std::{cmp, str::FromStr};

use crate::common::{Intersects, Point, Rectangle};

pub fn solution<T>(lines: T) -> u64
where
    T: Iterator<Item = String>,
{
    let mut points: Vec<Point> = lines.map(|l| Point::from_str(&l).unwrap()).collect();
    points.push(points[0].clone());

    let edges: Vec<Rectangle> = points
        .windows(2)
        .map(|points| Rectangle::from((&points[0], &points[1])))
        .collect();

    println!("Got edges: {:?}", edges);

    let mut rectangles: Vec<Rectangle> = Vec::new();

    for (i, p) in points.iter().enumerate() {
        for (j, q) in points.iter().enumerate() {
            if j > i {
                rectangles.push(Rectangle::from((p, q)));
            }
        }
    }

    println!("Got rectangles: {:?}", rectangles);

    let areas: Vec<usize> = rectangles
        .iter()
        .filter(|r| {
            !edges.iter().any(|edge| match r.inner() {
                Some(inn) => edge.intersects(&inn),
                None => false,
            })
        })
        .map(|r| (&r).area())
        .collect();
    println!("Computed areas: {:?}", areas);

    *areas.iter().max().expect("could not determine max area") as u64
}
