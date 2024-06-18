use crate::{pos::Pos, ui::*};
use ncurses::*;
use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

const W_RIGHT: f32 = 0.2;
const W_MIDDLE: f32 = 0.4;
const W_LEFT: f32 = 0.4;
const START_TOP: i32 = 1;

pub struct State {
    pub right_pane: SiuWin,
    pub middle_pane: SiuWin,
    pub left_pane: SiuWin,
    pub dim: Pos<i32>,
}

impl State {
    pub fn new<P: AsRef<Path>>(p: P) -> std::io::Result<Self> {
        let untested_path = p.as_ref().to_owned();
        let path = match untested_path.is_dir() {
            true => untested_path,
            false => current_dir()?,
        };

        initscr();
        noecho();
        raw();
        cbreak();
        refresh();

        let w = getmaxx(stdscr());
        let h = getmaxy(stdscr());

        let w_right = (w as f32 * W_RIGHT) as i32;
        let w_middle = (w as f32 * W_MIDDLE) as i32;
        let w_left = (w as f32 * W_LEFT) as i32;

        let parent = match path.parent() {
            Some(par) => par,
            None => panic!(),
        };

        //coord dim
        let left_pane = SiuWin::new(
            Pos::new(1, START_TOP),
            Pos::new(w_right, h - START_TOP),
            &parent,
        )?;
        let middle_pane = SiuWin::new(
            Pos::new(1 + w_right, START_TOP),
            Pos::new(w_middle, h - START_TOP),
            &path,
        )?;

        let child: PathBuf;
        if middle_pane.dir.dirs.is_empty() {
            child = path.clone();
        } else {
            child = middle_pane.dir.dirs.first().unwrap().path.clone();
        }

        let right_pane = SiuWin::new(
            Pos::new(1 + w_right + w_middle, START_TOP),
            Pos::new(w_left, h - START_TOP),
            &child,
        )?;

        Ok(Self {
            dim: Pos::new(w, h),
            right_pane,
            middle_pane,
            left_pane,
        })
    }

    fn display(&mut self) {
        self.resize();
        clear();
        refresh();
        mvwprintw(stdscr(), 0, 1, &self.right_pane.path.to_string_lossy());
        self.left_pane.display();
        self.middle_pane.display();
        self.right_pane.display_right(self.middle_pane.dir.dirs[self.middle_pane.idx.x].is_file);
    }

    pub fn update(&mut self) -> std::io::Result<&mut Self> {
        self.display();
        let mut ch = getch();
        while ch != 113 {
            match ch {
                //VIM movment keys
                //h
                104 => self.handle_movment_left(),
                //j
                106 => self.handle_movment_down()?,
                //k
                107 => self.handle_movment_up()?,
                //l
                108 => self.handle_movment_right(),
                _ => {}
            }

            self.display();
            ch = getch();
        }
        Ok(self)
    }

    fn handle_movment_down(&mut self) -> std::io::Result<()> {
        if self.middle_pane.idx.x < self.middle_pane.dir.dirs.len() - 1 {
            self.middle_pane.idx.x += 1;
            if self.middle_pane.dir.dirs[self.middle_pane.idx.x].is_file {
                self.right_pane.dir.read_dir(self.middle_pane.dir.dirs[self.middle_pane.idx.x].path.clone())?;
                self.right_pane.path = self.middle_pane.dir.dirs[self.middle_pane.idx.x].path.clone();
            } else {
                self.right_pane.update_dir(
                    self.middle_pane.dir.dirs[self.middle_pane.idx.x]
                        .path
                        .clone(),
                )?;
            }
        }
        Ok(())
    }
    fn handle_movment_up(&mut self) -> std::io::Result<()> {
        if self.middle_pane.idx.x > 0 {
            self.middle_pane.idx.x -= 1;
            if self.middle_pane.dir.dirs[self.middle_pane.idx.x].is_file {
                self.right_pane.dir.read_dir(self.middle_pane.dir.dirs[self.middle_pane.idx.x].path.clone())?;
            } else {
                self.right_pane.update_dir(
                    self.middle_pane.dir.dirs[self.middle_pane.idx.x]
                        .path
                        .clone(),
                )?;
            }
        }
        Ok(())
    }

    fn handle_movment_right(&mut self) {}
    fn handle_movment_left(&mut self) {}

    fn resize(&mut self) {
        let w = getmaxx(stdscr());
        let h = getmaxy(stdscr());

        if w != self.dim.x || h != self.dim.y {
            let w_right = (w as f32 * W_RIGHT) as i32;
            let w_middle = (w as f32 * W_MIDDLE) as i32;
            let w_left = (w as f32 * W_LEFT) as i32;

            self.right_pane
                .change_dim(Pos::new(1, START_TOP), Pos::new(w_right, h - START_TOP));
            self.middle_pane.change_dim(
                Pos::new(1 + w_right, START_TOP),
                Pos::new(w_middle, h - START_TOP),
            );
            self.left_pane.change_dim(
                Pos::new(1 + w_right + w_middle, START_TOP),
                Pos::new(w_left, h - START_TOP),
            );
            clear();
            refresh();
        }
    }

    pub fn exit(&mut self) {
        delwin(self.left_pane.win);
        delwin(self.middle_pane.win);
        delwin(self.right_pane.win);
        endwin();
    }
}
