use crate::{pos::Pos, ui::*};
use ncurses::*;
use std::{env::current_dir, path::Path};

const W_RIGHT: f32 = 0.2;
const W_MIDDLE: f32 = 0.4;
const W_LEFT: f32 = 0.4;
const START_TOP: i32 = 0;

pub struct State {
    pub right_pane: SiuWin,
    pub middle_pane: SiuWin,
    pub left_pane: SiuWin,
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

        //coord dim
        let right_pane = SiuWin::new(Pos::new(1, START_TOP), Pos::new(w_right, h), &path)?;
        let middle_pane = SiuWin::new(
            Pos::new(1 + w_right, START_TOP),
            Pos::new(w_middle, h),
            &path,
        )?;
        let left_pane = SiuWin::new(
            Pos::new(1 + w_right + w_middle, START_TOP),
            Pos::new(w_left, h),
            &path,
        )?;

        Ok(Self {
            right_pane,
            middle_pane,
            left_pane,
        })
    }

    pub fn display(&self) {
        self.left_pane.display();
        self.middle_pane.display();
        self.right_pane.display();
    }

    pub fn update(&mut self) -> &mut Self {
        self.display();
        let mut ch = getch();
        while ch != 113 {
            self.display();
            ch = getch();
        }
        self
    }

    pub fn exit(&mut self) {
        delwin(self.left_pane.win);
        delwin(self.middle_pane.win);
        delwin(self.right_pane.win);
        endwin();
    }
}
