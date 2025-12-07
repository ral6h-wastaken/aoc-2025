use crate::common::{self};

pub fn solution<T>(lines: T) -> u64
where
    T: Iterator<Item = String>,
{
    let grid = common::get_grid(lines);

    grid.iter()
        .filter(|(_, (tile, count))| *count < 4 && tile.as_ref().is_some_and(|t| t.is_roll()))
        .count()
        .try_into()
        .unwrap()
}
