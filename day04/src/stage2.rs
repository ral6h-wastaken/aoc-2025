use crate::common::{self, Tile};

pub fn solution<T>(lines: T) -> u64
where
    T: Iterator<Item = String>,
{
    let mut grid = common::get_grid(lines);
    let mut total = 0;

    //we repeatedly iterate on the grid removing the logs that can be removed (and keeping count of
    //them) until we can remove no more

    loop {
        let to_remove: Vec<(usize, usize)> = grid
            .iter()
            .filter(|(_, (tile, count))| *count < 4 && tile.as_ref().is_some_and(|t| t.is_roll()))
            .map(|(key, _)| key)
            .copied()
            .collect();

        let accessible = to_remove.len();

        if accessible == 0 {
            break;
        } else {
            total += to_remove.len();
            for (i, j) in to_remove {
                grid.entry((i, j)).and_modify(|(tile, _)| {
                    tile.replace(Tile::new(common::TileKind::DOT));
                });

                for (n, m) in common::calculate_neighbours((i as isize, j as isize))
                    .iter()
                    .filter(|(h, k)| *h >= 0 && *k >= 0)
                {
                    grid.entry((*n as usize, *m as usize)).and_modify(|(_, count)| *count -= 1);
                }
            }
        }
    }

    total.try_into().unwrap()
}
