#![allow(unused)]
#![allow(clippy::needless_return)]
#![allow(clippy::blocks_in_if_conditions)]

mod atom;
mod configuration;
mod display;
mod drw;
mod error;
mod util;
mod window_manager;
mod xerror;

use std::fs;
use std::io::prelude::*;

use crate::configuration::Config;
use crate::window_manager::WindowManager;

fn main() -> error::Result<()> {
    let mut config_file = fs::File::open("./config.toml")?;

    let mut config = String::with_capacity(100);
    config_file.read_to_string(&mut config)?;
    let config_toml = toml::from_str::<Config>(&config)?;

    let mut dpy = display::Display::new(None).expect("ERROR: cannot open display");
    dpy.check_other_wm();

    let _wm = WindowManager::new(&dpy, &config_toml)?;

    println!("Hello, world!");

    Ok(())
}
