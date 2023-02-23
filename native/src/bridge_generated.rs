#![allow(
    non_camel_case_types,
    unused,
    clippy::redundant_closure,
    clippy::useless_conversion,
    clippy::unit_arg,
    clippy::double_parens,
    non_snake_case,
    clippy::too_many_arguments
)]
// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.64.0.

use crate::api::*;
use core::panic::UnwindSafe;
use flutter_rust_bridge::*;
use std::ffi::c_void;
use std::sync::Arc;

// Section: imports

// Section: wire functions

fn wire_sudoku_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "sudoku",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| Ok(mirror_Sudoku(sudoku())),
    )
}
// Section: wrapper structs

#[derive(Clone)]
struct mirror_Candidates(Candidates);

#[derive(Clone)]
struct mirror_Cell(Cell);

#[derive(Clone)]
struct mirror_Coord(Coord);

#[derive(Clone)]
struct mirror_Digit(Digit);

#[derive(Clone)]
struct mirror_Sudoku(Sudoku);

// Section: static checks

const _: fn() = || {
    {
        let Candidates_ = None::<Candidates>.unwrap();
        let _: usize = Candidates_.0;
    }
    {
        let Cell = None::<Cell>.unwrap();
        let _: Coord = Cell.coord;
        let _: Option<Digit> = Cell.digit;
        let _: bool = Cell.is_given;
        let _: Candidates = Cell.candidates;
    }
    {
        let Coord_ = None::<Coord>.unwrap();
        let _: usize = Coord_.0;
        let _: usize = Coord_.1;
    }
    {
        let Digit_ = None::<Digit>.unwrap();
        let _: u8 = Digit_.0;
    }
    {
        let Sudoku_ = None::<Sudoku>.unwrap();
        let _: [Cell; 81] = Sudoku_.0;
    }
};
// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

pub trait Wire2Api<T> {
    fn wire2api(self) -> T;
}

impl<T, S> Wire2Api<Option<T>> for *mut S
where
    *mut S: Wire2Api<T>,
{
    fn wire2api(self) -> Option<T> {
        (!self.is_null()).then(|| self.wire2api())
    }
}
// Section: impl IntoDart

impl support::IntoDart for mirror_Candidates {
    fn into_dart(self) -> support::DartAbi {
        vec![self.0 .0.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for mirror_Candidates {}

impl support::IntoDart for mirror_Cell {
    fn into_dart(self) -> support::DartAbi {
        vec![
            mirror_Coord(self.0.coord).into_dart(),
            self.0.digit.map(|v| mirror_Digit(v)).into_dart(),
            self.0.is_given.into_dart(),
            mirror_Candidates(self.0.candidates).into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for mirror_Cell {}

impl support::IntoDart for mirror_Coord {
    fn into_dart(self) -> support::DartAbi {
        vec![self.0 .0.into_dart(), self.0 .1.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for mirror_Coord {}

impl support::IntoDart for mirror_Digit {
    fn into_dart(self) -> support::DartAbi {
        vec![self.0 .0.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for mirror_Digit {}

impl support::IntoDart for mirror_Sudoku {
    fn into_dart(self) -> support::DartAbi {
        vec![self.0 .0.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for mirror_Sudoku {}

// Section: executor

support::lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER: support::DefaultHandler = Default::default();
}

/// cbindgen:ignore
#[cfg(target_family = "wasm")]
#[path = "bridge_generated.web.rs"]
mod web;
#[cfg(target_family = "wasm")]
pub use web::*;

#[cfg(not(target_family = "wasm"))]
#[path = "bridge_generated.io.rs"]
mod io;
#[cfg(not(target_family = "wasm"))]
pub use io::*;
