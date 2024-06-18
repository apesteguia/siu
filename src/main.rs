pub mod files;
pub mod pos;
pub mod state;
pub mod ui;

use state::*;

fn main() {
    let _ = State::new("/home/mikel/").unwrap().update().unwrap().exit();
}
