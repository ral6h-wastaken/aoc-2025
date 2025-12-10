use std::{
    collections::HashSet,
    str::FromStr,
};

const SOURCE: char = 'S';

pub enum ManifoldCell {
    EMPTY,
    SPLITTER,
}

impl ManifoldCell {
    pub fn compute_cells(line: String) -> Vec<Self> {
        line.trim()
            .chars()
            .map(|c| Self::try_from(c).expect("input promised :("))
            .collect()
    }
}

impl FromStr for ManifoldCell {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Self::EMPTY),
            "^" => Ok(Self::SPLITTER),
            _ => Err("Invalid manifold cell".into()),
        }
    }
}

impl TryFrom<char> for ManifoldCell {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Self::from_str(value.to_string().as_str())
    }
}

pub struct RayState {
    ///indices of the manifold which the ray is in at the current state
    state: HashSet<usize>,
}

impl RayState {
    pub fn init(first_line: String) -> Self {
        Self {
            state: HashSet::from([first_line
                .split_once(SOURCE)
                .expect("no source found in first line")
                .0
                .len()]), //.....S..... -> (....., .....)
        }
    }

    /// returns the new state as well as the number of splits that occurred
    pub fn advance(&self, manifold_row: Vec<ManifoldCell>) -> (u64, Self) {
        let mut splits = 0_u64;
        let mut new_state = HashSet::<usize>::new();

        for (i, cell) in manifold_row.iter().enumerate() {
            match (self.state.contains(&i), cell) {
                (true, ManifoldCell::EMPTY) => {
                    new_state.insert(i);
                }
                (true, ManifoldCell::SPLITTER) => {
                    splits += 1;
                    if i > 0 {
                        new_state.insert(i - 1);
                    }
                    if i < manifold_row.len() - 1 {
                        new_state.insert(i + 1);
                    }
                }
                (false, ManifoldCell::EMPTY) | (false, ManifoldCell::SPLITTER) => {}
            }
        }

        (splits, RayState { state: new_state })
    }
}
