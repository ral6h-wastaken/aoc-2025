use std::{
    cmp::{max, min},
    ops::RangeInclusive,
    str::FromStr,
};

const COMMA: char = ',';

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Point(i32, i32);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Rectangle {
    base: RangeInclusive<i32>,
    height: RangeInclusive<i32>,
}

impl Rectangle {
    pub fn area(&self) -> usize {
        self.base.clone().count() * self.height.clone().count()
    }

    pub fn inner(&self) -> Option<Self> {
        let result = Self {
            base: *self.base.start() + 1..=*self.base.end() - 1,
            height: *self.height.start() + 1..=*self.height.end() - 1,
        };

        if result.base.is_empty() || result.height.is_empty() {
            None
        } else {
            Some(result)
        }
    }
}

pub trait Intersects {
    fn intersects(&self, other: &Self) -> bool;
}

impl Intersects for RangeInclusive<i32> {
    fn intersects(&self, other: &Self) -> bool {
        max(self.start(), other.start()) <= min(self.end(), other.end())
    }
}

impl Intersects for Rectangle {
    fn intersects(&self, other: &Self) -> bool {
        self.base.intersects(&other.base) && self.height.intersects(&other.height)
    }
}

impl From<(&Point, &Point)> for Rectangle {
    fn from((p1, p2): (&Point, &Point)) -> Self {
        Self {
            base: min(p1.0, p2.0)..=max(p1.0, p2.0),
            height: min(p1.1, p2.1)..=max(p1.1, p2.1),
        }
    }
}

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
