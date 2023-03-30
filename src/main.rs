mod x11;

use x11::display::XDisplay;

fn main() {
    let dpy = XDisplay::new();

    println!("Hello, world!");
}
