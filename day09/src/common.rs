use std::{
    cmp::{self, max, min},
    ops::{Range, RangeInclusive},
    str::FromStr,
};

const COMMA: char = ',';

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Point(i32, i32);

impl TryFrom<&str> for Point {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Point::from_str(value)
    }
}

impl Point {
    pub fn raw_coordinates(&self) -> (i32, i32) {
        (self.0, self.1)
    }

    pub fn rectangle_area(&self, other: &Point) -> u64 {
        let (x_1, y_1) = (self.0, self.1);
        let (x_2, y_2) = (other.0, other.1);

        let base = ((x_1 - x_2).abs() + 1) as u64;
        let heigth = ((y_1 - y_2).abs() + 1) as u64;

        base * heigth
    }

    pub fn rectangle(&self, other: &Point) -> Polygon {
        //println!("getting rectange for points {self:?} and {other:?}");

        let (x_1, y_1) = (self.0, self.1);
        let (x_2, y_2) = (other.0, other.1);

        let (x_min, x_max) = (cmp::min(x_1, x_2), cmp::max(x_1, x_2));
        let (y_min, y_max) = (cmp::min(y_1, y_2), cmp::max(y_1, y_2));

        //println!("x_min {x_min} x_max {x_max} y_min {y_min} y_max {y_max}");

        let edges = vec![
            Edge::Vertical { x: x_min, heigth: y_min..=y_max },
            Edge::Vertical { x: x_max, heigth: y_min..=y_max },
            Edge::Horizontal { y: y_min, width: x_min..=x_max },
            Edge::Horizontal { y: y_max, width: x_min..=x_max },
        ];

        let rectangle = Polygon { edges };
        //println!("{:?}", rectangle);

        rectangle
    }
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s
            .trim()
            .splitn(2, COMMA)
            .map(|s| {
                s.parse::<i32>()
                    .map_err(|err| format!("Invalid coordinate {}: err: {}", s, err.to_string()))
            })
            .collect::<Result<Vec<i32>, String>>()
        {
            Ok(v) => {
                if let Some(x) = v.get(0)
                    && let Some(y) = v.get(1)
                {
                    Ok(Point(*x, *y))
                } else {
                    Err(format!("Invalid point {}", s.to_string()))
                }
            }
            Err(s) => Err(s),
        }
    }
}

#[derive(Debug)]
pub enum Edge {
    Vertical { x: i32, heigth: RangeInclusive<i32> },
    Horizontal { y: i32, width: RangeInclusive<i32> },
}

#[derive(Debug)]
pub struct Polygon {
    edges: Vec<Edge>,
}

impl TryFrom<&Vec<Point>> for Polygon {
    type Error = String;

    fn try_from(points: &Vec<Point>) -> Result<Self, Self::Error> {
        let mut edges = Vec::<Edge>::new();
        let len = points.len();

        for i in 0..len {
            edges.push(Edge::try_from((&points[i.rem_euclid(len)], &points[(i+1).rem_euclid(len)]))?); //wraps around
        }

        Ok(Self{ edges })
    }
}

impl Intersects<Polygon> for Polygon {
    fn intersects(&self, other: &Polygon) -> bool {
        // //println!(">>>>>> checking intersection between Polygons {self:?} and {other:?}");

        for s in self.edges.iter() {
            for o in other.edges.iter() {
                if s.intersects(&o) {
                    // //println!("found intersection");
                    return true;
                }
            }
        }

        // //println!("intersection not found");
        false
    }
}

impl TryFrom<(&Point, &Point)> for Edge {
    type Error = String;

    fn try_from(value: (&Point, &Point)) -> Result<Self, Self::Error> {
        let (x_1, y_1) = value.0.raw_coordinates();
        let (x_2, y_2) = value.1.raw_coordinates();

        if x_1 == x_2 {
            Ok(Self::Vertical {
                x: x_1,
                heigth: min(y_1, y_2)..=max(y_1, y_2),
            })
        } else if y_1 == y_2 {
            Ok(Self::Horizontal {
                y: y_1,
                width: min(x_1, x_2)..=max(x_1, x_2),
            })
        } else {
            Err(format!("Invalid pair of coordinates {:?}", value))
        }
    }
}

pub trait Intersects<T> {
    fn intersects(&self, other: &T) -> bool;
}

impl Intersects<Edge> for Edge {
    fn intersects(&self, other: &Edge) -> bool {
        //print!("Checking intersection between edges {self:?} and {other:?} -> ");

        let result = match (self, other) {
            (
                Edge::Vertical {
                    x: x_1,
                    heigth: h_1,
                },
                Edge::Vertical {
                    x: x_2,
                    heigth: h_2,
                },
            ) => {
                if x_1 != x_2 {
                    false
                } else {
                    h_1.intersects(h_2)
                }
            }
            (Edge::Horizontal { y: y_1, width: w_1 }, Edge::Horizontal { y: y_2, width: w_2 }) => {
                if y_1 != y_2 {
                    false
                } else {
                    w_1.intersects(w_2)
                }
            }
            (Edge::Vertical { x, heigth }, Edge::Horizontal { y, width }) => {
                heigth.start() < y && y < heigth.end()
                &&
                width.start() < x && x < width.end()
            }
            (Edge::Horizontal { y, width }, Edge::Vertical { x, heigth }) => {
                heigth.start() < y && y < heigth.end()
                &&
                width.start() < x && x < width.end()
            }
        };

        // //println!("{result}");
        result
    }
}

impl Intersects<RangeInclusive<i32>> for RangeInclusive<i32> {
    fn intersects(&self, other: &RangeInclusive<i32>) -> bool {
        /*
        * ______________________________
        *   ____________________________
        */
        (other.start() <= self.start() && self.start() <= other.end())
        ^
        (other.start() <= self.end() && self.end() <= other.end())
        ^
        (self.start() <= other.start() && other.start() <= self.end())
        ^
        (self.start() <= other.end() && other.end() <= self.end())
    }
}
