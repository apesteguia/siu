use crate::files::*;
use crate::pos::Pos;
use ncurses::*;
use std::{
    path::{Path, PathBuf},
    usize,
};

pub struct SiuWin {
    pub win: WINDOW,
    pub idx: Pos<usize>,
    pub coord: Pos<i32>,
    pub dim: Pos<i32>,
    pub my_pos: Pos<i32>,
    pub path: PathBuf,
    pub dir: SiuDir,
}

impl SiuWin {
    pub fn new<P: AsRef<Path>>(
        coord: Pos<i32>,
        dim: Pos<i32>,
        user_path: P,
    ) -> std::io::Result<Self> {
        let path = user_path.as_ref().to_owned();

        let idx = Pos::<usize>::new(0, 0);
        let dir = SiuDir::new(&path)?;
        let my_pos = Pos::new(0, 0);

        let win = newwin(dim.y, dim.x, coord.y, coord.x);

        Ok(Self {
            idx,
            my_pos,
            win,
            dir,
            coord,
            dim,
            path,
        })
    }

    pub fn display(&self) {
        for (i, v) in self.dir.dirs.iter().enumerate() {
            mvwprintw(self.win, i as i32 + 1, 2, &v.name);
        }
        box_(self.win, 0, 0);
        wrefresh(self.win);
    }

    pub fn change_dim(&mut self, coord: Pos<i32>, dim: Pos<i32>) {
        self.dim = dim;
        self.coord = coord;
        self.win = newwin(dim.y, dim.x, coord.y, coord.x);
    }
}
