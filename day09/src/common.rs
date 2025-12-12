use std::str::FromStr;

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
