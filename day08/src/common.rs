use std::str::FromStr;

const COMMA: char = ',';
#[derive(Debug)]
pub struct Point(u64, u64, u64);

impl TryFrom<&str> for Point {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Point::from_str(value)
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
