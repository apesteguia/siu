pub mod files;
pub mod pos;
pub mod state;
pub mod ui;

use state::*;

fn main() {
    let s = State::new("/home/mikel/Escritorio/")
        .unwrap()
        .update()
        .unwrap()
        .exit();
}
