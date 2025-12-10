use crate::common::{ManifoldCell, RayState};

pub fn solution<T>(mut lines: T) -> u64
where
    T: Iterator<Item = String>,
{
    // .......S.......
    // .......|.......  7
    // .......^.......                               ar {}  spl {7}
    // ......|.|......  6 8
    // ......^.^......                               ar {}  spl {6, 8}
    // .....|.|.|.....  5 7 9
    // .....^.^.^.....                               ar {}  spl {5, 7, 9}
    // ....|.|.|.|....  4 6 8 10
    // ....^.^.|.^....                               ar {8} spl {4 6 10}
    // ...|.|.|||.|...  3 5 7 8 9 11
    // ...^.^.||^.^...                               ar {7 8} spl {3 5 9 11}
    // ..|.|.|||.|.|..  2 4 6 7 8 10 12
    // ..^.|.^||.|.^..                               ar {4 7 8 10} spl {2 6 12}
    // .|.|||.||.||.|.  1 3 4 5 7 8 10 11 13
    // .^.^|^.^|.||.^.                               ar {4 8 10 11} spl { 1 3 5 7 13}
    // |.|.|.|.|.|||.|  0 2 4 6 8 10 11 12 14

    let mut state = RayState::init(lines.next().expect("no first line found"));

    for line in lines.into_iter() {
        let new_state = state.advance(ManifoldCell::compute_cells(line));
        println!("got state {:?}", new_state);
        state = new_state;
    }

    state.compute_possibile_paths()

}
