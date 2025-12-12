use std::{cmp::{max, min}, str::FromStr};

const COMMA: char = ',';
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Point(u64, u64, u64);

impl TryFrom<&str> for Point {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Point::from_str(value)
    }
}

impl Point {
    pub fn raw_coordinates(&self) -> (u64, u64, u64) {
        (self.0, self.1, self.2)
    }

    pub fn distance_squared(&self, other: &Point) -> u64 {
        let (x_1, y_1, z_1) = (self.0, self.1, self.2);
        let (x_2, y_2, z_2) = (other.0, other.1, other.2);

        return (max(x_1, x_2) - min(x_1, x_2)).pow(2) + (max(y_1, y_2) - min(y_1, y_2)).pow(2) + (max(z_1, z_2) - min(z_1, z_2)).pow(2)
    }
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s
            .trim()
            .splitn(3, COMMA)
            .map(|s| {
                s.parse::<u64>()
                    .map_err(|err| format!("Invalid coordinate {}: err: {}", s, err.to_string()))
            })
            .collect::<Result<Vec<u64>, String>>()
        {
            Ok(v) => {
                if let Some(x) = v.get(0)
                    && let Some(y) = v.get(1)
                    && let Some(z) = v.get(2)
                {
                    Ok(Point(*x, *y, *z))
                } else {
                    Err(format!("Invalid point {}", s.to_string()))
                }
            }
            Err(s) => Err(s),
        }
    }
}
