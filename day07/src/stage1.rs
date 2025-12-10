use crate::common::*;

pub fn solution<T>(mut lines: T) -> u64
where
    T: Iterator<Item = String>,
{
    let mut total_splits = 0_u64;
    let mut state = RayState::init(lines.next().expect("no first line found"));

    for line in lines.into_iter() {
        let (new_splits, new_state) = state.advance(ManifoldCell::compute_cells(line));
        total_splits += new_splits;
        state = new_state;
    }

    total_splits
}
