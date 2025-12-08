use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Tile {
    kind: TileKind,
}

impl Tile {
    pub fn new(kind: TileKind) -> Self {
        Self { kind }
    }

    pub fn is_roll(&self) -> bool {
        self.kind == TileKind::ROLL
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum TileKind {
    ROLL,
    DOT,
}

impl TileKind {
    pub fn from(value: char) -> Self {
        match value {
            '@' => Self::ROLL,
            '.' => Self::DOT,
            _ => panic!("input promised just dots and rolls :("),
        }
    }
}

pub fn get_grid<T>(lines: T) -> HashMap<(usize, usize), (Option<Tile>, u8)>
where
    T: Iterator<Item = String>,
{
    let mut grid: HashMap<(usize, usize), (Option<Tile>, u8)> = HashMap::new();

    for (i, line) in lines.enumerate() {
        for (j, c) in line.chars().enumerate() {
            //lazily initialise if no other tile has already initialised the tile ij
            grid.entry((i, j))
                .and_modify(|(tile, _)| {
                    tile.get_or_insert(Tile::new(TileKind::from(c)));
                })
                .or_insert((Some(Tile::new(TileKind::from(c))), 0));

            //if current tyle is roll we update the count for each neighbour of ij
            if TileKind::from(c) == TileKind::ROLL {
                for (n, m) in calculate_neighbours((i as isize, j as isize))
                    .iter()
                    .filter(|(h, k)| *h >= 0 && *k >= 0)
                {
                    grid.entry((*n as usize, *m as usize))
                        .and_modify(|(_, count)| {
                            *count += 1;
                        })
                        .or_insert((None, 1));
                }
            }
        }
    }

    grid
}

pub fn calculate_neighbours((x, y): (isize, isize)) -> [(isize, isize); 8] {
    [
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        /*(x , y),*/ (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ]
}
