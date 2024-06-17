use crate::ui::*;

pub enum CurrentPane {
    Right,
    Middle,
    Left,
}

pub struct State {
    pub right_pane: SiuWin,
    pub middle_pane: SiuWin,
    pub left_pane: SiuWin,
    pub current_pane: CurrentPane,
}
