use crate::files::*;
use crate::pos::Pos;
use ncurses::*;
use std::{
    os::unix::fs::MetadataExt,
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

    pub fn update_dir<P: AsRef<Path>>(&mut self, p: P) -> std::io::Result<()> {
        let path = p.as_ref().to_owned();

        self.dir = SiuDir::new(&path)?;

        self.path = path;
        Ok(())
    }

    pub fn display(&self) {
        wclear(self.win);
        for (i, v) in self.dir.dirs.iter().enumerate() {
            //             let s = format!("{} {}", v.name, f.is_file);
            if i == self.idx.x {
                if v.is_file {
                    let f = format!("{} B", v.meta.clone().unwrap().size());
                    wattron(self.win, COLOR_PAIR(4));
                    mvwprintw(self.win, i as i32, 2, &v.name);
                    mvwprintw(self.win, i as i32, self.dim.x - f.len() as i32, &f);
                    wattroff(self.win, COLOR_PAIR(4));
                } else {
                    let f = format!("{}", v.meta.clone().unwrap().size());
                    wattron(self.win, COLOR_PAIR(2));
                    mvwprintw(self.win, i as i32, 2, &v.name);
                    mvwprintw(self.win, i as i32, self.dim.x - f.len() as i32, &f);
                    wattroff(self.win, COLOR_PAIR(2));
                }
            } else if (i as i32) > self.dim.y - 3 {
                break;
            } else {
                if v.is_file {
                    let f = format!("{} B", v.meta.clone().unwrap().size());
                    wattron(self.win, COLOR_PAIR(1));
                    mvwprintw(self.win, i as i32, 2, &v.name);
                    mvwprintw(self.win, i as i32, self.dim.x - f.len() as i32, &f);
                    wattroff(self.win, COLOR_PAIR(1));
                } else {
                    let f = format!("{}", v.meta.clone().unwrap().size());
                    wattron(self.win, COLOR_PAIR(3));
                    mvwprintw(self.win, i as i32, 2, &v.name);
                    mvwprintw(self.win, i as i32, self.dim.x - f.len() as i32, &f);
                    wattron(self.win, COLOR_PAIR(3));
                }
            }
        }
        wrefresh(self.win);
    }

    pub fn display_right(&self, is_file: bool) {
        match is_file {
            true => {
                let sanitized: String = self
                    .dir
                    .content
                    .clone()
                    .unwrap()
                    .chars()
                    .filter(|&c| c != '\0')
                    .collect();
                mvwprintw(self.win, 0, 2, &sanitized);
            }
            false => {
                if !self.dir.dirs.is_empty() {
                    for (i, v) in self.dir.dirs.iter().enumerate() {
                        //             let s = format!("{} {}", v.name, f.is_file);
                        if i == self.idx.x {
                            if v.is_file {
                                wattron(self.win, COLOR_PAIR(4));
                                mvwprintw(self.win, i as i32, 2, &v.name);
                                wattroff(self.win, COLOR_PAIR(4));
                            } else {
                                wattron(self.win, COLOR_PAIR(2));
                                mvwprintw(self.win, i as i32, 2, &v.name);
                                wattroff(self.win, COLOR_PAIR(2));
                            }
                        } else if (i as i32) > self.dim.y - 3 {
                            break;
                        } else {
                            if v.is_file {
                                wattron(self.win, COLOR_PAIR(1));
                                mvwprintw(self.win, i as i32, 2, &v.name);
                                wattroff(self.win, COLOR_PAIR(1));
                            } else {
                                wattron(self.win, COLOR_PAIR(3));
                                mvwprintw(self.win, i as i32, 2, &v.name);
                                wattron(self.win, COLOR_PAIR(3));
                            }
                        }
                    }
                } else {
                    wattron(self.win, COLOR_PAIR(5));
                    mvwprintw(self.win, 0, 0, "Dir empty");
                    wattroff(self.win, COLOR_PAIR(5));
                }
            }
        }
        wrefresh(self.win);
    }

    pub fn change_dim(&mut self, coord: Pos<i32>, dim: Pos<i32>) {
        self.dim = dim;
        self.coord = coord;
        self.win = newwin(dim.y, dim.x, coord.y, coord.x);
    }
}
