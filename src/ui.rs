use crate::files::*;
use ncurses::*;
use std::fmt::Display;

pub struct SiuWin {
    pub win: WINDOW,
    pub idx: Pos<usize>,
    pub coord: Pos<i32>,
    pub dim: Pos<i32>,
    pub my_pos: Pos<i32>,
    pub path: String,
    pub dir: SiuDir,
}

impl SiuWin {
    pub fn new() -> Self {
        todo!()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Pos<T> {
    pub x: T,
    pub y: T,
}

impl<T> Pos<T>
where
    T: Display + Clone + Ord,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
