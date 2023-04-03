mod atom;
mod display;
mod drw;
mod util;
mod window_manager;
mod xerror;

use crate::window_manager::WindowManager;

fn main() {
    let mut dpy = display::XDisplay::new(None).expect("ERROR: cannot open display");
    dpy.check_other_wm();

    let _wm = WindowManager::setup(dpy);

    println!("Hello, world!");
}
