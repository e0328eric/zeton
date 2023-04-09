use std::ffi::{self, CStr};

use crate::atom::*;
use crate::configuration::Config;
use crate::display::Display;
use crate::drw::Draw;
use crate::error;
use crate::util;

pub struct WindowManager<'dis> {
    dpy: &'dis Display,
    atoms: AtomContainer,
    screen: ffi::c_int,
    sw: ffi::c_int,
    sh: ffi::c_int,
    root: x11::Window,
    drw: Draw<'dis>,
    lrpad: ffi::c_uint,
    bh: ffi::c_uint,
}

macro_rules! cstr {
    ($bytes: literal) => {
        CStr::from_bytes_with_nul($bytes)
    };
}

impl<'dis> WindowManager<'dis> {
    pub fn new(dpy: &'dis Display, config: &Config) -> error::Result<Self> {
        // clean up any zombies immediately
        util::sigchld(0);

        // init screen
        let screen = unsafe { x11::XDefaultScreen(dpy.get_ptr()) };
        let sw = unsafe { x11::XDisplayWidth(dpy.get_ptr(), screen) };
        let sh = unsafe { x11::XDisplayHeight(dpy.get_ptr(), screen) };
        let root = unsafe { x11::XRootWindow(dpy.get_ptr(), screen) };
        let mut drw = unsafe { Draw::new(dpy, screen, root, sw as ffi::c_uint, sh as ffi::c_uint) };
        drw.fontset_create(&config.fonts)?;
        let lrpad = drw
            .get_font_height()
            .expect("FATAL_ERROR: lrpad cannot be None");
        let bh = lrpad + 2;

        // init atoms
        let mut atoms = AtomContainer::default();
        atoms.set_utf8string(dpy, cstr!(b"UTF8_STRING\0")?);

        atoms.insert_wmatom(dpy, WmAtomType::Protocols, cstr!(b"WM_PROTOCOLS\0")?);
        atoms.insert_wmatom(dpy, WmAtomType::Delete, cstr!(b"WM_DELETE_WINDOW\0")?);
        atoms.insert_wmatom(dpy, WmAtomType::State, cstr!(b"WM_STATE\0")?);
        atoms.insert_wmatom(dpy, WmAtomType::TakeFocus, cstr!(b"WM_TAKE_FOCUS\0")?);

        atoms.insert_netatom(
            dpy,
            NetAtomType::ActiveWindow,
            cstr!(b"_NET_ACTIVE_WINDOW\0")?,
        );
        atoms.insert_netatom(dpy, NetAtomType::Supported, cstr!(b"_NET_SUPPORTED\0")?);
        atoms.insert_netatom(
            dpy,
            NetAtomType::SystemTray,
            cstr!(b"_NET_SYSTEM_TRAY_S0\0")?,
        );

        Ok(Self {
            dpy,
            atoms,
            screen,
            sw,
            sh,
            root,
            drw,
            lrpad,
            bh,
        })
    }
}
