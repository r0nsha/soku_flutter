// use flutter_rust_bridge::frb;
// pub use soku::prelude::*;

// pub fn sudoku() -> Sudoku {
//     Sudoku::new_unique(Config {
//         difficulty: Difficulty::Easy,
//     })
// }
//
// #[frb(mirror(Sudoku))]
// pub struct _Sudoku(pub [Cell; 81]);
//
// #[frb(mirror(Cell))]
// pub struct _Cell {
//     pub coord: Coord,
//     pub digit: Option<Digit>,
//     pub is_given: bool,
//     pub candidates: Candidates,
// }
//
// #[frb(mirror(Coord))]
// pub struct _Coord(pub usize, pub usize);
//
// #[frb(mirror(Digit))]
// pub struct _Digit(pub u8);
//
// #[frb(mirror(Candidates))]
// pub struct _Candidates(pub usize);
