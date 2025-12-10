use std::{
    collections::{HashMap, HashSet},
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

#[derive(Debug)]
pub struct RayState {
    ///indices of the manifold which the ray is in at the current state
    state: HashMap<usize, u64>,
}

impl RayState {
    pub fn compute_possibile_paths(&self) -> u64 {
        self.state.values().sum()
    }

    pub fn init(first_line: String) -> Self {
        Self {
            state: HashMap::from([(
                first_line
                    .split_once(SOURCE)
                    .expect("no source found in first line")
                    .0
                    .len(),
                1_u64,
            )]), //.....S..... -> (....., .....)
        }
    }

    pub fn get_indices(&self) -> HashSet<usize> {
        self.state.clone().into_keys().collect()
    }

    /// returns the new state, by also calculating the number of possible paths for each location
    /// in the state
    pub fn advance(&self, manifold_row: Vec<ManifoldCell>) -> Self {
        let mut new_state = HashMap::<usize, u64>::new();

        for (i, cell) in manifold_row.iter().enumerate() {
            match (self.state.contains_key(&i), cell) {
                (true, ManifoldCell::EMPTY) => {
                    let current_possible_paths = self
                                .state
                                .get(&i)
                                .expect("the if condition guarantees that key i is present");

                    new_state
                        .entry(i)
                        .and_modify(|numpaths| {
                            *numpaths += current_possible_paths
                        })
                        .or_insert(*current_possible_paths);
                }
                (true, ManifoldCell::SPLITTER) => {
                    let current_possible_paths = self
                                .state
                                .get(&i)
                                .expect("the if condition guarantees that key i is present");

                    if i > 0 {
                        new_state
                            .entry(i - 1)
                            .and_modify(|numpaths| {
                                *numpaths += current_possible_paths
                            })
                            .or_insert(*current_possible_paths);
                    }
                    if i < manifold_row.len() - 1 {
                        new_state
                            .entry(i + 1)
                            .and_modify(|numpaths| {
                                *numpaths += current_possible_paths
                            })
                            .or_insert(*current_possible_paths);
                    }
                }
                (false, ManifoldCell::EMPTY) | (false, ManifoldCell::SPLITTER) => {}
            }
        }

        Self { state: new_state }
    }
}
