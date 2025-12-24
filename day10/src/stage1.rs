use std::{fmt::format, str::FromStr};

use crate::common::{self, Machine};

pub fn solution<T>(mut lines: T) -> u64
where
    T: Iterator<Item = String>,
{
    let machines = lines
        .map(|l| Machine::from_str(&l).expect(&format!("Invalid line {l}")))
        .collect::<Vec<Machine>>();

    println!("Parsed machines {:?}", machines);
    0
}
