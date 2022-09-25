use std::env;
use std::path::Path;

mod input;
mod rnd;
mod startrek;

mod prelude {
    pub use crate::input::*;
    pub use crate::rnd::*;
    pub use crate::startrek::*;
}

use crate::prelude::*;

fn main() {
    let trek_dir = env::var("TREK_DIR").unwrap();
    //println!("TREK_DIR: {}", trek_dir);

    let root = Path::new(&trek_dir);
    let _ = env::set_current_dir(&root).is_ok();

    intro();

    loop {
        let exit_flag = run_game();

        if exit_flag {
            break;
        }
    }
}
