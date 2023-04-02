mod display;
mod xerror;

fn main() {
    let mut dpy = display::XDisplay::new(None).expect("ERROR: cannot open display");
    dpy.check_other_wm();

    println!("Hello, world!");
}
